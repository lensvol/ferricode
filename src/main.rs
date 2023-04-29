mod intcode;

use std::fmt::{Display, Formatter};
use std::ops::Range;
use crate::intcode::computer::Computer;

fn main() {
    let program = vec![
        4, 3,
        101, 72, 14, 3,
        101, 1, 4, 4,
        5, 3, 16,
        99,
        29, 7, 0, 3, -67, -12, 87, -8, 3, -6, -8, -67, -23, -10
    ];

    let mut computer = Computer::new(program, vec![4, 3, 2, 1, 0]);
    computer.run();

    println!("Output: {:?}", computer.output);
}
