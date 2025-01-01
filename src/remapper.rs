use std::{collections::VecDeque, time::Duration};

use crate::key_converter::{InputKey, IN_KEYS_COUNT, LEFT_KEYS, OUT_KEYS, RIGHT_KEYS};

pub struct Remapper {
    pub params: [InputKey; IN_KEYS_COUNT],
    cutoff: Duration,

    // This is a case where a linkedlist could be faster
    //  but cursor and retain are expiremental
    left_keys: VecDeque<(usize, Duration)>,
    right_keys: VecDeque<(usize, Duration)>
}

impl Remapper {
    pub fn new(params: [InputKey; IN_KEYS_COUNT], cutoff: Duration) -> Self {
        Remapper { params, cutoff, left_keys: VecDeque::new(), right_keys: VecDeque::new() }
    }

    pub fn push_key(&mut self, key: char, time: Duration) -> Option<char> {
        if let Some(index) = LEFT_KEYS.iter().position(|curr| *curr == key) {
            self.left_keys.push_back((index, time));
            self.right_keys.retain(|value| value.1 >= time - self.cutoff);
        } else if let Some(index) = RIGHT_KEYS.iter().position(|curr| *curr == key) {
            self.right_keys.push_back((index, time));
            self.left_keys.retain(|value| value.1 >= time - self.cutoff);
        }

        if let (Some(left), Some(right)) = (self.left_keys.front(), self.right_keys.front()) {
            let res = self.params.iter().position(|value| value.compare(left.0, right.0));

            self.left_keys.pop_front();
            self.right_keys.pop_front();

            Some(OUT_KEYS[res?])
        } else {
            None
        }
    }
}
