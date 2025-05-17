pub struct JoyPad {
    buttons: u8,
    strobe: bool,
    button_indx: u8,
}

const RIGHT_BUTTON: u8 = 0b1000_0000;
const LEFT_BUTTON: u8 = 0b0100_0000;
const DOWN_BUTTON: u8 = 0b0010_0000;
const UP_BUTTON: u8 = 0b0001_0000;
const START_BUTTON: u8 = 0b0000_1000;
const SELECT_BUTTON: u8 = 0b0000_0100;
const B_BUTTON: u8 = 0b0000_0010;
const A_BUTTON: u8 = 0b0000_0001;
const SHIFT_REG_STROBE: u8 = 0b1;
const NUM_OF_BUTTONS: u8 = 8;
const BUTTON_MASK: u8 = 1;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Buttons {
    Right = RIGHT_BUTTON,
    Left = LEFT_BUTTON,
    Down = DOWN_BUTTON,
    Up = UP_BUTTON,
    Start = START_BUTTON,
    Select = SELECT_BUTTON,
    B = B_BUTTON,
    A = A_BUTTON,
}

impl JoyPad {
    pub fn new() -> Self {
        JoyPad {
            buttons: 0,
            strobe: false,
            button_indx: 0,
        }
    }

    pub fn write(&mut self, data: u8) {
        self.strobe = (data & SHIFT_REG_STROBE) != 0;
        if self.strobe {
            self.button_indx = 0;
        }
    }

    pub fn read(&mut self) -> u8 {
        let mut res = 1;

        if self.button_indx < NUM_OF_BUTTONS {
            res = (self.buttons & (BUTTON_MASK << self.button_indx)) >> self.button_indx;
            if !self.strobe {
                self.button_indx += 1;
            }
        }

        res
    }

    pub fn set_button(&mut self, button: Buttons) {
        self.buttons |= button as u8;
    }

    pub fn unset_button(&mut self, button: Buttons) {
        self.buttons &= !(button as u8)
    }
}
