#![allow(dead_code)]
use system::cpu::CPU;

pub mod console;
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
fn add() {
    let mut cpu = setup();

    cpu.registers[0] = 1;
    cpu.registers[1] = 2;

    let mem = &mut cpu.memory;
    
    mem[0x000] = 0x80; mem[0x001] = 0x14; // add
    mem[0x002] = 0x00; mem[0x003] = 0x00; // 0x0000, HALT(end)

    cpu.run();

    assert_eq!(cpu.registers[0], 3);
}

#[test]
fn sub() {
    let mut cpu = setup();

    cpu.registers[0] = 5;
    cpu.registers[1] = 2;

    let mem = &mut cpu.memory;
    
    mem[0x000] = 0x80; mem[0x001] = 0x15; // sub
    mem[0x002] = 0x00; mem[0x003] = 0x00; // 0x0000, HALT(end)

    cpu.run();

    assert_eq!(cpu.registers[0], 3);
}

#[test]
fn mov() {
    let mut cpu = setup();

    cpu.registers[0] = 0;
    cpu.registers[1] = 1;

    let mem = &mut cpu.memory;

    mem[0x000] = 0x80; mem[0x001] = 0x10; // mov
    mem[0x002] = 0x00; mem[0x003] = 0x00; // 0x0000, HALT(end)

    cpu.run();

    assert_eq!(cpu.registers[0], 1);
}

#[test]
fn jmp() {
    let mut cpu = setup();

    cpu.registers[0] = 0;
    cpu.registers[1] = 1;

    let mem = &mut cpu.memory;

    mem[0x000] = 0x11; mem[0x001] = 0x00; // jmp 0x100

    // ... Jump 0x100
    mem[0x100] = 0x80; mem[0x101] = 0x10; // mov 
    mem[0x102] = 0x10; mem[0x103] = 0x02; // jmp 0x002

    // ... Jump 0x002
    mem[0x002] = 0x00; mem[0x003] = 0x00; // 0x0000, HALT(end)

    cpu.run();

    assert_eq!(cpu.registers[0], 1);
}

#[test]
fn function() {
    let mut cpu = setup();

    // add
    cpu.registers[0] = 0;
    cpu.registers[1] = 20;

    // sub
    cpu.registers[2] = 20;
    cpu.registers[3] = 20;

    // mov
    cpu.registers[4] = 0;
    cpu.registers[5] = 20;

    let mem = &mut cpu.memory;

    mem[0x000] = 0x21; mem[0x001] = 0x00; // call 0x100
    mem[0x002] = 0x00; mem[0x003] = 0x00; // no-op

    // call 0x100
    mem[0x100] = 0x80; mem[0x101] = 0x14; // add
    mem[0x102] = 0x82; mem[0x103] = 0x35; // sub
    mem[0x104] = 0x84; mem[0x105] = 0x50; // mov
    mem[0x106] = 0x00; mem[0x107] = 0xEE; // ret

    cpu.run();

    assert_eq!(cpu.registers[0], 20);
    assert_eq!(cpu.registers[2], 0);
    assert_eq!(cpu.registers[4], 20);
}
