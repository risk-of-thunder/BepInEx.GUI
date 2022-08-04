use eframe::{egui::*, CreationContext};
use sysinfo::{Pid, SystemExt};
use tab::settings_tab::SettingsTab;

use tab::console_tab::ConsoleTab;

use tab::general_tab::GeneralTab;

use eframe::*;

use eframe;
use winapi::shared::minwindef::BOOL;
use winapi::shared::windef::{HWND, POINT};
use winapi::um::winuser::{
    CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData, CF_HDROP,
};
use zip::write::FileOptions;

use core::time;
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufReader, Write};
use std::mem::size_of;
use std::os::windows::prelude::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr::copy_nonoverlapping;
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
use crate::{egui_utils, settings, thunderstore_communities};

pub struct BepInExGUI {
    config: BepInExGUIConfig,
    target_name: String,
    game_folder_full_path: PathBuf,
    bepinex_root_full_path: PathBuf,
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
        bepinex_root_full_path: PathBuf,
        bepinex_log_output_file_full_path: PathBuf,
        bepinex_gui_csharp_cfg_full_path: PathBuf,
        target_process_id: Pid,
        log_socket_port_receiver: u16,
    ) -> Self {
        Self {
            config: Default::default(),
            target_name,
            game_folder_full_path,
            bepinex_root_full_path,
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
            self.bepinex_root_full_path.clone(),
            self.bepinex_log_output_file_full_path.clone(),
        )));
        self.tabs.push(Box::new(ConsoleTab::new(
            self.mods.clone(),
            self.logs.clone(),
            self.target_process_id,
            self.game_folder_full_path.clone(),
            self.bepinex_root_full_path.clone(),
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
    bepinex_root_full_path: &PathBuf,
    bepinex_log_output_file_full_path: &PathBuf,
    target_process_id: Pid,
) {
    ui.horizontal_centered(|ui| {
        const FONT_SIZE: f32 = 14.;
        // let mut FONT_SIZE = 20. * (ui.available_width() / 900.);

        let mut button_size = ui.available_size() / 16.;
        let spacing = ui.available_width() / 16.;
        button_size.y += 25.;

        let placement_cursor = ui.cursor();
        ui.add_space(spacing * 0.9);

        if ui
            .add_sized(
                button_size,
                Button::new(
                    RichText::new("Open Game Folder").font(FontId::proportional(FONT_SIZE)),
                ),
            )
            .clicked()
        {
            egui_utils::open_folder(game_folder_full_path);
        }

        ui.set_cursor(placement_cursor);
        ui.add_space(spacing * 5.);

        if ui
            .add_sized(
                button_size,
                Button::new(RichText::new("Open Log Folder").font(FontId::proportional(FONT_SIZE))),
            )
            .clicked()
        {
            egui_utils::open_folder(bepinex_root_full_path);
        }

        ui.set_cursor(placement_cursor);
        ui.add_space(spacing * 8.5);

        if ui
            .add_sized(
                button_size,
                Button::new(
                    RichText::new("Copy Log to Clipboard").font(FontId::proportional(FONT_SIZE)),
                ),
            )
            .clicked()
        {
            // check log file size, if its more than size limit, just zip it
            if let Ok(log_file_metadata) = fs::metadata(&bepinex_log_output_file_full_path) {
                let file_size_bytes = log_file_metadata.len();
                const FIVE_MEGABYTES: u64 = 5000000;
                if file_size_bytes >= FIVE_MEGABYTES {
                    if let Some(zip_file_path) = settings::get_tmp_zip_log_full_path() {
                        match zip_log_file(&zip_file_path, &bepinex_log_output_file_full_path) {
                            Ok(_) => {
                                // file is zipped, clipboard it
                                copy_files_to_clipboard(vec![zip_file_path.into_os_string()])
                            }
                            Err(e) => {
                                tracing::error!("Failed zipping: {}", e.to_string());
                            }
                        }
                    }
                } else {
                    copy_files_to_clipboard(vec![bepinex_log_output_file_full_path
                        .clone()
                        .into_os_string()])
                }
            }
        }

        ui.set_cursor(placement_cursor);
        ui.add_space(spacing * 13.);

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

    let log_file = File::open(log_file_path)?;
    zip.start_file("LogOutput.log", options)?;
    zip.write_all(BufReader::new(log_file).buffer())?;

    zip.finish()?;
    Ok(())
}

#[repr(C, packed(1))]
pub struct DROPFILES {
    pub p_files: u32,
    pub pt: POINT,
    pub f_nc: BOOL,
    pub f_wide: BOOL,
}

#[cfg(windows)]
fn copy_files_to_clipboard(entries: Vec<OsString>) {
    let mut clip_buf: Vec<u16> = vec![];
    for entry in &entries {
        let mut result: Vec<u16> = entry.encode_wide().collect();
        clip_buf.append(&mut result);
        clip_buf.push(0);
    }
    clip_buf.push(0);
    let p_files = size_of::<DROPFILES>();
    let mut h_global = vec![0u8; clip_buf.len() * 2 + p_files];
    let dropfiles: *mut DROPFILES = h_global.as_mut_ptr() as *mut DROPFILES;
    let buf_ptr = clip_buf.as_ptr();
    unsafe {
        (*dropfiles).p_files = p_files as _;
        (*dropfiles).f_wide = 1 as BOOL;
        copy_nonoverlapping(
            buf_ptr,
            h_global.as_mut_ptr().offset(p_files as _) as *mut u16,
            clip_buf.len(),
        );
        let h_mem = core::mem::transmute(h_global.as_mut_ptr());
        OpenClipboard(0 as HWND);
        EmptyClipboard();
        CloseClipboard();

        OpenClipboard(0 as HWND);
        SetClipboardData(CF_HDROP, h_mem);
        CloseClipboard();
    }
}

#[cfg(not(windows))]
fn copy_files_to_clipboard(entries: Vec<OsString>) {
    // todo
}
