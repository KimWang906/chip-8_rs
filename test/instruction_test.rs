mod common;

#[test]
fn add_test() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        pc: 0,
        stack: [0; 16],
        sp: 0,
    };
}