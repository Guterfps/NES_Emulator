use super::Reg;

pub struct Sweep {
    data: u8,

    divider_count: u8,
    reload_flag: bool,
    mute: bool,
}

pub const ENABLE_MASK: u8 = 0b1000_0000;
pub const PERIOD_MASK: u8 = 0b0111_0000;
pub const NEGATE_MAKS: u8 = 0b0000_1000;
pub const SHIFT_MASK: u8 = 0b0000_0111;

const MUTE_MIN: u16 = 8;
const MUTE_MAX: u16 = 0x7FF;
const PERIOD_OFFSET: usize = 4;

impl Sweep {
    pub fn new() -> Self {
        Self {
            data: 0,
            divider_count: 0,
            reload_flag: false,
            mute: false,
        }
    }

    pub fn tick(&mut self, current_period: u16, is_pulse_1: bool) -> Option<u16> {
        let change_amount = current_period >> (self.data & SHIFT_MASK);
        let mut target_period = current_period;
        let mut res = None;

        if (self.data & NEGATE_MAKS) != 0 {
            target_period = target_period.wrapping_sub(change_amount);

            if is_pulse_1 && (target_period >= 1) {
                target_period -= 1;
            }
        } else {
            target_period = target_period.wrapping_add(change_amount);
        }

        self.mute = (current_period < MUTE_MIN) || (target_period > MUTE_MAX);

        if !self.mute {
            if (self.divider_count == 0) && self.reload_flag {
                self.reload_divider();
                self.reload_flag = false;
            } else if self.divider_count > 0 {
                self.divider_count -= 1;
            } else {
                self.reload_divider();
                self.reload_flag = false;

                if ((self.data & ENABLE_MASK) != 0) && ((self.data & SHIFT_MASK) != 0) && !self.mute
                {
                    res = Some(target_period);
                }
            }
        }

        res
    }

    fn reload_divider(&mut self) {
        let p = (self.data & PERIOD_MASK) >> PERIOD_OFFSET;
        self.divider_count = p;
    }

    pub fn is_muted(&self) -> bool {
        self.mute
    }
}

impl Reg for Sweep {
    fn data(&self) -> u8 {
        self.data
    }

    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }

    fn write(&mut self, mask: u8, val: u8) {
        self.data = (self.data & !mask) | (val & mask);
        self.reload_flag = true;
    }
}
