use crate::{adders::{Adder8Out, adder8, subtractor8}, big_logical_gates::{big_and, big_not, big_or, big_xor}, logical_gates::{and, nor, not, or}, multiplexer::eight_to_one_multiplexer, types::EightBit};

pub struct ALUOut {
    pub data: EightBit,
    pub zero: bool,
    pub carry: bool,
    pub negative: bool,
    pub overflow: bool,
}

pub fn alu(a: EightBit, b:EightBit, s0: bool, s1: bool, s2: bool) -> ALUOut {
    let add_op= adder8(a, b); // s2 = false, s1 = false, s0 = false | 000
    let sub_op= subtractor8(a, b); // s2 = false, s1 = false, s0 = true | 001
    let and_op= big_and(a, b); // s2 = false, s1 = true, s0 = false | 010
    let or_op= big_or(a, b); // s2 = false, s1 = true, s0 = true | 011
    let xor_op= big_xor(a, b); // s2 = true, s1 = false, s0 = false | 100
    let not_op= big_not(a); // s2 = true, s1 = false, s0 = true | 101

    let mut data: EightBit = [false; 8];

    for i in 0..8 {
        data[i] = eight_to_one_multiplexer([add_op.sum[i], sub_op.sum[i], and_op[i], or_op[i], xor_op[i], not_op[i], false, false], s0, s1, s2);
    }

    let mut zero = false;
    for &i in data.iter() {
        zero = nor(and(i, true), zero);
    }

    let carry: bool = eight_to_one_multiplexer([add_op.carry_out, sub_op.carry_out, false, false, false, false, false, false], s0, s1, s2);

    ALUOut {
        data,
        zero,
        carry,
        negative: data[0], // left most bit
        overflow: false,
    }
}