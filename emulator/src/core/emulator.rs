use crate::core::ram::*;
use crate::core::register::RegisterArray;

type EResult<T> = Result<T, &'static str>;

pub struct Emulator {
    pc: u16,
    sp: u16,
    ram: Box<dyn RAM>,
    reg: RegisterArray,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            pc: 0,
            sp: 0,
            ram: Box::new(DefaultRam::new()),
            reg: RegisterArray::new(),
        }
    }

    pub fn execute_next(&mut self) -> EResult<()> {
        let opcode = self.ram[self.pc];
        self.pc += 1;
        match opcode {
            0xc2 => {
                // JNZ adr
                self.jmp_not("zero")?;
            }
            0xc3 => {
                // JMP adr
                self.pc = self.read_addr()?;
            }
            0xc4 => unimplemented!(),
            0xc5 => {
                // PUSH B
                self.push_reg("bc")?;
            }
            0xc6 => unimplemented!(),
            0xc7 => {
                // RST 0
                self.call(0x0)?;
            }
            0xc8 => {
                // RZ
                if self.reg.get_flag("zero") {
                    self.pc = self.pop()?;
                }
            }
            0xc9 => {
                // RET
                self.pc = self.pop()?;
            }
            0xca => {
                // JZ adr
                self.jmp_if("zero")?;
                
            }
            0xcc => {
                // CZ addr
                self.call_if("zero")?;
            }
            0xcd => {
                // CALL addr
                self.call_imm()?;
            }
            0xce => {
                unimplemented!()
            }
            0xcf => {
                // RST 1
                self.call(0x8)?;
            }
            0xd0 => {
                // RNC
                if !self.reg.get_flag("carry") {
                    self.pc = self.pop()?;
                }
            }
            0xd1 => {
                // POP D
                self.reg["de"] = self.pop()?;
            }
            0xd2 => {
                // JNC adr
                self.jmp_not("carry")?;
            }
            0xd3 => {
                // OUT
                unimplemented!()
            }
            0xd4 => {
                // CNC adr
                self.call_not("carry")?;

            }
            0xd5 => {
                // PUSH D
                self.push_reg("de")?;
            }
            0xd6 => {
                // SUI D8
                unimplemented!()
            }
            0xd7 => {
                // RST 2
                unimplemented!()
            }
            0xd8 => {
                // RC
                unimplemented!()
            }
            0xd9 => {
                // no-op
                unimplemented!()
            }
            0xda => {
                // JC adr
                self.jmp_if("carry")?;
            }
            0xdb => {
                // Unimplemented
                unimplemented!()
            }
            0xdc => {
                // CC adr
                self.call_if("carry")?;
            }
            0xdd => {
                // Unimplemented
                unimplemented!()
            }
            0xde => {
                // Unimplemented
                unimplemented!()
            }
            0xdf => {
                // Unimplemented
                unimplemented!()
            }
            0xe0 => {
                // Unimplemented
                unimplemented!()
            }
            0xe1 => {
                // Unimplemented
                unimplemented!()
            }
            0xe2 => {
                // JPO adr
                self.jmp_not("parity")?;
            }
            0xe3 => {
                // Unimplemented
                unimplemented!()
            }
            0xe4 => {
                // CPO adr
                self.call_not("parity")?;
            }
            0xe5 => {
                // Unimplemented
                unimplemented!()
            }
            0xe6 => {
                // Unimplemented
                unimplemented!()
            }
            0xe7 => {
                // Unimplemented
                unimplemented!()
            }
            0xe8 => {
                // Unimplemented
                unimplemented!()
            }
            0xe9 => {
                // Unimplemented
                unimplemented!()
            }
            0xea => {
                // JPE adr
                self.jmp_if("parity")?;
            }
            0xeb => {
                // Unimplemented
                unimplemented!()
            }
            0xec => {
                // CPE
                self.call_if("parity")?;
            }
            0xed => {
                // Unimplemented
                unimplemented!()
            }
            0xee => {
                // Unimplemented
                unimplemented!()
            }
            0xef => {
                // Unimplemented
                unimplemented!()
            }
            0xf0 => {
                // Unimplemented
                unimplemented!()
            }
            0xf1 => {
                // Unimplemented
                unimplemented!()
            }
            0xf2 => {
                // JP adr
                self.jmp_not("sign")?;
            }
            0xf3 => {
                // Unimplemented
                unimplemented!()
            }
            0xf4 => {
                // CP adr
                self.call_not("sign")?;
            }
            0xf5 => {
                // Unimplemented
                unimplemented!()
            }
            0xf6 => {
                // Unimplemented
                unimplemented!()
            }
            0xf7 => {
                // Unimplemented
                unimplemented!()
            }
            0xf8 => {
                // Unimplemented
                unimplemented!()
            }
            0xf9 => {
                // Unimplemented
                unimplemented!()
            }
            0xfa => {
                // JM adr
                self.jmp_if("sign")?;
            }
            0xfb => {
                // Unimplemented
                unimplemented!()
            }
            0xfc => {
                // CM adr
                self.call_if("sign")?;
            }
            0xfd => {
                // Unimplemented
                unimplemented!()
            }
            0xfe => {
                // Unimplemented
                unimplemented!()
            }
            0xff => {
                // Unimplemented
                unimplemented!()
            }
            _ => unimplemented!("Opcode not yet implemented")
        }
        Ok(())
    }

    fn jmp_not(&mut self, flag: &str) -> EResult<()> {
        if !self.reg.get_flag(flag) {
            self.pc = self.read_addr()?;
        } else {
            self.pc += 2;
        }
        Ok(())
    }

    fn jmp_if(&mut self, flag: &str) -> EResult<()> {
        if self.reg.get_flag(flag) {
            self.pc = self.read_addr()?;
        } else {
            self.pc += 2;
        }
        Ok(())
    }

    fn call_not(&mut self, flag: &str) -> EResult<()> {
        if !self.reg.get_flag(flag) {
            self.call_imm()?;
        } else {
            self.pc += 2;
        }
        Ok(())
    }

    fn call_if(&mut self, flag: &str) -> EResult<()> {
        if self.reg.get_flag(flag) {
            self.call_imm()?;
        } else {
            self.pc += 2;
        }
        Ok(())
    }

    fn call_imm(&mut self) -> EResult<()> {
        let adr = self.read_addr()?;
        self.push(self.pc)?;
        self.pc = adr;
        Ok(())
    }

    fn call(&mut self, adr: u16) -> EResult<()> {
        self.push(self.pc)?;
        self.pc = adr;
        Ok(())
    }

    fn ret(&mut self) -> EResult<()> {
        self.pc = self.pop()?;
        Ok(())
    }

    fn push(&mut self, val: u16) -> EResult<()> {
        if self.sp < 2 {
            return Err("PUSH: No more stack space");
        }
        self.sp -= 1;
        self.ram[self.sp] = (val >> 8) as u8;
        self.sp -= 1;
        self.ram[self.sp] = val as u8;
        Ok(())
    }

    fn push_reg(&mut self, reg: &str) -> EResult<()> {
        self.push(self.reg[reg])
    }

    fn pop(&mut self) -> EResult<u16> {
        if self.sp + 2 > self.ram.size() as u16 {
            return Err("POP: No return address on the stack");
        }
        let low = self.ram[self.sp] as u16;
        self.sp += 1;
        let high = self.ram[self.sp] as u16;
        self.sp += 1;
        Ok((high << 8) | low)

    }

    fn read_addr(&mut self) -> EResult<u16> {
        if self.pc + 2 > self.ram.size() as u16 {
            return Err("READ_ADDR: Not enough bytes available");
        }
        let low = self.ram[self.pc] as u16;
        self.pc += 1;
        let high = self.ram[self.pc] as u16;
        self.pc += 1;
        Ok((high << 8) | low)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop() {
        let mut e = Emulator::new();

        e.sp = 0x3fff;
        e.push(0xabcd).expect("Push failed");
        assert_eq!(e.sp, 0x3ffd);
        assert_eq!(0xabcd, e.pop().expect("Fuck"));
        assert_eq!(e.sp, 0x3fff);
        assert_eq!(e.pop(), Err("POP: No return address on the stack"));

        e.sp = 0x1;
        assert_eq!(e.push(0x1234), Err("PUSH: No more stack space"));
    }

    #[test]
    fn call_ret() {
        let mut e = Emulator::new();

        e.sp = 0x3fff;
        e.ram[0x1234] = 0xc9;

        e.call(0x1234).expect("Fuck");
        assert_eq!(e.sp, 0x3fff - 2);
        assert_eq!(e.pc, 0x1234);

        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x0);
    }

    #[test]
    fn jmp_if() {
        let mut e = Emulator::new();

        e.ram.load_vec(vec![0x04, 0x00, 0x00, 0x00], 0);

        // Performs
        // a) one failing jmp (-> pc = 2)
        // b) one succeeding jmp (pc = ram[pc] = ram[2] -> 0)
        // c) Back in starting position
        // -> Repeat for each flag
        for flag in vec!["zero", "carry", "sign", "parity", "aux"] { 
            e.jmp_if(flag).expect("");
            assert_eq!(e.pc, 2);
            e.reg.set_flag(flag);
            e.jmp_if(flag).expect("");
            assert_eq!(e.pc, 0);
        }
    }

    #[test]
    fn jmp_not() {
        let mut e = Emulator::new();

        e.ram.load_vec(vec![0x04, 0x00, 0x00, 0x00], 0);
        e.reg.set_flags(0xff);

        // same as tests::jmp_if
        for flag in vec!["zero", "carry", "sign", "parity", "aux"] { 
            e.jmp_not(flag).expect("");
            assert_eq!(e.pc, 2);
            e.reg.flip_flag(flag);
            e.jmp_not(flag).expect("");
            assert_eq!(e.pc, 0);
        }
    }

    #[test]
    fn jmps() {
        let mut e = Emulator::new();

        // Test JMP
        e.ram.load_vec(vec![0xc3, 0x34, 0x12], 0);
        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x1234);

        // Test JZ
        e.ram.load_vec(vec![0xca, 0x03, 0x00, 0xca, 0x03, 0x00], 0x1234);
        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x1237);
        e.reg.set_flag("zero");
        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x0003);

        // Test JNZ
        e.ram.load_vec(vec![0xc2, 0x34, 0x12, 0xc2, 0x34, 0x12], 0x3);
        e.reg.set_flag("zero");
        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x6);
        e.reg.flip_flag("zero");
        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x1234);

        // Test JC
        e.ram.load_vec(vec![0xda, 0x03, 0x00, 0xda, 0x03, 0x00], 0x1234);
        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x1237);
        e.reg.set_flag("carry");
        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x0003);

        // Test JNC
        e.ram.load_vec(vec![0xd2, 0x34, 0x12, 0xd2, 0x34, 0x12], 0x3);
        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x6);
        e.reg.flip_flag("carry");
        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x1234);
    }

    #[test]
    fn call() {
        let mut e = Emulator::new();

        e.sp = 0x3fff;

        assert_eq!(e.pc, 0x0);
        e.call_if("carry").expect("Fuck");
        assert_eq!(e.pc, 0x2);
        e.reg.set_flag("carry");
        e.ram.load_vec(vec![0x34, 0x12], 2);
        e.call_if("carry").expect("Fuck");
        assert_eq!(e.pc, 0x1234);
        

    }

    #[test]
    fn calls() {
        let mut e = Emulator::new();

        e.sp = 0x3fff;
        e.pc = 0x1111;

        // Test CALL
        e.ram.load_vec(vec![0xcd, 0x34, 0x12], e.pc);
        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x1234);
        e.ret().expect("Fuck");
        assert_eq!(e.pc, 0x1114);

        // Test CZ
        e.ram.load_vec(vec![0xcc, 0x03, 0x00, 0xcc, 0x03, 0x00], 0x1114);
        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x1117);
        e.reg.set_flag("zero");
        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x0003);
        e.ret().expect("Fuck");
        assert_eq!(e.pc, 0x111a);
    }
}
