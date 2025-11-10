use std::{cmp::min, process::exit, time::{Duration, Instant}};

use iced::{event, keyboard::Key, widget::{column, text, Column}, Alignment::Center, Event, Fill, Subscription};
// use rand::Rng;

use crate::{key_converter::{InputKey, IN_KEYS_COUNT, LEFT_KEYS, OUT_KEYS, RIGHT_KEYS}, remapper::Remapper};

pub struct App {
    remapper: Remapper,

    start: Instant,

    target: String,
    garbage_index: usize,
    hinted: bool,
    start_hint: u8
}

#[derive(Debug, Clone)]
pub enum Message {
    Press(Key),
    Release(Key)
}

impl App {
    pub fn new(params: [InputKey; IN_KEYS_COUNT], cutoff: Duration, target: String) -> Self {
        Self { remapper: Remapper::new(params, cutoff), start: Instant::now(), target, garbage_index: 0, hinted: false, start_hint: 2/*rand::rng().random_range(0..2)*/ }
    }

    pub fn view(&self) -> Column<Message> {
        if self.target.is_empty() {
            // Cringe
            exit(0)
        }

        let hint = if self.hinted {
            let char = self.target.chars().nth(self.garbage_index).unwrap();
            let key = self.remapper.params[OUT_KEYS.iter().position(|value| *value == char).unwrap()];
            text(format!("{}:{}", LEFT_KEYS[key.left], RIGHT_KEYS[key.right]))
        } else {
            let char = self.target.chars().nth(self.garbage_index).unwrap();
            let key = self.remapper.params[OUT_KEYS.iter().position(|value| *value == char).unwrap()];
            if self.start_hint == 0 {
                text(format!("{}:", LEFT_KEYS[key.left]))
            } else if self.start_hint == 1 {
                text(format!(":{}", RIGHT_KEYS[key.right]))
            } else {
                text(":")
            }
        };

        let mut pad = "".to_owned();
        for _i in 0..self.garbage_index {
            pad.push(' ');
        }

        column![
            text(self.target[0..min(16, self.target.len())].to_owned() + "\n" + &pad + "^").size(50),
            hint.size(50)
        ].width(Fill).align_x(Center)
    }

    pub fn update(&mut self, message: Message) {
        if let Message::Press(Key::Character(chars)) = message {
            let time = Instant::now() - self.start;

            if let Some(char) = chars.chars().next() {
                if char == '`' {
                    // Cringe
                    exit(0)
                }

                if char == '=' {
                    self.hinted = true;
                    return
                }

                if let Some(char) = self.remapper.push_key(char, time) {
                    if char == '←' {
                        if self.garbage_index > 0 {
                            self.garbage_index -= 1;
                            self.target.remove(0);
                        }
                    } else if self.garbage_index == 0 && self.target.starts_with(char) {
                        self.target.remove(0);
                        self.hinted = false;
                        /*self.start_hint += 1;
                        if self.start_hint == 3 {
                            self.start_hint = 0;
                        }*/
                    } else {
                        self.target.insert(0, char);

                        self.garbage_index += 1;
                        self.hinted = true;
                    }
                }
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _id| match event {
            Event::Keyboard(key_event) => match key_event {
                iced::keyboard::Event::KeyPressed { key, .. } => Some(Message::Press(key)),
                iced::keyboard::Event::KeyReleased { key, .. } => Some(Message::Release(key)),
                _ => None
            },
            _ => None
        })
    }
}
