use iced::widget::{button, column, Column};
use std::{
    io::Error,
    process::{Command, Output},
};

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
}

struct WlMirrorGui {
    x_monitor: u32,
    y_monitor: u32,
    height: u32,
    width: u32,
    x_position: u32,
    y_position: u32,
}

#[derive(Clone)]
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
}

impl WlMirrorGui {
    pub fn view(&mut self) -> Column<Message> {
        column![
            button("Top-left").on_press(Message::TopLeft),
            button("Top-right").on_press(Message::TopRight),
            button("Bottom-left").on_press(Message::BottomLeft),
            button("Bottom-right").on_press(Message::BottomRight),
            button("Top").on_press(Message::Top),
            button("Bottom").on_press(Message::Bottom),
            button("Left").on_press(Message::Left),
            button("Right").on_press(Message::Right),
            button("Full").on_press(Message::Full),
        ]
    }
}

impl WlMirrorGui {
    pub fn run_command(self) -> Result<Output, Error> {
        Command::new("wl-mirror")
            .arg("-r")
            .arg(format!(
                "{},{} {}x{}",
                self.x_position, self.y_position, self.width, self.height,
            ))
            .output()
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
        }
    }
}
