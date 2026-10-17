use crate::logical_gates::{and, not, or, xor};
use crate::types::EightBit;

pub struct AdderOut {
    pub sum: bool,
    pub carry_out: bool,
}

pub struct Adder8Out {
    pub sum: EightBit,
    pub carry_out: bool,
}

pub fn full_adder(a: bool, b: bool, carry_in: bool) -> AdderOut {
    AdderOut {
        sum: xor(xor(a, b), carry_in),
        carry_out: or(and(a, b), and(xor(a, b), carry_in)),
    }
}

pub fn adder8(a: EightBit, b: EightBit) -> Adder8Out {
    let mut adder8_out = Adder8Out {
        sum: [false; 8],
        carry_out: false,
    };

    for i in (0..8).rev() {
        let adder_out = full_adder(a[i], b[i], adder8_out.carry_out);
        adder8_out.sum[i] = adder_out.sum;
        adder8_out.carry_out = adder_out.carry_out;
    }
    adder8_out
}

pub fn subtractor8(a: EightBit, b: EightBit) -> Adder8Out {
    let mut subtractor8_out = Adder8Out {
        sum: [false; 8],
        carry_out: true,
    };

    for i in (0..8).rev() {
        let subtractor_out = full_adder(a[i], not(b[i]), subtractor8_out.carry_out);
        subtractor8_out.sum[i] = subtractor_out.sum;
        subtractor8_out.carry_out = subtractor_out.carry_out;
    }
    subtractor8_out
}
