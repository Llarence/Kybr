use std::{fs::File, io::{Error, Write}, slice::from_raw_parts};
use phf::phf_map;

use crate::uhid::{uhid_event, uhid_event__bindgen_ty_1, uhid_event_type_UHID_CREATE2, uhid_event_type_UHID_DESTROY, uhid_event_type_UHID_INPUT2, BUS_USB};

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

const SHIFT: u8 = 0b0000_0010;

const PRESS_MAP: phf::Map<char, KeyPress> = phf_map! {
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
    '\n' => KeyPress::new(40, &[]),
    ' ' => KeyPress::new(44, &[]),
    '\'' => KeyPress::new(52, &[]),
    ',' => KeyPress::new(54, &[]),
    '.' => KeyPress::new(55, &[])
};

pub struct KeyPress {
    key: u8,
    mods: u8
}

impl KeyPress {
    pub const fn new(key: u8, in_mods: &[u8]) -> Self {
        let mut mods = 0;

        let mut i = 0;
        while i < in_mods.len() {
            mods |= in_mods[i];
            i += 1;
        }

        KeyPress{ key, mods }
    }

    pub fn to_input(&self) -> [u8; 8] {
        [self.mods, 0, self.key, 0, 0, 0, 0, 0]
    }
}

#[allow(clippy::upper_case_acronyms)]
pub struct UHID {
    file: File
}

impl UHID {
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

    pub fn press(&mut self, character: char) -> Result<(), Error> {
        self.push_input_event(&PRESS_MAP[&character].to_input())?;
        self.push_input_event(&[0, 0, 0, 0, 0, 0, 0, 0])?;

        Ok(())
    }

    fn push_input_event(&mut self, event: &[u8; 8]) -> Result<(), Error> {
        let mut data = uhid_event__bindgen_ty_1::default();

        let input= unsafe { &mut data.input2 };

        input.size = 8;
        input.data[..8].copy_from_slice(event);

        self.push_event(&uhid_event { type_: uhid_event_type_UHID_INPUT2, u: data })?;

        Ok(())
    }

    fn push_event(&mut self, event: &uhid_event) -> Result<(), Error> {
        self.file.write_all(unsafe { from_raw_parts(event as *const uhid_event as *const u8, size_of::<uhid_event>()) } )
    }
}

impl Drop for UHID {
    fn drop(&mut self) {
        // If it errs it is not really a big deal there is nothing the code can do
        let _ = self.push_event(&uhid_event { type_: uhid_event_type_UHID_DESTROY, u: uhid_event__bindgen_ty_1::default() });
    }
}
