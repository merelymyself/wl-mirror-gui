use iced::{widget::column, widget::*, Element, Length, Sandbox, Settings};
use std::process::Command;

fn main() -> iced::Result {
    WlMirrorGui::run(Settings::default())
}

struct WlMirrorGui {
    x_monitor: u32,
    y_monitor: u32,
    height: u32,
    width: u32,
    x_position: u32,
    y_position: u32,
}

#[derive(Clone, Debug)]
enum Message {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
    Full,
    XMonitorChanged(String),
    YMonitorChanged(String),
    HeightChanged(String),
    WidthChanged(String),
    XPositionChanged(String),
    YPositionChanged(String),
    RunCommand,
}

impl Sandbox for WlMirrorGui {
    type Message = Message;

    fn new() -> Self {
        Self {
            x_monitor: 1920,
            y_monitor: 1080,
            height: 1080,
            width: 1920,
            x_position: 0,
            y_position: 0,
        }
    }

    fn title(&self) -> String {
        String::from("wl-mirror-gui")
    }

    fn view(&self) -> Element<Message> {
        let buttom_column: Element<Message> = column![
            button("Top-left").on_press(Message::TopLeft),
            vertical_space(Length::Units(3)),
            button("Top-right").on_press(Message::TopRight),
            vertical_space(Length::Units(3)),
            button("Bottom-left").on_press(Message::BottomLeft),
            vertical_space(Length::Units(3)),
            button("Bottom-right").on_press(Message::BottomRight),
            vertical_space(Length::Units(3)),
            button("Top").on_press(Message::Top),
            vertical_space(Length::Units(3)),
            button("Bottom").on_press(Message::Bottom),
            vertical_space(Length::Units(3)),
            button("Left").on_press(Message::Left),
            vertical_space(Length::Units(3)),
            button("Right").on_press(Message::Right),
            vertical_space(Length::Units(3)),
            button("Full").on_press(Message::Full),
        ]
        .padding(20)
        .into();

        let number_column: Element<Message> = column![
            text("Screen Dimensions: ").size(26),
            row![
                text_input(
                    "Screen Width",
                    &self.x_monitor.to_string(),
                    Message::XMonitorChanged,
                )
                .width(iced::Length::Units(50)),
                text("x").size(26),
                text_input(
                    "Screen Height",
                    &self.y_monitor.to_string(),
                    Message::YMonitorChanged,
                )
                .width(iced::Length::Units(50)),
            ],
            text("Mirror Dimensions: ").size(26),
            row![
                text_input(
                    "Mirror Width",
                    &self.width.to_string(),
                    Message::WidthChanged,
                )
                .width(iced::Length::Units(50)),
                text("x").size(26),
                text_input(
                    "Mirror Height",
                    &self.height.to_string(),
                    Message::HeightChanged,
                )
                .width(iced::Length::Units(50)),
            ],
            text("Mirror Location: ").size(26),
            row![
                text("x:").size(26),
                text_input(
                    "Mirror x-position",
                    &self.x_position.to_string(),
                    Message::XPositionChanged,
                )
                .width(iced::Length::Units(50)),
                text(" y:").size(26),
                text_input(
                    "Mirror y-position",
                    &self.y_position.to_string(),
                    Message::YPositionChanged,
                )
                .width(iced::Length::Units(50)),
            ],
            vertical_space(Length::Units(3)),
            button("Create Mirror Window!").on_press(Message::RunCommand),
        ]
        .into();

        Container::new(
            Row::new()
                .spacing(10)
                .push(buttom_column)
                .push(number_column),
        )
        .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::RunCommand => { 
                self.run_command();
            },
            _ => {
                self.set_values(message);
            }
        }
    }
}

impl WlMirrorGui {
    pub fn run_command(&self) {
        let s = Command::new("wl-mirror")
            .arg("-r")
            .arg(format!(
                "{},{} {}x{}",
                self.x_position, self.y_position, self.width, self.height,
            ))
            .output();
        println!("{}", String::from_utf8_lossy(&s.expect("panic").stderr));
    }
    pub fn set_values(&mut self, config: Message) {
        match config {
            Message::Full => {
                self.x_position = 0;
                self.y_position = 0;
                self.width = self.x_monitor;
                self.height = self.y_monitor;
            }
            Message::TopLeft => {
                self.x_position = 0;
                self.y_position = 0;
                self.width = self.x_monitor / 2;
                self.height = self.y_monitor / 2;
            }
            Message::TopRight => {
                self.x_position = self.x_monitor / 2;
                self.y_position = 0;
                self.width = self.x_monitor / 2;
                self.height = self.y_monitor / 2;
            }
            Message::BottomLeft => {
                self.x_position = 0;
                self.y_position = self.y_monitor / 2;
                self.width = self.x_monitor / 2;
                self.height = self.y_monitor / 2;
            }
            Message::BottomRight => {
                self.x_position = self.x_monitor / 2;
                self.y_position = self.y_monitor / 2;
                self.width = self.x_monitor / 2;
                self.height = self.y_monitor / 2;
            }
            Message::Bottom => {
                self.x_position = 0;
                self.y_position = self.y_monitor / 2;
                self.width = self.x_monitor;
                self.height = self.y_monitor / 2;
            }
            Message::Top => {
                self.x_position = 0;
                self.y_position = 0;
                self.width = self.x_monitor;
                self.height = self.y_monitor / 2;
            }
            Message::Left => {
                self.x_position = 0;
                self.y_position = 0;
                self.width = self.x_monitor / 2;
                self.height = self.y_monitor;
            }
            Message::Right => {
                self.x_position = self.x_monitor / 2;
                self.y_position = 0;
                self.width = self.x_monitor / 2;
                self.height = self.y_monitor;
            }
            Message::XMonitorChanged(s) => {
                if let Ok(x) = s.parse::<u32>() {
                    self.x_monitor = x;
                } else if s.is_empty() {
                    self.x_monitor = 0;
                }
            }
            Message::YMonitorChanged(s) => {
                if let Ok(x) = s.parse::<u32>() {
                    self.y_monitor = x;
                } else if s.is_empty() {
                    self.y_monitor = 0;
                }
            }
            Message::WidthChanged(s) => {
                if let Ok(x) = s.parse::<u32>() {
                    self.width = x;
                } else if s.is_empty() {
                    self.width = 0;
                }
            }
            Message::HeightChanged(s) => {
                if let Ok(x) = s.parse::<u32>() {
                    self.height = x;
                } else if s.is_empty() {
                    self.height = 0;
                }
            }
            Message::XPositionChanged(s) => {
                if let Ok(x) = s.parse::<u32>() {
                    self.x_position = x;
                } else if s.is_empty() {
                    self.x_position = 0;
                }
            }
            Message::YPositionChanged(s) => {
                if let Ok(x) = s.parse::<u32>() {
                    self.y_position = x;
                } else if s.is_empty() {
                    self.y_position = 0;
                }
            }
            _ => {}
        }
    }
}
