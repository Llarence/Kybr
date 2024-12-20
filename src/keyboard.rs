use std::fmt;

use include_data::include_data;

// Maybe some of this should be defined in the Problem struct

// Each hand has four fingers with three keys for each and one extra for the index
const LEFT_KEYS_COUNT: usize = 4 * 3 + 1;
// Each hand has four fingers with three keys for each and two extra for the index
const RIGHT_KEYS_COUNT: usize = 4 * 3 + 2;

pub const IN_KEYS_COUNT: usize = LEFT_KEYS_COUNT * RIGHT_KEYS_COUNT;
pub const OUT_KEYS_COUNT: usize = 98;

pub const LEFT_KEYS: [char; LEFT_KEYS_COUNT] =
    ['q', 'a', 'z', 'w', 's', 'x', 'e', 'd', 'c', 'r', 'f', 'v', 'g'];
const LEFT_COST: [f64; LEFT_KEYS_COUNT] =
    [2.3, 1.3, 2.5, 1.4, 1.2, 3.5, 1.3, 1.1, 2.5, 1.4, 1.0, 1.7, 1.5];
const LEFT_MASK: [u32; LEFT_KEYS_COUNT] =
    [ 0,   0,   0,   1,   1,   1,   2,   2,   2,   3,   3,   3,   3 ];

pub const RIGHT_KEYS: [char; RIGHT_KEYS_COUNT] =
    ['/', ';', '.', 'p', 'l', ',', 'o', 'k', 'm', 'i', 'j', 'n', 'u', 'h'];
const RIGHT_COST: [f64; RIGHT_KEYS_COUNT] =
    [3.8, 1.3, 3.5, 2.5, 1.2, 3.0, 1.4, 1.1, 1.5, 1.3, 1.0, 2.0, 2.5, 1.5];
const RIGHT_MASK: [u32; RIGHT_KEYS_COUNT] =
    [ 0,   0,   1,   0,   1,   2,   1,   2,   3,   2,   3,   3,   3,   3 ];

// Maybe there is a better way
pub const IN_KEYS: [InputKey; IN_KEYS_COUNT] = const {
    let mut ret: [InputKey; IN_KEYS_COUNT] = [InputKey::new(0, 0); IN_KEYS_COUNT];

    let mut value = 0;
    while value < IN_KEYS_COUNT {
        ret[value] = InputKey::new(value / RIGHT_KEYS_COUNT, value % RIGHT_KEYS_COUNT);
        value += 1;
    }

    ret
};

// \n -> ↲ \t -> → DEL -> ←
pub const OUT_KEYS: [char; OUT_KEYS_COUNT] =
    ['↲', '→', ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', '←'];
pub const OUT_KEY_PAIR_PROBS: [f64; OUT_KEYS_COUNT * OUT_KEYS_COUNT] = include_data!("../data/code.data");

pub fn index_pair(prev: usize, curr: usize) -> usize {
    (prev * OUT_KEYS_COUNT) + curr
}

#[derive(Copy, Clone)]
pub struct InputKey {
    pub left: usize,
    pub right: usize,

    cost: f64,
    left_mask: u32,
    right_mask: u32
}

impl InputKey {
    pub const fn new(left: usize, right: usize) -> Self {
        Self { left, right, cost: LEFT_COST[left] + RIGHT_COST[right], left_mask: LEFT_MASK[left], right_mask: RIGHT_MASK[right] }
    }

    // LEFT_KEYS_COUNT and RIGHT_KEYS_COUNT < BYTE
    pub fn from_bytes(bytes: [u8; 2]) -> Self {
        Self::new(bytes[0] as usize, bytes[1] as usize)
    }

    pub fn compare(&self, left: usize, right: usize) -> bool {
        self.left == left && self.right == right
    }

    pub fn get_cost(&self, prev: &Self) -> f64 {
        // Maybe it should use the distance the finger has to move?
        if (self.left_mask == prev.left_mask && self.left != prev.left) || (self.right_mask == prev.right_mask && self.right != prev.right) { self.cost * 2.0 } else { self.cost }
    }

    // LEFT_KEYS_COUNT and RIGHT_KEYS_COUNT < BYTE
    pub fn as_bytes(&self) -> [u8; 2] {
        [self.left as u8, self.right as u8]
    }
}

impl fmt::Display for InputKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InputKey")
            .field("left", &LEFT_KEYS[self.left])
            .field("right", &RIGHT_KEYS[self.right])
            .finish()
    }
}
