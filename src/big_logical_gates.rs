use crate::{
    logical_gates::{and, not, or, xor},
    types::EightBit,
};

pub fn big_and(a: EightBit, b: EightBit) -> EightBit {
    let mut result: EightBit = [false; 8];

    for i in 0..8 {
        result[i] = and(a[i], b[i]);
    }

    result
}

pub fn big_or(a: EightBit, b: EightBit) -> EightBit {
    let mut result: EightBit = [false; 8];

    for i in 0..8 {
        result[i] = or(a[i], b[i]);
    }

    result
}

pub fn big_xor(a: EightBit, b: EightBit) -> EightBit {
    let mut result: EightBit = [false; 8];

    for i in 0..8 {
        result[i] = xor(a[i], b[i]);
    }

    result
}

pub fn big_not(a: EightBit) -> EightBit {
    let mut result: EightBit = [false; 8];

    for i in 0..8 {
        result[i] = not(a[i]);
    }

    result
}
