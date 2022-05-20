pub mod packet_protocol;

use clipboard::{ClipboardContext, ClipboardProvider};
use eframe::egui::{
    Button, FontData, FontDefinitions, Key, Label, Layout, PointerButton, RichText,
    SelectableLabel, Sense, TopBottomPanel,
};
use eframe::epaint::FontFamily;
use eframe::{egui, CreationContext};
use egui::{Color32, Context, Pos2};
use serde::{Deserialize, Serialize};
use std::io::{self};
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};

use std::thread;

use crate::APP_NAME;

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
const RED: Color32 = Color32::from_rgb(255, 0, 0);

pub enum Msg {
    ApiKeySet(String),
    Refresh,
}

#[derive(Serialize, Deserialize)]
pub struct BepInExGUIConfig {
    pub dark_mode: bool,
    pub window_pos: Pos2,
}

impl Default for BepInExGUIConfig {
    fn default() -> Self {
        Self {
            dark_mode: true,
            window_pos: Default::default(),
        }
    }
}

pub struct BepInExGUI {
    pub config: BepInExGUIConfig,
    pub logs_receiver: Option<Receiver<String>>,
    pub logs: Vec<String>,
    pub button_currently_down: bool,
    pub first_index_of_log_that_is_selected: u32,
    pub smallest_index_of_hovered_log: u32,
    pub biggest_index_of_hovered_log: u32,
}

impl BepInExGUI {
    pub fn new() -> BepInExGUI {
        BepInExGUI {
            config: Default::default(),
            logs_receiver: None,
            logs: vec![],
            button_currently_down: false,
            first_index_of_log_that_is_selected: std::u32::MAX,
            smallest_index_of_hovered_log: std::u32::MAX,
            biggest_index_of_hovered_log: std::u32::MAX,
        }
    }

    pub fn init(mut self, cc: &CreationContext) -> Self {
        if let Some(storage) = cc.storage {
            self.config = eframe::get_value(storage, APP_NAME).unwrap_or_default();
        }
        self.configure_fonts(&cc.egui_ctx);

        self.init_log_receiver_thread();

        self
    }

    fn init_log_receiver_thread(&mut self) {
        let (logs_sender, logs_receiver) = channel();
        self.logs_receiver = Some(logs_receiver);

        log_receiver_thread_loop(logs_sender);
    }

