use eframe::{egui::*, CreationContext};
use sysinfo::{Pid, SystemExt};
use tab::settings_tab::SettingsTab;

use tab::console_tab::ConsoleTab;

use tab::general_tab::GeneralTab;

use eframe::*;

use eframe;
use winapi::shared::minwindef::{BOOL, DWORD, LPARAM};
use winapi::shared::windef::HWND;
use winapi::um::processthreadsapi::GetCurrentProcessId;
use winapi::um::winuser::{EnumWindows, GetWindowThreadProcessId, HWND_NOTOPMOST};
use zip::write::FileOptions;

use core::time;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::SystemTime;
use std::{cell::RefCell, sync::mpsc::Receiver};
use std::{fs, io, thread};

use std::rc::Rc;

use tab::Tab;

use crate::bepinex_mod::BepInExMod;
use crate::log_receiver_thread::LogReceiverThread;
use crate::{bepinex_gui_config::BepInExGUIConfig, bepinex_log::BepInExLog, tab};
use crate::{egui_utils, path_utils, settings, thunderstore_communities};

pub struct BepInExGUI {
    config: BepInExGUIConfig,
    target_name: String,
    game_folder_full_path: PathBuf,
    bepinex_log_output_file_full_path: PathBuf,
    bepinex_gui_csharp_cfg_full_path: PathBuf,
    target_process_id: Pid,
    time_when_disclaimer_showed_up: Option<SystemTime>,
    should_exit_app: Arc<AtomicBool>,
    tabs: Vec<Box<dyn Tab>>,
    mods: Rc<RefCell<Option<Vec<BepInExMod>>>>,
    logs: Rc<RefCell<Option<Vec<BepInExLog>>>>,
    logs_receiver: Option<Receiver<BepInExLog>>,
    log_receiver_thread: Option<LogReceiverThread>,
    log_socket_port_receiver: u16,
}

impl App for BepInExGUI {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        if self.should_exit_app.load(Ordering::Relaxed) {
            frame.quit();
        }
        ctx.request_repaint();

        #[cfg(debug_assertions)]
        ctx.set_debug_on_hover(true);

        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        self.update_receive_logs_from_channel();

        if self.config.first_time {
            self.show_first_time_disclaimer(ctx);
        } else {
            self.render_header(ctx, frame);

            let tab = &mut self.tabs[self.config.selected_tab_index];

            tab.update(&mut self.config, ctx, frame);
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, settings::APP_NAME, &self.config);
        _ = self.config.save_csharp_cfg_file();
    }
}

impl BepInExGUI {
    pub fn new(
        target_name: String,
        game_folder_full_path: PathBuf,
        bepinex_log_output_file_full_path: PathBuf,
        bepinex_gui_csharp_cfg_full_path: PathBuf,
        target_process_id: Pid,
        log_socket_port_receiver: u16,
    ) -> Self {
        Self {
            config: Default::default(),
            target_name,
            game_folder_full_path,
            bepinex_log_output_file_full_path,
            bepinex_gui_csharp_cfg_full_path,
            target_process_id: target_process_id,
            time_when_disclaimer_showed_up: None,
            should_exit_app: Arc::new(AtomicBool::new(false)),
            tabs: vec![],
            mods: Rc::new(RefCell::new(Some(vec![BepInExMod {
                name: "".to_string(),
                version: "".to_string(),
            }]))),
            logs: Rc::new(RefCell::new(Some(vec![]))),
            logs_receiver: None,
            log_receiver_thread: None,
            log_socket_port_receiver: log_socket_port_receiver,
        }
    }

    pub fn init(mut self, cc: &CreationContext) -> Self {
        if let Some(storage) = cc.storage {
            self.config = eframe::get_value(storage, settings::APP_NAME).unwrap_or_default();
        }
        self.configure_fonts(&cc.egui_ctx);

        self.start_thread_exit_gui_if_target_process_not_alive(self.target_process_id);

        self.start_thread_window_foreground_on_target_start(self.target_process_id);

        self.init_log_receiver(self.log_socket_port_receiver);

        self.init_tabs();

        self.config.bepinex_gui_csharp_cfg_full_path =
            self.bepinex_gui_csharp_cfg_full_path.clone();

        _ = self.config.read_csharp_cfg_file();

        self
    }

