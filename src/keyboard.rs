use std::{fs::File, io::{Error, Read, Write}, slice::from_raw_parts, time::Duration};

use phf::phf_map;

use crate::{input::{input_event, EV_KEY}, keyboard};
use crate::uhid::{uhid_event, uhid_event__bindgen_ty_1, uhid_event_type_UHID_CREATE2, uhid_event_type_UHID_DESTROY, uhid_event_type_UHID_INPUT2, BUS_USB};

// Maybe make the key rollover higher
// Volume doesn't work

const NAME: [u8; 5] = [b'T', b'e', b's', b't', b'\0'];
const DESC: [u8; 63] = [
    0x05, 0x01,
    0x09, 0x06,
    0xA1, 0x01,
    0x05, 0x07,
    0x19, 0xE0,
    0x29, 0xE7,
    0x15, 0x00,
    0x25, 0x01,
    0x75, 0x01,
    0x95, 0x08,
    0x81, 0x02,
    0x95, 0x01,
    0x75, 0x08,
    0x81, 0x01,
    0x95, 0x05,
    0x75, 0x01,
    0x05, 0x08,
    0x19, 0x01,
    0x29, 0x05,
    0x91, 0x02,
    0x95, 0x01,
    0x75, 0x03,
    0x91, 0x01,
    0x95, 0x06,
    0x75, 0x08,
    0x15, 0x00,
    0x25, 0x65,
    0x05, 0x07,
    0x19, 0x00,
    0x29, 0x65,
    0x81, 0x00,
    0xC0
];

const CTRL: u8 = 0b0000_0001;
const SHIFT: u8 = 0b0000_0010;
const ALT: u8 = 0b0000_0100;

// TODO: Merge the two following hashmaps
// Should really be CODE_TO_KEYCODE, but I am lazy

pub const CHAR_TO_KEYCODE: phf::Map<char, u8> = phf_map! {
    'a' => 4,
    'b' => 5,
    'c' => 6,
    'd' => 7,
    'e' => 8,
    'f' => 9,
    'g' => 10,
    'h' => 11,
    'i' => 12,
    'j' => 13,
    'k' => 14,
    'l' => 15,
    'm' => 16,
    'n' => 17,
    'o' => 18,
    'p' => 19,
    'q' => 20,
    'r' => 21,
    's' => 22,
    't' => 23,
    'u' => 24,
    'v' => 25,
    'w' => 26,
    'x' => 27,
    'y' => 28,
    'z' => 29,
    '1' => 30,
    '2' => 31,
    '3' => 32,
    '4' => 33,
    '5' => 34,
    '6' => 35,
    '7' => 36,
    '8' => 37,
    '9' => 38,
    '0' => 39,
    '↲' => 40,
    '\x1B' => 41,
    '←' => 42,
    '→' => 43,
    ' ' => 44,
    '-' => 45,
    '=' => 46,
    '[' => 47,
    ']' => 48,
    '\\' => 49,
    ';' => 51,
    '\'' => 52,
    '`' => 53,
    ',' => 54,
    '.' => 55,
    '/' => 56,
    '⇨' => 79,
    '⇦' => 80,
    '⇩' => 81,
    '⇧' => 82,
    '🔊' => 128,
    '🔇' => 129,
    '\x07' => 224,
    '\x0E' => 225,
    '↹' => 226
};

