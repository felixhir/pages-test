use std::cell::RefCell;
use std::rc::Rc;

use crate::core::io::*;
use crate::core::ram::*;
use crate::core::register::RegisterArray;

pub type EResult<T> = Result<T, &'static str>;

pub struct Emulator {
    pc: u16,
    sp: u16,
    ram: Box<dyn RAM>,
    reg: RegisterArray,
    input_devices: [Option<Rc<RefCell<dyn InputDevice>>>; 256],
    output_devices: [Option<Rc<RefCell<dyn OutputDevice>>>; 256],
    running: bool,
    interrupts_enabled: bool,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            pc: 0,
            sp: 0,
            ram: Box::new(DefaultRam::new()),
            reg: RegisterArray::new(),
            input_devices: unsafe { std::mem::zeroed() },
            output_devices: unsafe { std::mem::zeroed() },
            running: true,
            interrupts_enabled: true, // INTE
        }
    }

    fn execute_instruction(&mut self, opcode: u8) -> EResult<()> {
        match opcode {
            0x01 => {
                // LXI B, D16
                self.lxi("bc")?;
            }
            0x06 => {
                // MVI B, D8
                self.mvi('b')?;
            }
            0x0e => {
                // MVI C, D8
                self.mvi('c')?;
            }
            0x11 => {
                // LXI D, D16
                self.lxi("de")?;
            }
            0x16 => {
                // MVI D, D8
                self.mvi('d')?;
            }
            0x1e => {
                // MVI E, D8
                self.mvi('e')?;
            }
            0x21 => {
                // LXI H, D16
                self.lxi("hl")?;
            }
            0x26 => {
                // MVI H, D8
                self.mvi('h')?;
            }
            0x2e => {
                // MVI L, D8
                self.mvi('l')?;
            }
            0x31 => {
                // LXI SP, D16
                self.sp = self.read_addr()?;
            }
            0x36 => {
                // MVI M, D8
                self.mvi_adr()?;
            }
            0x3e => {
                // MVI A, D8
                self.mvi('a')?;
            }
            0x40..=0x7f => {
                if opcode == 0x76 {
                    // HLT
                    self.running = false;
                } else {
                    // MOV DST, SRC
                    self.resolve_mov(opcode)?;
                }
            }
            0x80..=0x87 => {
                // ADD
                self.add(opcode, false)?;
            }
            0x88..=0x8F => {
                // ADC
                self.add(opcode, true)?;
            }
            0x90..=0x97 => {
                // SUB
                self.sub(opcode, false)?;
            }
            0x98..=0x9F => {
                // SBB
                self.sub(opcode, true)?;
            }
            0xA0..=0xA7 => {
                // ANA
                self.and(opcode)?;
            }
            0xA8..=0xAF => {
                // XRA
                self.xor(opcode)?;
            }
            0xB0..=0xB7 => {
                // ORA
                self.or(opcode)?;
            }
            0xB8..=0xBF => {
                // CMP
                self.cmp(opcode)?;
            }
            0xc0 => {
                // RNZ
                self.ret_not("zero")?;
            }
            0xc1 => {
                // Unimplemented
                unimplemented!();
            }
            0xc2 => {
                // JNZ adr
                self.jmp_not("zero")?;
            }
            0xc3 => {
                // JMP adr
                self.pc = self.read_addr()?;
            }
            0xc4 => {
                // Unimplemented
                unimplemented!();
            }
            0xc5 => {
                // PUSH B
                self.push_reg("bc")?;
            }
            0xc6 => {
                // Unimplemented
                unimplemented!();
            }
            0xc7 => {
                // RST 0
                self.call(0x0)?;
            }
            0xc8 => {
                // RZ
                self.ret_if("zero")?;
            }
            0xc9 => {
                // RET
                self.ret()?;
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
                // Unimplemented
                unimplemented!()
            }
            0xcf => {
                // RST 1
                self.call(0x8)?;
            }
            0xd0 => {
                // RNC
                self.ret_not("carry")?;
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
                let port = self.read_byte()?;
                self.output(port)?;
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
                self.call(0x10)?;
            }
            0xd8 => {
                // RC
                self.ret_if("carry")?;
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
                // IN
                let port = self.read_byte()?;
                self.input(port)?;
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
                // RST 3
                self.call(0x18)?;
            }
            0xe0 => {
                // RPO
                self.ret_not("parity")?;
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
                // RST 4
                self.call(0x20)?;
            }
            0xe8 => {
                // RPE
                self.ret_if("parity")?;
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
                // RST 5
                self.call(0x28)?;
            }
            0xf0 => {
                // RP
                self.ret_not("sign")?;
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
                // DI
                self.interrupts_enabled = false;
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
                // RST 6
                self.call(0x30)?;
            }
            0xf8 => {
                // RM
                self.ret_if("sign")?;
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
                // EI
                self.interrupts_enabled = true;
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
                // RST 7
                self.call(0x38)?;
            }
            _ => unimplemented!("Opcode not yet implemented"),
        }
        Ok(())
    }

    fn execute_next(&mut self) -> EResult<()> {
        let opcode = self.ram[self.pc];
        self.pc += 1;
        self.execute_instruction(opcode)
    }

    fn read_byte(&mut self) -> EResult<u8> {
        if self.pc + 1 > self.ram.size() as u16 {
            return Err("READ_BYTE: Not enough bytes available");
        }
        self.pc += 1;
        Ok(self.ram[self.pc - 1])
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

    pub fn load_ram(&mut self, data: Vec<u8>, start: u16) {
        self.ram.load_vec(data, start)
    }

    pub fn interrupt(&mut self, opcode: u8) -> EResult<()> {
        if self.interrupts_enabled {
            self.interrupts_enabled = false;
            return self.execute_instruction(opcode);
        }
        Err("Interrupts disabled")
    }
}

mod instructions;
mod devices;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::load_asm_file;
    use std::io;

    #[test]
    fn int() -> io::Result<()> {
        let mut emu = Emulator::new();
        load_asm_file(&mut emu, "./src/core/asm/int.s")?;

        emu.pc = 0x03;
        emu.sp = 0x3fff;

        // Test DI and EI
        emu.execute_next().expect("");
        assert!(!emu.interrupts_enabled);
        emu.execute_next().expect("");
        assert!(emu.interrupts_enabled);

        emu.execute_next().expect("");
        assert_eq!(emu.reg['c'], 69);

        emu.interrupt(0xc7).expect("");
        assert_eq!(emu.pc, 0);
        assert!(!emu.interrupts_enabled);

        assert_eq!(emu.interrupt(0x0), Err("Interrupts disabled"));

        emu.execute_next().expect("");
        emu.execute_next().expect("");

        assert_eq!(emu.reg['b'], 69);
        assert_eq!(emu.pc, 0x07);

        emu.execute_next().expect("");

        assert_eq!(emu.reg['h'], 69);

        // TODO: Add another test for non RST instruction interrupts
        Ok(())
    }
}
