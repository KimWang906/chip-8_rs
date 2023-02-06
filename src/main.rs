// 목표: 1970년대에 유통된 CHIP-8 시스템의 부분집합을 구현
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

use cpu_emulator::system::cpu::CPU;
use cpu_emulator::console::io::read_input;

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        pc: 0,
        stack: [0; 16],
        sp: 0,
    };

    loop {
        let read = read_input();
        println!("{read:?}");
    }

    // cpu.run();
}