const CHAR_TO_KEYPRESS: phf::Map<char, keyboard::KeyPress> = phf_map! {
    'a' => KeyPress::new(4, &[]),
    'A' => KeyPress::new(4, &[SHIFT]),
    'b' => KeyPress::new(5, &[]),
    'B' => KeyPress::new(5, &[SHIFT]),
    'c' => KeyPress::new(6, &[]),
    'C' => KeyPress::new(6, &[SHIFT]),
    'd' => KeyPress::new(7, &[]),
    'D' => KeyPress::new(7, &[SHIFT]),
    'e' => KeyPress::new(8, &[]),
    'E' => KeyPress::new(8, &[SHIFT]),
    'f' => KeyPress::new(9, &[]),
    'F' => KeyPress::new(9, &[SHIFT]),
    'g' => KeyPress::new(10, &[]),
    'G' => KeyPress::new(10, &[SHIFT]),
    'h' => KeyPress::new(11, &[]),
    'H' => KeyPress::new(11, &[SHIFT]),
    'i' => KeyPress::new(12, &[]),
    'I' => KeyPress::new(12, &[SHIFT]),
    'j' => KeyPress::new(13, &[]),
    'J' => KeyPress::new(13, &[SHIFT]),
    'k' => KeyPress::new(14, &[]),
    'K' => KeyPress::new(14, &[SHIFT]),
    'l' => KeyPress::new(15, &[]),
    'L' => KeyPress::new(15, &[SHIFT]),
    'm' => KeyPress::new(16, &[]),
    'M' => KeyPress::new(16, &[SHIFT]),
    'n' => KeyPress::new(17, &[]),
    'N' => KeyPress::new(17, &[SHIFT]),
    'o' => KeyPress::new(18, &[]),
    'O' => KeyPress::new(18, &[SHIFT]),
    'p' => KeyPress::new(19, &[]),
    'P' => KeyPress::new(19, &[SHIFT]),
    'q' => KeyPress::new(20, &[]),
    'Q' => KeyPress::new(20, &[SHIFT]),
    'r' => KeyPress::new(21, &[]),
    'R' => KeyPress::new(21, &[SHIFT]),
    's' => KeyPress::new(22, &[]),
    'S' => KeyPress::new(22, &[SHIFT]),
    't' => KeyPress::new(23, &[]),
    'T' => KeyPress::new(23, &[SHIFT]),
    'u' => KeyPress::new(24, &[]),
    'U' => KeyPress::new(24, &[SHIFT]),
    'v' => KeyPress::new(25, &[]),
    'V' => KeyPress::new(25, &[SHIFT]),
    'w' => KeyPress::new(26, &[]),
    'W' => KeyPress::new(26, &[SHIFT]),
    'x' => KeyPress::new(27, &[]),
    'X' => KeyPress::new(27, &[SHIFT]),
    'y' => KeyPress::new(28, &[]),
    'Y' => KeyPress::new(28, &[SHIFT]),
    'z' => KeyPress::new(29, &[]),
    'Z' => KeyPress::new(29, &[SHIFT]),
    '1' => KeyPress::new(30, &[]),
    '!' => KeyPress::new(30, &[SHIFT]),
    '2' => KeyPress::new(31, &[]),
    '@' => KeyPress::new(31, &[SHIFT]),
    '3' => KeyPress::new(32, &[]),
    '#' => KeyPress::new(32, &[SHIFT]),
    '4' => KeyPress::new(33, &[]),
    '$' => KeyPress::new(33, &[SHIFT]),
    '5' => KeyPress::new(34, &[]),
    '%' => KeyPress::new(34, &[SHIFT]),
    '6' => KeyPress::new(35, &[]),
    '^' => KeyPress::new(35, &[SHIFT]),
    '7' => KeyPress::new(36, &[]),
    '&' => KeyPress::new(36, &[SHIFT]),
    '8' => KeyPress::new(37, &[]),
    '*' => KeyPress::new(37, &[SHIFT]),
    '9' => KeyPress::new(38, &[]),
    '(' => KeyPress::new(38, &[SHIFT]),
    '0' => KeyPress::new(39, &[]),
    ')' => KeyPress::new(39, &[SHIFT]),
    '↲' => KeyPress::new(40, &[]),
    '←' => KeyPress::new(42, &[]),
    '→' => KeyPress::new(43, &[]),
    ' ' => KeyPress::new(44, &[]),
    '-' => KeyPress::new(45, &[]),
    '_' => KeyPress::new(45, &[SHIFT]),
    '=' => KeyPress::new(46, &[]),
    '+' => KeyPress::new(46, &[SHIFT]),
    '[' => KeyPress::new(47, &[]),
    '{' => KeyPress::new(47, &[SHIFT]),
    ']' => KeyPress::new(48, &[]),
    '}' => KeyPress::new(48, &[SHIFT]),
    '\\' => KeyPress::new(49, &[]),
    '|' => KeyPress::new(49, &[SHIFT]),
    ';' => KeyPress::new(51, &[]),
    ':' => KeyPress::new(51, &[SHIFT]),
    '\'' => KeyPress::new(52, &[]),
    '"' => KeyPress::new(52, &[SHIFT]),
    '`' => KeyPress::new(53, &[]),
    '~' => KeyPress::new(53, &[SHIFT]),
    ',' => KeyPress::new(54, &[]),
    '<' => KeyPress::new(54, &[SHIFT]),
    '.' => KeyPress::new(55, &[]),
    '>' => KeyPress::new(55, &[SHIFT]),
    '/' => KeyPress::new(56, &[]),
    '?' => KeyPress::new(56, &[SHIFT])
};

