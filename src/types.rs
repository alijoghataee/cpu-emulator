pub type EightBit = [bool; 8];

/// Index 0 is the most significant bit.
pub fn to_bits(n: u8) -> EightBit {
    std::array::from_fn(|i| (n >> (7 - i)) & 1 == 1)
}

pub fn from_bits(bits: EightBit) -> u8 {
    bits.iter().fold(0, |acc, &b| (acc << 1) | b as u8)
}

pub fn fmt_bits(bits: EightBit) -> String {
    bits.iter().map(|&b| if b { '1' } else { '0' }).collect()
}
