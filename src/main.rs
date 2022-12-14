use eframe::egui::{self, Ui};
use std::process::Command;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    x_monitor: String,
    y_monitor: String,
    height: String,
    width: String,
    x_position: String,
    y_position: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            x_monitor: 1920.to_string(),
            y_monitor: 1080.to_string(),
            height: 1080.to_string(),
            width: 1920.to_string(),
            x_position: 0.to_string(),
            y_position: 0.to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("wl-mirror-gui");
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                    if ui.add(egui::Button::new("All")).clicked() {
                        self.set_values(ui, MirrorConfig::Full);
                    }
                    if ui.add(egui::Button::new("Top-Left")).clicked() {
                        self.set_values(ui, MirrorConfig::TopLeft);
                    }
                    if ui.add(egui::Button::new("Top-Right")).clicked() {
                        self.set_values(ui, MirrorConfig::TopRight);
                    }
                    if ui.add(egui::Button::new("Bottom-Left")).clicked() {
                        self.set_values(ui, MirrorConfig::BottomLeft);
                    }
                    if ui.add(egui::Button::new("Bottom-Right")).clicked() {
                        self.set_values(ui, MirrorConfig::BottomRight);
                    }
                    if ui.add(egui::Button::new("Top")).clicked() {
                        self.set_values(ui, MirrorConfig::Top);
                    }
                    if ui.add(egui::Button::new("Bottom")).clicked() {
                        self.set_values(ui, MirrorConfig::Bottom);
                    }
                    if ui.add(egui::Button::new("Left")).clicked() {
                        self.set_values(ui, MirrorConfig::Left);
                    }
                    if ui.add(egui::Button::new("Right")).clicked() {
                        self.set_values(ui, MirrorConfig::Right);
                    }
                });
                ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                    text_and_button(&mut self.x_monitor, "Monitor Width:".to_string(), ui);
                    text_and_button(&mut self.y_monitor, "Monitor Height:".to_string(), ui);
                    text_and_button(&mut self.width, "Area Width:".to_string(), ui);
                    text_and_button(&mut self.height, "Area Height:".to_string(), ui);
                    text_and_button(&mut self.x_position, "Area X-position:".to_string(), ui);
                    text_and_button(&mut self.y_position, "Area Y-position:".to_string(), ui);
                    if ui.add(egui::Button::new("Create Window!")).clicked() {
                        if self.width.parse::<u32>().is_err()
                            || self.height.parse::<u32>().is_err()
                            || self.x_position.parse::<u32>().is_err()
                            || self.y_position.parse::<u32>().is_err()
                        {
                            ui.label("All your values need to be numbers.");
                        } else {
                            let s = Command::new("wl-mirror")
                                .arg("-r")
                                .arg(format!(
                                    "{},{} {}x{}",
                                    self.x_position, self.y_position, self.width, self.height,
                                ))
                                .output()
                                .expect("worked!");
                            ui.label(String::from_utf8_lossy(&s.stderr));
                        }
                    }
                })
            });
        });
    }
}

#[allow(clippy::ptr_arg)]
fn text_and_button(val: &mut String, label: String, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(egui::TextEdit::singleline(val));
    });
    if val.parse::<u32>().is_err() {
        ui.label("This value must be a number.");
    }
}

enum MirrorConfig {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
    Full,
}

impl MyApp {
    pub fn set_values(&mut self, ui: &mut Ui, config: MirrorConfig) {
        if let (Ok(x), Ok(y)) = (self.x_monitor.parse::<u32>(), self.y_monitor.parse::<u32>()) {
            match config {
                MirrorConfig::Full => {
                    self.x_position = 0.to_string();
                    self.y_position = 0.to_string();
                    self.width = self.x_monitor.clone();
                    self.height = self.y_monitor.clone();
                }
                MirrorConfig::TopLeft => {
                    self.x_position = 0.to_string();
                    self.y_position = 0.to_string();
                    self.width = (x / 2).to_string();
                    self.height = (y / 2).to_string();
                }
                MirrorConfig::TopRight => {
                    self.x_position = (x / 2).to_string();
                    self.y_position = 0.to_string();
                    self.width = (x / 2).to_string();
                    self.height = (y / 2).to_string();
                }
                MirrorConfig::BottomLeft => {
                    self.x_position = 0.to_string();
                    self.y_position = (y / 2).to_string();
                    self.width = (x / 2).to_string();
                    self.height = (y / 2).to_string();
                }
                MirrorConfig::BottomRight => {
                    self.x_position = (x / 2).to_string();
                    self.y_position = (y / 2).to_string();
                    self.width = (x / 2).to_string();
                    self.height = (y / 2).to_string();
                }
                MirrorConfig::Bottom => {
                    self.x_position = 0.to_string();
                    self.y_position = (y / 2).to_string();
                    self.width = self.x_monitor.clone();
                    self.height = (y / 2).to_string();
                }
                MirrorConfig::Top => {
                    self.x_position = 0.to_string();
                    self.y_position = 0.to_string();
                    self.width = self.x_monitor.clone();
                    self.height = (y / 2).to_string();
                }
                MirrorConfig::Left => {
                    self.x_position = 0.to_string();
                    self.y_position = 0.to_string();
                    self.width = (x / 2).to_string();
                    self.height = self.y_monitor.clone();
                }
                MirrorConfig::Right => {
                    self.x_position = (x / 2).to_string();
                    self.y_position = 0.to_string();
                    self.width = (x / 2).to_string();
                    self.height = self.y_monitor.clone();
                }
            }
        } else {
            ui.label("Your provided monitor size are not numbers.");
        }
    }
}