// These are from the input.rs file, but phf needs them as u16
// Why are \x07 ctrl and \x0E shift, also ↹ is alt
const CODE_TO_CHAR: phf::Map<u16, char> = phf_map! {
    1u16 => '\x1B',
    2u16 => '1',
    3u16 => '2',
    4u16 => '3',
    5u16 => '4',
    6u16 => '5',
    7u16 => '6',
    8u16 => '7',
    9u16 => '8',
    10u16 => '9',
    11u16 => '0',
    12u16 => '-',
    13u16 => '=',
    14u16 => '←',
    15u16 => '→',
    16u16 => 'q',
    17u16 => 'w',
    18u16 => 'e',
    19u16 => 'r',
    20u16 => 't',
    21u16 => 'y',
    22u16 => 'u',
    23u16 => 'i',
    24u16 => 'o',
    25u16 => 'p',
    26u16 => '[',
    27u16 => ']',
    28u16 => '↲',
    29u16 => '\x07',
    30u16 => 'a',
    31u16 => 's',
    32u16 => 'd',
    33u16 => 'f',
    34u16 => 'g',
    35u16 => 'h',
    36u16 => 'j',
    37u16 => 'k',
    38u16 => 'l',
    39u16 => ';',
    40u16 => '\'',
    41u16 => '`',
    42u16 => '\x0E',
    43u16 => '\\',
    44u16 => 'z',
    45u16 => 'x',
    46u16 => 'c',
    47u16 => 'v',
    48u16 => 'b',
    49u16 => 'n',
    50u16 => 'm',
    51u16 => ',',
    52u16 => '.',
    56u16 => '↹',
    57u16 => ' ',
    103u16 => '⇧',
    105u16 => '⇦',
    106u16 => '⇨',
    108u16 => '⇩',
    111u16 => '\x7F',
    114u16 => '🔇',
    115u16 => '🔊'
};

pub const CHAR_TO_SHIFTED: phf::Map<char, char> = phf_map! {
    '`' => '~',
    '1' => '!',
    '2' => '@',
    '3' => '#',
    '4' => '$',
    '5' => '%',
    '6' => '^',
    '7' => '&',
    '8' => '*',
    '9' => '(',
    '0' => ')',
    '-' => '_',
    '=' => '+',
    'q' => 'Q',
    'w' => 'W',
    'e' => 'E',
    'r' => 'R',
    't' => 'T',
    'y' => 'Y',
    'u' => 'U',
    'i' => 'I',
    'o' => 'O',
    'p' => 'P',
    '[' => '{',
    ']' => '}',
    '\\' => '|',
    'a' => 'A',
    's' => 'S',
    'd' => 'D',
    'f' => 'F',
    'g' => 'G',
    'h' => 'H',
    'j' => 'J',
    'k' => 'K',
    'l' => 'L',
    ';' => ':',
    '\'' => '"',
    'z' => 'Z',
    'x' => 'X',
    'c' => 'C',
    'v' => 'V',
    'b' => 'B',
    'n' => 'N',
    'm' => 'M',
    ',' => '<',
    '.' => '>'
};

pub struct BoardState {
    state: [u8; 8]
}

impl BoardState {
    pub const CLEAR: BoardState = BoardState { state: [0, 0, 0, 0, 0, 0, 0, 0] };

    pub fn new_single(mods: u8, key: u8) -> Self {
        Self { state: [mods, 0, key, 0, 0, 0, 0, 0] }
    }

    fn get_key_mod(character: u8) -> u8 {
        match character {
            224 => CTRL,
            225 => SHIFT,
            226 => ALT,
            _ => 0b0000_0000
        }
    }

    pub fn push_key(&mut self, key: u8) -> bool {
        let key_mod = Self::get_key_mod(key);

        if key_mod != 0b0000_0000 {
            self.state[0] |= key_mod;
            return true;
        }

        let mut index = 0;
        for i in 2..7 {
            if self.state[i] == key {
                return true;
            }

            if index == 0 && self.state[i] == 0 {
                index = i;
            }
        }

        if index == 0 {
            return false;
        }

        self.state[index] = key;

        true
    }

    pub fn pop_key(&mut self, key: u8) {
        let key_mod = Self::get_key_mod(key);

        if key_mod != 0b0000_0000 {
            self.state[0] &= !key_mod;
            return;
        }

        for i in 2..7 {
            if self.state[i] == key {
                self.state[i] = 0;
                break;
            }
        }
    }

