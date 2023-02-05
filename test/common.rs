use cpu_emulator::system::cpu::CPU;

pub fn setup() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        pc: 0,
        stack: [0; 16],
        sp: 0,
    };
}