use crate::{
    logical_gates::{and, not, or},
    types::EightBit,
};

pub fn two_to_one_multiplexer(d0: bool, d1: bool, s: bool) -> bool {
    or(and(d0, not(s)), and(d1, s))
}

pub fn four_to_one_multiplexer(d0: bool, d1: bool, d2: bool, d3: bool, s0: bool, s1: bool) -> bool {
    two_to_one_multiplexer(
        two_to_one_multiplexer(d0, d1, s0),
        two_to_one_multiplexer(d2, d3, s0),
        s1,
    )
}

pub fn eight_to_one_multiplexer(d: EightBit, s0: bool, s1: bool, s2: bool) -> bool {
    two_to_one_multiplexer(
        four_to_one_multiplexer(d[0], d[1], d[2], d[3], s0, s1),
        four_to_one_multiplexer(d[4], d[5], d[6], d[7], s0, s1),
        s2,
    )
}