    fn init_log_receiver(&mut self, log_socket_port_receiver: u16) {
        let (logs_sender, logs_receiver) = channel();
        self.logs_receiver = Some(logs_receiver);

        let log_receiver = LogReceiverThread::new(log_socket_port_receiver, logs_sender.clone());
        log_receiver.start_thread_loop();
        self.log_receiver_thread = Some(log_receiver);
    }

    fn init_tabs(&mut self) {
        self.tabs.push(Box::new(GeneralTab::new(
            self.mods.clone(),
            self.logs.clone(),
            self.target_process_id,
            self.target_name.clone(),
            self.game_folder_full_path.clone(),
            self.bepinex_log_output_file_full_path.clone(),
        )));
        self.tabs.push(Box::new(ConsoleTab::new(
            self.mods.clone(),
            self.logs.clone(),
            self.target_process_id,
            self.game_folder_full_path.clone(),
            self.bepinex_log_output_file_full_path.clone(),
        )));
        self.tabs.push(Box::new(SettingsTab::new()));
    }

    fn update_receive_logs_from_channel(&mut self) {
        let mut logs_borrow = self.logs.borrow_mut();
        let logs = logs_borrow.as_mut().unwrap();

        let mut mods_borrow = self.mods.borrow_mut();
        let mods = mods_borrow.as_mut().unwrap();

        if let Some(receiver) = &self.logs_receiver {
            match receiver.try_recv() {
                Ok(log) => {
                    if log.data.contains("Loading [") {
                        let split: Vec<&str> = log.data.split('[').collect();
                        let mod_info_text = split[2];
                        let mod_version_start_index_ = mod_info_text.rfind(' ');
                        if let Some(mod_version_start_index) = mod_version_start_index_ {
                            let mod_name = &mod_info_text[0..mod_version_start_index];
                            let mod_version = &mod_info_text
                                [mod_version_start_index + 1..mod_info_text.len() - 1];
                            mods.push(BepInExMod {
                                name: mod_name.to_string(),
                                version: mod_version.to_string(),
                            });
                        }
                    }

                    logs.push(log);
                }
                Err(_) => {}
            }
        }
    }

    fn show_first_time_disclaimer(&mut self, ctx: &Context) {
        Window::new("Disclaimer").min_width(ctx.available_rect().size().x).anchor(Align2::CENTER_CENTER, Vec2::ZERO).show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.add(
            Label::new(RichText::new(
    r#"The console is now disabled by default.
If you notice issues with a mod while playing:
- Head to the Modding Discord by clicking on the button below.
- Post the log file by copying it to clipboard through the button below.
- Wait for help.
For mod developers that like the old conhost console, you can enable it back by opening the BepInEx/config/BepInEx.cfg and 
setting to true the "Enables showing a console for log output." config option."#).font(FontId::proportional(20.0))).wrap(true));

            static mut FIRST_TIME_SHOW:bool = true;
            unsafe {
                if FIRST_TIME_SHOW {
                    self.time_when_disclaimer_showed_up = Some(SystemTime::now());
                    FIRST_TIME_SHOW = false;
                }
            }

            if let Ok(_elapsed) = self.time_when_disclaimer_showed_up.unwrap().elapsed() {
                let elapsed = _elapsed.as_secs() as i64;
                if 9 - elapsed >= 0 {
                    ui.label(RichText::new((10 - elapsed).to_string()).font(FontId::proportional(20.0)));
                }
                else {
                    if ui.button(RichText::new("Ok").font(FontId::proportional(20.0))).clicked() {
                        self.config.first_time = false;
                    }
                }
            }
        });
    });
    }

    fn configure_fonts(&self, ctx: &Context) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MesloLGS".to_string(),
            FontData::from_static(include_bytes!("../../assets/fonts/MesloLGS_NF_Regular.ttf")),
        );

        font_def
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_string());

        ctx.set_fonts(font_def);
    }

    fn start_thread_exit_gui_if_target_process_not_alive(&self, target_process_id: Pid) {
        let mut sys = sysinfo::System::new_all();

        let close_window_when_game_closes = self.config.close_window_when_game_closes.clone();
        let should_exit_app = self.should_exit_app.clone();
        thread::spawn(move || -> io::Result<()> {
            loop {
                if !sys.refresh_process(target_process_id)
                    && close_window_when_game_closes.load(Ordering::Relaxed)
                {
                    break;
                }

                thread::sleep(time::Duration::from_millis(2000));
            }

            tracing::info!("Target process is not alive, scheduling exit");
            should_exit_app.store(true, Ordering::Relaxed);

            Ok(())
        });
    }

    #[cfg(windows)]
    fn start_thread_window_foreground_on_target_start(&self, target_process_id: Pid) {
        thread::spawn(move || -> io::Result<()> {
            loop {
                if check_current_process_in_front_of_target_process_window(target_process_id) {
                    break;
                }

                thread::sleep(time::Duration::from_millis(500));
            }

            thread::spawn(|| -> io::Result<()> {
                thread::sleep(time::Duration::from_millis(500));

                set_topmost_current_process_window(false);

                Ok(())
            });

            Ok(())
        });
    }

    #[cfg(not(windows))]
    fn start_thread_window_foreground_on_target_start(&self, target_process_id: Pid) {}

    fn render_header(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut button_size = ui.available_size() / 3.;
                button_size.y += 25.;

                ui.spacing_mut().item_spacing.x = 1.;
                ui.spacing_mut().item_spacing.y = 1.;

                let mut i = 0;
                for tab in &self.tabs {
                    if ui
                        .add_sized(
                            button_size,
                            Button::new(RichText::new(tab.name()).font(FontId::proportional(20.0))),
                        )
                        .clicked()
                    {
                        self.config.selected_tab_index = i;
                    }

                    i += 1;
                }
            });

            ui.add_space(10.);

            if !self.config.first_time_console_disclaimer {
                self.tabs[self.config.selected_tab_index].update_top_panel(&mut self.config, ui);
                ui.add_space(10.);
            }
        });
    }
}