    pub fn configure_fonts(&self, ctx: &Context) {
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

    pub fn render_logs(&mut self, ui: &mut eframe::egui::Ui) {
        let clip_rect = ui.painter().clip_rect();
        ui.interact(clip_rect, ui.id(), Sense::drag());
        let is_button_down = ui.ctx().input().pointer.primary_down();
        let is_button_up = !ui.ctx().input().pointer.button_down(PointerButton::Primary);

        if is_button_up {
            self.button_currently_down = false;
        }

        let mut i = 0;
        for log in &self.logs {
            let selectable_response = ui.add(SelectableLabel::new(
                i >= self.smallest_index_of_hovered_log && i <= self.biggest_index_of_hovered_log,
                log,
            ));

            let mut log_rect = selectable_response.rect;
            log_rect.max.x = clip_rect.max.x;
            if ui.rect_contains_pointer(log_rect) && is_button_down {
                if !self.button_currently_down {
                    self.button_currently_down = true;
                    self.first_index_of_log_that_is_selected = i;
                    self.smallest_index_of_hovered_log = i;
                    self.biggest_index_of_hovered_log = i;
                }

                if i < self.first_index_of_log_that_is_selected {
                    self.smallest_index_of_hovered_log = i;
                }

                if i > self.first_index_of_log_that_is_selected {
                    self.biggest_index_of_hovered_log = i;
                }
            }

            i += 1;
        }

        if ui.ctx().input().modifiers.command && ui.ctx().input().key_pressed(Key::C) {
            match ClipboardProvider::new() {
                Ok(ctx_) => {
                    let mut ctx: ClipboardContext = ctx_;
                    let (start_index, end_index) = if self.first_index_of_log_that_is_selected
                        < self.biggest_index_of_hovered_log
                    {
                        (
                            self.first_index_of_log_that_is_selected as usize,
                            self.biggest_index_of_hovered_log as usize,
                        )
                    } else {
                        (
                            self.smallest_index_of_hovered_log as usize,
                            self.first_index_of_log_that_is_selected as usize,
                        )
                    };
                    let selected_logs = &mut self.logs[start_index..end_index + 1];
                    let selected_logs_string = selected_logs.join("\n");
                    match ctx.set_contents(selected_logs_string) {
                        Ok(_) => {}
                        Err(err) => {
                            eprintln!("Failed copying to clipboard logs {}", err);
                        }
                    }
                }
                Err(_) => {}
            }
        }

        /*for a in &self.articles {
            ui.add_space(PADDING);
            // render title
            let title = format!("▶ {}", a.title);
            if self.config.dark_mode {
                ui.colored_label(WHITE, title);
            } else {
                ui.colored_label(BLACK, title);
            }
            // render desc
            ui.add_space(PADDING);
            let desc =
                Label::new(RichText::new(&a.desc).text_style(eframe::egui::TextStyle::Button));
            ui.add(desc);

            // render hyperlinks
            if self.config.dark_mode {
                ui.style_mut().visuals.hyperlink_color = CYAN;
            } else {
                ui.style_mut().visuals.hyperlink_color = RED;
            }
            ui.add_space(PADDING);
            ui.with_layout(
                Layout::right_to_left().with_cross_align(eframe::emath::Align::Min),
                |ui| {
                    ui.add(Hyperlink::from_label_and_url("read more ⤴", &a.url));
                },
            );
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }*/
    }

    pub fn update_receive_logs_from_channel(&mut self) {
        if let Some(receiver) = &self.logs_receiver {
            match receiver.try_recv() {
                Ok(log) => {
                    self.logs.push(log);
                }
                Err(_) => {}
            }
        }
    }

    pub(crate) fn render_top_panel(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                // logo
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Label::new(
                        RichText::new("📓").text_style(egui::TextStyle::Heading),
                    ));
                });
                // controls
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let close_btn = ui.add(Button::new(
                        RichText::new("❌").text_style(egui::TextStyle::Body),
                    ));
                    if close_btn.clicked() {
                        frame.quit();
                    }
                    let theme_btn = ui.add(Button::new(
                        RichText::new({
                            if self.config.dark_mode {
                                "🌞"
                            } else {
                                "🌙"
                            }
                        })
                        .text_style(egui::TextStyle::Body),
                    ));
                    if theme_btn.clicked() {
                        self.config.dark_mode = !self.config.dark_mode;
                    }
                });
            });
            ui.add_space(10.);
        });
    }
}

fn log_receiver_thread_loop(logs_sender: Sender<String>) {
    thread::spawn(move || -> io::Result<()> {
        loop {
            match TcpStream::connect("127.0.0.1:27090") {
                Ok(mut tcp_stream) => loop {
                    match packet_protocol::read_packet_length(&mut tcp_stream) {
                        Ok(packet_length) => {
                            match packet_protocol::read_packet(&mut tcp_stream, packet_length) {
                                Ok(packet_bytes) => {
                                    send_utf8_string_packet_to_channel(&logs_sender, &packet_bytes);
                                }
                                Err(err) => {
                                    eprintln!(
                                        "Error reading packet {}\n Disconnecting socket",
                                        err
                                    );
                                    break;
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("Error reading packet length {}\n Disconnecting socket", err);
                            break;
                        }
                    }
                },
                Err(err) => eprintln!("Failed connecting {}", err),
            }
        }
    });
}

fn send_utf8_string_packet_to_channel(
    channel_sender: &std::sync::mpsc::Sender<String>,
    packet_bytes: &Vec<u8>,
) {
    if let Err(err) =
        channel_sender.send(packet_protocol::packet_bytes_to_utf8_string(&packet_bytes))
    {
        eprintln!("error while sending utf8 string to channel : {}", err);
    }
}
