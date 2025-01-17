struct Cpu {
    registers: [u8; 16],
    position_in_memory: usize, // program counter
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl Cpu {
    fn run(&mut self) {
        loop {
            let op_byte1 = self.memory[self.position_in_memory] as u16;
            let op_byte2 = self.memory[self.position_in_memory + 1] as u16;
            let op_code: u16 = op_byte1 << 8 | op_byte2;

            let x = ((op_code & 0x0F00) >> 8) as u8;
            let y = ((op_code & 0x00F0) >> 4) as u8;
            // let n  =       ((opcode & 0x000F) >>  4) as u8;
            let kk = (op_code & 0x00FF) as u8;
            let op_minor = (op_code & 0x000F) as u8;
            let addr = op_code & 0x0FFF;

            self.position_in_memory += 2;

            match op_code {
                0x0000 => {
                    return; // Let's call it 'HALT'
                }
                0x00E0 => { /* CLEAR SCREEN */ }
                0x00EE => {
                    self.ret();
                }
                0x1000..=0x1FFF => {
                    self.jmp(addr);
                }
                0x2000..=0x2FFF => {
                    self.call(addr);
                }
                0x3000..=0x3FFF => {
                    self.se(x, kk);
                }
                0x4000..=0x4FFF => {
                    self.sne(x, kk);
                }
                0x5000..=0x5FFF => {
                    self.se(x, y);
                }
                0x6000..=0x6FFF => {
                    self.ld(x, kk);
                }
                0x7000..=0x7FFF => {
                    self.add(x, kk);
                }
                0x8000..=0x8FFF => match op_minor {
                    0 => self.ld(x, self.registers[y as usize]),
                    1 => self.or_xy(x, y),
                    2 => self.and_xy(x, y),
                    3 => self.xor_xy(x, y),
                    4 => {
                        self.add_xy(x, y);
                    }
                    _ => {
                        todo!("opcode: {:04x}", op_code);
                    }
                },
                _ => todo!("opcode {:04x}", op_code),
            }
        }
    }

    /// (6xkk) LD sets the value `kk` into register `vx`
    fn ld(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] == kk;
    }

    /// (7xkk) Add sets the value `kk` into register `vx`
    fn add(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] += kk;
    }

    fn se(&mut self, vx: u8, kk: u8) {
        if vx == kk {
            self.position_in_memory += 2;
        }
    }

    /// () SNE Store if not equal
    fn sne(&mut self, vx: u8, kk: u8) {
        if vx != kk {
            self.position_in_memory += 2;
        }
    }

    /// (1nnn) JUMP to `addr`
    fn jmp(&mut self, addr: u16) {
        self.position_in_memory = addr as usize;
    }

    /// (2nnn) CALL sub-routine at `addr`
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp >= stack.len() {
            panic!("Stack overflow");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    /// (00ee) RET return from the current sub-routine
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize;
    }

    // (7xkk)
    fn add_xy(&mut self, x: u8, y: u8) {
        let src_reg = x as usize;
        let dst_reg = y as usize;
        self.registers[src_reg] += self.registers[dst_reg];

        // TODO: SET CARRY FLAG!!!!
    }

    fn and_xy(&mut self, x: u8, y: u8) {
        let src_reg = x as usize;
        let dst_reg = y as usize;
        let x_ = self.registers[src_reg];
        let y_ = self.registers[dst_reg];

        self.registers[src_reg] = x_ & y_;
    }

    fn or_xy(&mut self, x: u8, y: u8) {
        let src_reg = x as usize;
        let dst_reg = y as usize;
        let x_ = self.registers[src_reg];
        let y_ = self.registers[dst_reg];

        self.registers[src_reg] = x_ | y_;
    }

    fn xor_xy(&mut self, x: u8, y: u8) {
        let src_reg = x as usize;
        let dst_reg = y as usize;
        let x_ = self.registers[src_reg];
        let y_ = self.registers[dst_reg];

        self.registers[src_reg] = x_ ^ y_;
    }
}

fn main() {
    let mut cpu = Cpu {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    //
    cpu.memory[0x000] = 0x21;
    cpu.memory[0x001] = 0x00;
    cpu.memory[0x002] = 0x21;
    cpu.memory[0x003] = 0x00;

    cpu.memory[0x100] = 0x80;
    cpu.memory[0x101] = 0x14;
    cpu.memory[0x102] = 0x80;
    cpu.memory[0x103] = 0x14;
    cpu.memory[0x104] = 0x00;
    cpu.memory[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
