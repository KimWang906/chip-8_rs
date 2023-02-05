#![allow(dead_code)]
use system::cpu::CPU;
pub mod system;

fn setup() -> CPU {
    let cpu = system::cpu::CPU {
        registers: [0; 16],
        memory: [0; 4096],
        pc: 0,
        stack: [0; 16],
        sp: 0,
    };

    cpu
}

#[test]
fn add_inst() {
    let mut cpu = setup();

    cpu.registers[0] = 1;
    cpu.registers[1] = 2;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; mem[0x001] = 0x00; // 0x2100, 0x2(call) | 0x100(addr)
    mem[0x004] = 0x00; mem[0x005] = 0x00; // 0x0000, HALT(end)

    mem[0x100] = 0x80; mem[0x101] = 0x14; // add
    mem[0x102] = 0x00; mem[0x103] = 0xEE; // 0x00EE, ret

    cpu.run();

    assert_eq!(cpu.registers[0], 3);
}

#[test]
fn sub_inst() {
    let mut cpu = setup();

    cpu.registers[0] = 5;
    cpu.registers[1] = 2;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; mem[0x001] = 0x00; // 0x2100, 0x2(call) | 0x100(addr)
    mem[0x004] = 0x00; mem[0x005] = 0x00; // 0x0000, HALT(end)

    mem[0x100] = 0x80; mem[0x101] = 0x15; // sub
    mem[0x102] = 0x00; mem[0x103] = 0xEE; // 0x00EE, ret

    cpu.run();

    assert_eq!(cpu.registers[0], 3);
}

#[test]
fn mov_inst() {
    let mut cpu = setup();

    cpu.registers[0] = 0;
    cpu.registers[1] = 1;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; mem[0x001] = 0x00; // 0x2100, 0x2(call) | 0x100(addr)
    mem[0x004] = 0x00; mem[0x005] = 0x00; // 0x0000, HALT(end)

    mem[0x100] = 0x80; mem[0x101] = 0x10; // mov
    mem[0x102] = 0x00; mem[0x103] = 0xEE; // 0x00EE, ret

    cpu.run();

    assert_eq!(cpu.registers[0], 1);
}