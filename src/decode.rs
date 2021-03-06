use crate::mmu::MMU;
use crate::registers::Registers;

pub fn decode_instruction(opcode: u8, mmu: &MMU, regs: &Registers) -> String {
    match opcode {
        0x00 => String::from("NOP"),
        0x01 => String::from("LD BC,d16"),
        0x02 => {
            String::from(format!("LD (BC), A (BC = {:4X}, A = {:2X})", regs.get_bc(), regs.a))
        },
        0x04 => {
            String::from("INC B")
        }
        0x05 => String::from(format!("DEC B (B = {:2X})", regs.b)),
        0x06 => {
            let d8 = mmu.read(regs.pc + 1);
            String::from(format!("LD B, {:2X}", d8))
        },
        0x0B => {
            String::from("DEC BC")
        }
        0x0C => {
            String::from("INC C")
        }
        0x0D => {
            String::from(format!("DEC C (C = {:2X})", regs.c))
        }
        0x0E => {
            let d8 = mmu.read(regs.pc + 1);
            String::from(format!("LD C, {:2X}", d8))
        },
        0x11 => {
            let imm16 = mmu.read_wide(regs.pc + 1);
            String::from(format!("LD DE, {:4X}", imm16))
        }
        0x13 => {
            String::from("INC DE")
        }
        0x14 => {
            String::from("INC D")
        }
        0x15 => {
            String::from("DEC D")
        },
        0x16  => {
            String::from(format!("LD D,{:2X}", mmu.read(regs.pc + 1)))
        }
        0x17 => {
            String::from("RLA")
        }
        0x18 => {
            String::from(format!("JR {}", mmu.read(regs.pc + 1) as i8))
        }
        0x1A => {
            String::from("LD A,(DE)")
        }
        0x1D => {
            String::from("DEC E")
        }
        0x1E => {
            String::from(format!("LD E,{:2X}", mmu.read(regs.pc)))
        }
        0x1F => {
            String::from("RRA")
        },
        0x20 => {
            let r8 = mmu.read(regs.pc + 1);
            String::from(format!("JR NZ, {:2X}", r8))
        },
        0x21 => {
            let d16 = mmu.read_wide(regs.pc + 1);
            String::from(format!("LD HL, {:4X}", d16))
        },
        0x22 => {
            String::from("LD (HL+),A")
        }
        0x23 => {
            String::from("INC HL")
        }
        0x24 => {
            String::from("INC H")
        }
        0x25 => {
            String::from("DEC H")
        }
        0x28 => {
            String::from(format!("JR Z,{}", mmu.read(regs.pc + 1) as i8))
        }
        0x2A => {
            format!("LD A, (HL+) (HL = {:4X}", regs.get_hl())
        }
        0x2E => {
            String::from(format!("LD L,{:2X}", mmu.read(regs.pc + 1)))
        }
        0x2F => {
            String::from("CPL")
        }
        0x31 => {
            let d16 = mmu.read_wide(regs.pc + 1);
            String::from(format!("LD SP, {:4X}", d16))
        }
        0x32 => {
            String::from(format!("LD [HL-] (HL = {:4X}), A (A = {:2X})", regs.get_hl(), regs.a))
        }
        0x36 => {
            String::from(format!("LD (HL),{:2X}", mmu.read(regs.pc + 1)))
        }
        0x3E => {
            String::from(format!("LD A, {:2X}", mmu.read(regs.pc + 1)))
        }
        0x3D => {
            String::from("DEC A")
        }
        0x47 => {
            String::from("LD B,A")
        }
        0x4F => {
            String::from("LD C,A")
        }
        0x57 => {
            String::from("LD D,A")
        }
        0x67 => {
            String::from("LD H,A")
        }
        0x77 => {
            String::from("LD (HL),A")
        }
        0x78 => {
            String::from("LD A,B")
        }
        0x79 => {
            String::from("LD A,C")
        }
        0x7B => {
            String::from("LD A, E")
        }
        0x7C => {
            String::from("LD A,H")
        }
        0x7D => {
            String::from("LD A,L")
        }
        0x86 => {
            String::from("ADD A, (HL)")
        }
        0x90 => {
            String::from("SUB B")
        }
        0xA1 => {
            String::from("AND C")
        }
        0xA9 => {
            String::from("XOR C")
        }
        0xAF => {
            let val = regs.a;
            String::from(format!("XOR A (A = {:2X})", val))
        },
        0xB0 => {
            String::from("OR B")
        },
        0xB1 => {
            String::from("OR C")
        }
        0xB2 => {
            String::from("OR D")
        }
        0xB3 => {
            String::from("OR E")
        }
        0xB4 => {
            String::from("OR H")
        }
        0xB5 => {
            String::from("OR L")
        }
        0xB6 => {
            String::from("OR (BC)")
        }
        0xB7 => {
            String::from("OR A")
        }
        0xBE => {
            String::from("CP (HL)")
        }
        0xBF => {
            String::from("CP A")
        }
        0xC1 => {
            String::from("POP BC")
        }
        0xC3 => { 
            let a16 = mmu.read_wide(regs.pc + 1);
            String::from(format!("JP a16 {:4X}", a16))
        },
        0xC5 => {
            String::from("PUSH BC")
        }
        0xC9 => {
            String::from("RET")
        }
        0xCB => {
            let instr = mmu.read(regs.pc + 1);
            String::from(format!("CB {:2X}", instr))
        }
        0xCD => {
            let addr = mmu.read_wide(regs.pc + 1);
            String::from(format!("CALL {:4X}", addr))
        }
        0xE0 => {
            String::from(format!("LDH ($FF00 + {:2X}),A", mmu.read(regs.pc + 1)))
        }
        0xE2 => {
            String::from("LD ($FF00 + C),A")
        }
        0xE6 => {
            format!("AND {:2x}", mmu.read(regs.pc + 1))
        }
        0xEA => {
            String::from(format!("LD ({:4X}),A", mmu.read_wide(regs.pc + 1)))
        }
        0xEF => {
            String::from("RST 28h")
        }
        0xF0 => {
            String::from(format!("LDH A,($FF00 + {:2X})", mmu.read(regs.pc + 1)))
        }
        0xF3 => {
            String::from("DI")
        }
        0xFB => {
            String::from("EI")
        }
        0xFE => {
            String::from(format!("CP {:2X}", mmu.read(regs.pc + 1)))
        }
        _ => String::from("NOT IMPLEMENTED IN DECODER")
    }
}