fn set_topmost_current_process_window(set_topmost: bool) {
    use winapi::um::winuser::{SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE};

    unsafe {
        static mut CURRENT_PROCESS_ID: u32 = 0;
        CURRENT_PROCESS_ID = GetCurrentProcessId() as u32;

        static mut SET_TOPMOST: bool = false;
        SET_TOPMOST = set_topmost;

        extern "system" fn enum_window(window: HWND, _: LPARAM) -> BOOL {
            unsafe {
                let mut proc_id: DWORD = 0 as DWORD;
                let _ = GetWindowThreadProcessId(window, &mut proc_id as *mut DWORD);
                if proc_id == CURRENT_PROCESS_ID {
                    SetWindowPos(
                        window,
                        if SET_TOPMOST {
                            HWND_TOPMOST
                        } else {
                            HWND_NOTOPMOST
                        },
                        0,
                        0,
                        0,
                        0,
                        SWP_NOMOVE | SWP_NOSIZE,
                    );
                }

                true.into()
            }
        }

        EnumWindows(Some(enum_window), 0 as LPARAM);
    }
}

#[cfg(windows)]
fn check_current_process_in_front_of_target_process_window(target_process_id_: Pid) -> bool {
    unsafe {
        static mut CURRENT_PROCESS_ID: u32 = 0;
        CURRENT_PROCESS_ID = GetCurrentProcessId() as u32;

        static mut TARGET_PROCESS_ID: u32 = 0;
        TARGET_PROCESS_ID = std::mem::transmute_copy(&target_process_id_);

        static mut GOT_CURRENT_PROC_WINDOW: bool = false;
        GOT_CURRENT_PROC_WINDOW = false;
        static mut GOT_TARGET_PROC_WINDOW: bool = false;
        GOT_TARGET_PROC_WINDOW = false;
        static mut GOT_RESULT: bool = false;

        GOT_RESULT = false;
        extern "system" fn enum_window(window: HWND, _: LPARAM) -> BOOL {
            unsafe {
                if GOT_RESULT {
                    return true.into();
                }

                let mut proc_id: DWORD = 0 as DWORD;
                let _ = GetWindowThreadProcessId(window, &mut proc_id as *mut DWORD);
                if proc_id == TARGET_PROCESS_ID {
                    let is_current_proc_in_front = GOT_CURRENT_PROC_WINDOW;
                    if is_current_proc_in_front {
                        GOT_RESULT = true;
                    } else {
                        GOT_TARGET_PROC_WINDOW = true;
                    }
                } else if proc_id == CURRENT_PROCESS_ID {
                    let is_target_proc_in_front = GOT_TARGET_PROC_WINDOW;
                    if is_target_proc_in_front {
                        set_topmost_current_process_window(true);
                        tracing::info!("Put bep gui window in front");
                        GOT_RESULT = true;
                    } else {
                        GOT_CURRENT_PROC_WINDOW = true;
                    }
                }

                true.into()
            }
        }

        EnumWindows(Some(enum_window), 0 as LPARAM);

        return GOT_CURRENT_PROC_WINDOW && GOT_RESULT;
    }
}
#[cfg(not(windows))]
fn is_current_process_in_front_of_target_process_window(&self, target_process_id: Pid) {}

