use std::cell::RefCell;
use crate::system::mapping::*;

pub struct CPU {
    pub registers: [u8; 16], // Register, count: 16
    pub pc: usize, // Program Counter
    pub memory: [u8; 0x1000], // 4096byte
    pub stack: [u16; 16], // Stack, MAX: 15(index)
    pub sp: usize, // Stack Pointer
}

impl CPU {
    pub fn read_opcode(&self) -> u16 {
        let pc = self.pc;
        let op_byte1 = self.memory[pc] as u16;
        let op_byte2 = self.memory[pc + 1] as u16;

        // u16 옵코드를 생성하기 위해 메모리에서 두 값을 논리합(logical or) 연산으로 결합한다.
        // 메모리의 값은 먼저 u16으로 변환해야 하는데, 그렇지 않으면 왼쪽 시프트를 할 때 모든 비트가 0으로 결정된다.
        op_byte1 << 8 | op_byte2
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.pc += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let nnn = opcode & 0x0FFF; // F: select

            match (c, x, y, d) {
                NO_OP => { return; },
                RET   => self.ret(),
                (CALL, _, _, _)     => self.call(nnn),
                (0x8, _, _, ADD)    => self.add_xy(x, y),
                (0x8, _, _, SUB)    => self.sub_xy(x, y),
                (0x8, _, _, MOV)    => self.mov_xy(x, y),
                _                       => todo!("opcode {:04x}", opcode)
            }
        }
    }

    pub fn call(&mut self, addr: u16) {
        let sp = self.sp;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack Overflow!!");
        }

        stack[sp] = self.pc as u16;
        self.sp += 1;
        self.pc = addr as usize;
    }

    pub fn ret(&mut self) {
        if self.sp == 0 {
            panic!("Stack underflow");
        }

        self.sp -= 1;
        let addr = self.stack[self.sp];
        self.pc = addr as usize;
    }

    pub fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = dbg!(val);

        // CHIP-8 안에 있는 마지막 레지스터는 Carry Flag에 오버플로우의 여부를 저장한다.
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    pub fn sub_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_sub(arg2);
        self.registers[x as usize] = val;

        // CHIP-8 안에 있는 마지막 레지스터는 Carry Flag에 오버플로우의 여부를 저장한다.
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    pub fn mov_xy(&mut self, x: u8, y: u8) {
        let arg1 = RefCell::new(self.registers[x as usize]);
        let arg2 = self.registers[y as usize];
        *arg1.borrow_mut() = arg2;

        self.registers[x as usize] = arg1.into_inner();
    }
}