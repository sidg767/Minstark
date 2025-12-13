use crate::field::BaseElement;
pub const NUM_FULL_ROUNDS: usize = 4;
pub const NUM_PARTIAL_ROUNDS: usize = 4;
pub const ALPHA: u64 = 5;
pub const MDS: [[BaseElement; 3]; 3]=[
    [BaseElement(1), BaseElement(2), BaseElement(3)],
    [BaseElement(4),BaseElement(5),BaseElement(6)],
    [BaseElement(7),BaseElement(8),BaseElement(9)],
];
pub const ROUND_CONSTANTS:[[BaseElement; 3];NUM_FULL_ROUNDS + NUM_PARTIAL_ROUNDS]=[
    [BaseElement(1), BaseElement(2), BaseElement(3)],
    [BaseElement(4),BaseElement(5),BaseElement(6)],
    [BaseElement(7),BaseElement(8),BaseElement(9)],
    [BaseElement(10),BaseElement(11),BaseElement(12)],
    [BaseElement(13),BaseElement(14),BaseElement(15)],
    [BaseElement(16),BaseElement(17),BaseElement(18)],
    [BaseElement(19),BaseElement(20),BaseElement(21)],
    [BaseElement(22),BaseElement(23),BaseElement(24)],
];