pub fn render_theme_button(gui_config: &mut BepInExGUIConfig, ui: &mut egui::Ui) {
    let theme_btn_text = if gui_config.dark_mode { "🌞" } else { "🌙" };
    let theme_btn_size = egui_utils::compute_text_size(ui, theme_btn_text, true, false, None);
    ui.add_space(ui.available_width() - theme_btn_size.x);
    let theme_btn_resp = ui.add(Button::new(
        RichText::new(theme_btn_text).text_style(egui::TextStyle::Heading),
    ));
    if theme_btn_resp.clicked() {
        gui_config.dark_mode ^= true;
    }
}

pub fn render_useful_buttons_footer(
    ui: &mut Ui,
    _ctx: &Context,
    game_folder_full_path: &PathBuf,
    bepinex_log_output_file_full_path: &PathBuf,
    target_process_id: Pid,
) {
    ui.horizontal_centered(|ui| {
        const FONT_SIZE: f32 = 18.;
        // let mut FONT_SIZE = 20. * (ui.available_width() / 900.);

        let mut button_size = ui.available_size() / 5.;
        let spacing = ui.available_width() / 8.;
        button_size.y += 25.;

        let placement_cursor = ui.cursor();
        ui.add_space(spacing * 0.5);

        if ui
            .add_sized(
                button_size,
                Button::new(
                    RichText::new("Open Game Folder").font(FontId::proportional(FONT_SIZE)),
                ),
            )
            .clicked()
        {
            path_utils::open_path_in_explorer(game_folder_full_path);
        }

        ui.set_cursor(placement_cursor);
        ui.add_space(spacing * 3.);

        if ui
            .add_sized(
                button_size,
                Button::new(RichText::new("Copy Log File").font(FontId::proportional(FONT_SIZE))),
            )
            .clicked()
        {
            // check log file size, if its more than size limit, just zip it
            if let Ok(log_file_metadata) = fs::metadata(&bepinex_log_output_file_full_path) {
                let file_size_bytes = log_file_metadata.len();
                const ONE_MEGABYTE: u64 = 1000000;
                if file_size_bytes >= ONE_MEGABYTE {
                    let zip_file_full_path = bepinex_log_output_file_full_path
                        .parent()
                        .unwrap()
                        .join("zipped_log.zip");
                    match zip_log_file(&zip_file_full_path, &bepinex_log_output_file_full_path) {
                        Ok(_) => {
                            path_utils::highlight_path_in_explorer(&zip_file_full_path);
                        }
                        Err(e) => {
                            tracing::error!("Failed zipping: {}", e.to_string());
                        }
                    }
                } else {
                    path_utils::highlight_path_in_explorer(bepinex_log_output_file_full_path);
                }
            }
        }

        ui.set_cursor(placement_cursor);
        ui.add_space(spacing * 5.5);

        if ui
            .add_sized(
                button_size,
                Button::new(RichText::new("Modding Discord").font(FontId::proportional(FONT_SIZE))),
            )
            .clicked()
        {
            thunderstore_communities::open_modding_discord(target_process_id);
        }
    });
    ui.add_space(25.);
}

fn zip_log_file<P: AsRef<Path>, P2: AsRef<Path>>(
    zip_file_path: P,
    log_file_path: P2,
) -> zip::result::ZipResult<()> {
    let zip_file = std::fs::File::create(&zip_file_path).unwrap();

    let mut zip = zip::ZipWriter::new(zip_file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    zip.start_file("LogOutput.log", options)?;
    let mut log_file_buffer = BufReader::new(File::open(log_file_path)?);
    let mut zip_buf_writer = BufWriter::new(zip);
    std::io::copy(&mut log_file_buffer, &mut zip_buf_writer)?;

    // zip.write_all()?;

    // zip.finish()?;
    Ok(())
}
