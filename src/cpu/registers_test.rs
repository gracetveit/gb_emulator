use crate::cpu::registers::Registers;

#[test]
#[should_panic]
fn init_with_fewer_than_8_registers() {
    let registers: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0];
    Registers::new(&registers);
}

#[test]
#[should_panic]
fn init_with_more_than_8_registers() {
    let registers: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    Registers::new(&registers);
}

// #[test]
// fn get_set_bc() {
//     let test = Registers::new(vec![0, 0, 0, 0, 0, 0, 0, 0]);
//     test.set_bc(3640)
// }
