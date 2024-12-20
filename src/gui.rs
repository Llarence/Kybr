use std::{cmp::min, collections::VecDeque, process::exit, time::{Duration, Instant}};

use iced::{event, keyboard::Key, widget::{column, text, Column}, Alignment::Center, Event, Fill, Subscription};
use rand::Rng;

use crate::keyboard::{InputKey, IN_KEYS_COUNT, LEFT_KEYS, OUT_KEYS, RIGHT_KEYS};

pub struct App {
    params: [InputKey; IN_KEYS_COUNT],
    cutoff: Duration,

    // This is a rare case where a linkedlist could be faster
    //  but cursor and retain are expiremental
    left_keys: VecDeque<(usize, Instant)>,
    right_keys: VecDeque<(usize, Instant)>,

    target: String,
    garbage_index: usize,
    hinted: bool,
    left: bool
}

#[derive(Debug, Clone)]
pub enum Message {
    Press(Key),
    Release(Key)
}

impl App {
    pub fn new(params: [InputKey; IN_KEYS_COUNT], cutoff: Duration, target: String) -> Self {
        Self { params, cutoff, left_keys: VecDeque::new(), right_keys: VecDeque::new(), target, garbage_index: 0, hinted: false, left: rand::thread_rng().gen_bool(0.5) }
    }

    pub fn view(&self) -> Column<Message> {
        if self.target.is_empty() {
            // Cringe
            exit(0)
        }

        let hint = if self.hinted {
            let char = self.target.chars().nth(self.garbage_index).unwrap();
            let key = self.params[OUT_KEYS.iter().position(|value| *value == char).unwrap()];
            text(format!("{}:{}", LEFT_KEYS[key.left], RIGHT_KEYS[key.right]))
        } else {
            let char = self.target.chars().nth(self.garbage_index).unwrap();
            let key = self.params[OUT_KEYS.iter().position(|value| *value == char).unwrap()];
            text(format!(":{}", if self.left { LEFT_KEYS[key.left] } else { RIGHT_KEYS[key.right] }))
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

    fn refresh_keys(&mut self, time: Instant) {
        let cutoff = time - self.cutoff;

        self.left_keys.retain(|value | value.1 >= cutoff);
        self.right_keys.retain(|value | value.1 >= cutoff);

        while let (Some(left), Some(right)) = (self.left_keys.front(), self.right_keys.front()) {
            let res = self.params.iter().position(|value| value.compare(left.0, right.0));

            if let Some(index) = res {
                if OUT_KEYS[index] == '←' {
                    if self.garbage_index > 0 {
                        self.garbage_index -= 1;
                        self.target.remove(0);
                    }
                } else if self.garbage_index == 0 && self.target.starts_with(OUT_KEYS[index]) {
                    self.target.remove(0);
                    self.hinted = false;
                    self.left = !self.left;
                } else {
                    self.target.insert(0, OUT_KEYS[index]);

                    self.garbage_index += 1;
                    self.hinted = true;
                }
            }

            self.left_keys.pop_front();
            self.right_keys.pop_front();
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Press(key) => {
                if let Key::Character(chars) = key {
                    let time = Instant::now();

                    if let Some(char) = chars.chars().next() {
                        if char == '`' {
                            // Cringe
                            exit(0)
                        }

                        if char == '=' {
                            self.hinted = true;
                            return
                        }

                        if let Some(index) = LEFT_KEYS.iter().position(|curr| *curr == char) {
                            self.left_keys.push_back((index, time));
                            self.refresh_keys(time);
                        } else if let Some(index) = RIGHT_KEYS.iter().position(|curr| *curr == char) {
                            self.right_keys.push_back((index, time));
                            self.refresh_keys(time);
                        }
                    }
                }
            }

            Message::Release(key) => {
                if let Key::Character(chars) = key {
                    if let Some(char) = chars.chars().next() {
                        let cutoff = Instant::now() - self.cutoff;

                        if let Some(index) = LEFT_KEYS.iter().position(|value| *value == char) {
                            self.left_keys.retain(|value | value.0 != index && value.1 >= cutoff);
                        } else if let Some(index) = RIGHT_KEYS.iter().position(|value| *value == char) {
                            self.right_keys.retain(|value | value.0 != index && value.1 >= cutoff);
                        }
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