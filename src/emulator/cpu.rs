use status::*;

mod status;

pub struct CPU6502 {
    status_reg: StatusReg,
    program_counter: u16,
    stack_pointer: u8,
    accumulator: u8,
    indx_reg_x: u8,
    indx_reg_y: u8,
}

impl CPU6502 {
    pub fn new() -> Self {
        CPU6502 {
            status_reg: StatusReg::new(),
            program_counter: 0,
            stack_pointer: 0,
            accumulator: 0,
            indx_reg_x: 0,
            indx_reg_y: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        let mut break_status: bool = false;

        while !break_status {
            let op_code = program[self.program_counter as usize];
            self.program_counter += 1;

            break_status = self.op_code_instraction(op_code, &program);
        }
    }

    // load and store ops
    fn lda(&mut self, program: &[u8]) {
        let param = program[self.program_counter as usize];
        self.program_counter += 1;
        self.accumulator = param;

        Self::update_status_zero_flag(self.accumulator, &mut self.status_reg);
        Self::update_status_negative_flag(self.accumulator, &mut self.status_reg);
    }

    // transfer ops
    fn tax(&mut self) {
        self.indx_reg_x = self.accumulator;

        Self::update_status_zero_flag(self.indx_reg_x, &mut self.status_reg);
        Self::update_status_negative_flag(self.indx_reg_x, &mut self.status_reg);
    }

    fn update_status_zero_flag(reg: u8, status: &mut StatusReg) {
        if reg == 0 {
            status.set_flag(ZERO_FLAG);
        } else {
            status.unset_flag(ZERO_FLAG);
        }
    }

    fn update_status_negative_flag(reg: u8, status: &mut StatusReg) {
        if is_negative(reg) {
            status.set_flag(NEGATIVE_FLAG);
        } else {
            status.unset_flag(NEGATIVE_FLAG);
        }
    }

    fn op_code_instraction(&mut self, op_code: u8, program: &[u8]) -> bool {
        let mut is_break = false;

        match op_code {
            0xA9 => self.lda(program),
            0xAA => self.tax(),
            0x00 => is_break = true,
            _ => todo!(),
        };

        is_break
    }
}

fn is_negative(val: u8) -> bool {
    const NEGATIVE_BIT: u8 = 0b1000_0000;
    (val & NEGATIVE_BIT) != 0
}

#[cfg(test)]
mod test;