    pub fn to_event(&self) -> uhid_event {
        let mut data = uhid_event__bindgen_ty_1::default();

        let input= unsafe { &mut data.input2 };

        input.size = 8;
        input.data[..8].copy_from_slice(&self.state);

        uhid_event { type_: uhid_event_type_UHID_INPUT2, u: data }
    }
}

#[derive(Clone, Copy)]
pub struct KeyPress {
    key: u8,
    mods: u8
}

impl KeyPress {
    pub const fn new(key: u8, in_mods: &[u8]) -> Self {
        let mut mods = 0b0000_0000;

        let mut i = 0;
        while i < in_mods.len() {
            mods |= in_mods[i];
            i += 1;
        }

        KeyPress { key, mods }
    }

    pub fn add_mod(&mut self, new: u8) {
        self.mods |= new;
    }

    pub fn to_press(&self) -> BoardState {
        BoardState::new_single(self.mods, self.key)
    }

    pub fn to_release(&self) -> BoardState {
        BoardState::CLEAR
    }
}

#[allow(clippy::upper_case_acronyms)]
pub struct HIDWriter {
    file: File
}

impl HIDWriter {
    pub fn open() -> Result<Self, Error> {
        let mut uhid = Self { file: File::create("/dev/uhid")? };

        let mut data = uhid_event__bindgen_ty_1::default();

        let create= unsafe { &mut data.create2 };

        create.name[..NAME.len()].copy_from_slice(&NAME);
        create.rd_data[..DESC.len()].copy_from_slice(&DESC);
        create.rd_size = DESC.len() as u16;
        create.bus = BUS_USB as u16;
        create.vendor = 0x15D9;
        create.product = 0x0A37;

        uhid.push_event(&uhid_event { type_: uhid_event_type_UHID_CREATE2, u: data })?;
        Ok(uhid)
    }

    pub fn tap(&mut self, character: char) -> Result<(), Box<dyn std::error::Error>> {
        let inp = CHAR_TO_KEYPRESS.get(&character).ok_or("Invalid character")?;

        self.push_state(&inp.to_press())?;
        self.push_state(&inp.to_release())?;

        Ok(())
    }

    pub fn push_state(&mut self, state: &BoardState) -> Result<(), Box<dyn std::error::Error>> {
        self.push_event(&state.to_event())?;

        Ok(())
    }

    fn push_event(&mut self, event: &uhid_event) -> Result<(), Error> {
        self.file.write_all(unsafe { from_raw_parts(event as *const uhid_event as *const u8, size_of::<uhid_event>()) } )
    }
}

impl Drop for HIDWriter {
    fn drop(&mut self) {
        // If it errs it is not really a big deal there is nothing the code can do
        let _ = self.push_event(&uhid_event { type_: uhid_event_type_UHID_DESTROY, u: uhid_event__bindgen_ty_1::default() });
    }
}

pub struct HIDReader {
    file: File
}

pub struct KeyInput {
    pub character: char,
    pub time: Duration,
    pub down: bool
}

impl HIDReader {
    pub fn open(id: &str) -> Result<Self, Error> {
        let hid = Self { file: File::open("/dev/input/event".to_owned() + id)? };

        Ok(hid)
    }

    pub fn read(&mut self) -> Result<Option<KeyInput>, Box<dyn std::error::Error>> {
        // This isn't packed so I don't know why it is valid to load read in raw memory, but whatever
        // That's what the info I read said to do
        let mut input_event = input_event::default();

        // Probably nicer way to write the cast
        self.file.read_exact(unsafe { &mut *(&mut input_event as *mut input_event as *mut [u8; size_of::<input_event>()]) } )?;

        let duration = Duration::new(input_event.time.tv_sec as u64, input_event.time.tv_usec as u32 * 1000);

        let down = input_event.value == 1;
        if !(down || input_event.value == 0) || input_event.type_ != EV_KEY as u16 || input_event.code >= u8::MAX.into() {
            return Ok(None);
        }

        let maybe_char = CODE_TO_CHAR.get(&input_event.code);

        if let Some(character) = maybe_char {
            Ok(Some(KeyInput { character: *character, time: duration, down }))
        } else {
            Ok(None)
        }
    }

    pub fn read_valid(&mut self) -> Result<KeyInput, Box<dyn std::error::Error>> {
        loop {
            if let Some(res) = self.read()? {
                return Ok(res);
            }
        }
    }

    pub fn read_valid_down(&mut self) -> Result<KeyInput, Box<dyn std::error::Error>> {
        loop {
            if let Some(res) = self.read()? {
                if !res.down {
                    continue;
                }

                return Ok(res);
            }
        }
    }
}
