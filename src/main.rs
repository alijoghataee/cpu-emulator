mod adders;
mod alu;
mod big_logical_gates;
mod logical_gates;
mod multiplexer;
mod types;

use adders::{adder8, full_adder, subtractor8};
use logical_gates::{and, nand, nor, not, or, xnor, xor};
use types::{fmt_bits, from_bits, to_bits};

fn main() {
    test_gates();
    test_full_adder();

    for (a, b) in [(3, 7), (100, 55), (200, 100), (255, 1)] {
        test_add(a, b);
    }
    for (a, b) in [(7, 3), (100, 55), (3, 7), (0, 1)] {
        test_sub(a, b);
    }
}

fn test_gates() {
    println!("=== Logic gates ===");
    println!(" a b | and or nand nor xor xnor | not(a)");
    for (a, b) in [(false, false), (false, true), (true, false), (true, true)] {
        println!(
            " {} {} |  {}   {}   {}    {}   {}   {}   |   {}",
            a as u8,
            b as u8,
            and(a, b) as u8,
            or(a, b) as u8,
            nand(a, b) as u8,
            nor(a, b) as u8,
            xor(a, b) as u8,
            xnor(a, b) as u8,
            not(a) as u8,
        );
    }
    println!();
}

fn test_full_adder() {
    println!("=== Full adder ===");
    println!(" a b cin | sum cout");
    for n in 0..8u8 {
        let (a, b, cin) = (n & 4 != 0, n & 2 != 0, n & 1 != 0);
        let out = full_adder(a, b, cin);
        println!(
            " {} {}  {}  |  {}    {}",
            a as u8, b as u8, cin as u8, out.sum as u8, out.carry_out as u8
        );
    }
    println!();
}

/// Ripples through the bits one by one (LSB first) so each step is visible,
/// then checks the result against `adder8`.
fn test_add(a: u8, b: u8) {
    println!("=== Add {a} + {b} ===");
    let (a_bits, b_bits) = (to_bits(a), to_bits(b));
    println!("  a = {} ({a})", fmt_bits(a_bits));
    println!("  b = {} ({b})", fmt_bits(b_bits));

    let mut carry = false;
    for bit in (0..8).rev() {
        let out = full_adder(a_bits[bit], b_bits[bit], carry);
        println!(
            "  bit {}: a={} b={} cin={} -> sum={} cout={}",
            7 - bit,
            a_bits[bit] as u8,
            b_bits[bit] as u8,
            carry as u8,
            out.sum as u8,
            out.carry_out as u8
        );
        carry = out.carry_out;
    }

    let result = adder8(a_bits, b_bits);
    let (expected, overflow) = a.overflowing_add(b);
    report(from_bits(result.sum), result.carry_out, expected, overflow);
}

fn test_sub(a: u8, b: u8) {
    println!("=== Subtract {a} - {b} ===");
    let (a_bits, b_bits) = (to_bits(a), to_bits(b));
    println!("  a      = {} ({a})", fmt_bits(a_bits));
    println!("  b      = {} ({b})", fmt_bits(b_bits));
    println!(
        "  not(b) = {}  (a - b = a + not(b) + 1)",
        fmt_bits(b_bits.map(not))
    );

    let result = subtractor8(a_bits, b_bits);
    // In two's complement subtraction, carry_out = 1 means no borrow.
    let (expected, borrow) = a.overflowing_sub(b);
    report(from_bits(result.sum), result.carry_out, expected, !borrow);
}

fn report(got: u8, carry: bool, expected: u8, expected_carry: bool) {
    let ok = got == expected && carry == expected_carry;
    println!(
        "  result = {} ({got}), carry = {} | expected {expected}, carry = {} -> {}",
        fmt_bits(to_bits(got)),
        carry as u8,
        expected_carry as u8,
        if ok { "PASS" } else { "FAIL" }
    );
    println!();
}
