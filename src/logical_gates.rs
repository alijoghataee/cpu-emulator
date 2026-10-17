pub fn and(a: bool, b: bool) -> bool {
    match (a, b) {
        (true, true) => true,
        _ => false,
    }
}

pub fn or(a: bool, b: bool) -> bool {
    match (a, b) {
        (false, false) => false,
        _ => true,
    }
}

pub fn not(a: bool) -> bool {
    if a { false } else { true }
}

pub fn nand(a: bool, b: bool) -> bool {
    not(and(a, b))
}

pub fn nor(a: bool, b: bool) -> bool {
    not(or(a, b))
}

pub fn xor(a: bool, b: bool) -> bool {
    if a != b { true } else { false }
}

pub fn xnor(a: bool, b: bool) -> bool {
    not(xor(a, b))
}
