pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

pub fn not(a: bool) -> bool {
    nand(a, true)
}

pub fn not16(a: [bool; 16]) -> [bool; 16] {
    [
        not(a[0]),
        not(a[1]),
        not(a[2]),
        not(a[3]),
        not(a[4]),
        not(a[5]),
        not(a[6]),
        not(a[7]),
        not(a[8]),
        not(a[9]),
        not(a[10]),
        not(a[11]),
        not(a[12]),
        not(a[13]),
        not(a[14]),
        not(a[15]),
    ]
}

pub fn and(a: bool, b: bool) -> bool {
    not(nand(a, b))
}

pub fn and16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    [
        and(a[0], b[0]),
        and(a[1], b[1]),
        and(a[2], b[2]),
        and(a[3], b[3]),
        and(a[4], b[4]),
        and(a[5], b[5]),
        and(a[6], b[6]),
        and(a[7], b[7]),
        and(a[8], b[8]),
        and(a[9], b[9]),
        and(a[10], b[10]),
        and(a[11], b[11]),
        and(a[12], b[12]),
        and(a[13], b[13]),
        and(a[14], b[14]),
        and(a[15], b[15]),
    ]
}

pub fn or(a: bool, b: bool) -> bool {
    nand(not(a), not(b))
}

pub fn or16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    [
        or(a[0], b[0]),
        or(a[1], b[1]),
        or(a[2], b[2]),
        or(a[3], b[3]),
        or(a[4], b[4]),
        or(a[5], b[5]),
        or(a[6], b[6]),
        or(a[7], b[7]),
        or(a[8], b[8]),
        or(a[9], b[9]),
        or(a[10], b[10]),
        or(a[11], b[11]),
        or(a[12], b[12]),
        or(a[13], b[13]),
        or(a[14], b[14]),
        or(a[15], b[15]),
    ]
}

#[rustfmt::skip]
pub fn or8way(a: [bool; 8]) -> bool {
    or(or(or(or(or(or(or(a[0], a[1]), a[2]), a[3]), a[4]), a[5]), a[6]), a[7])
}

pub fn xor(a: bool, b: bool) -> bool {
    and(nand(a, b), or(a, b))
}

pub fn mux(a: bool, b: bool, sel: bool) -> bool {
    or(and(sel, b), and(not(sel), a))
}

pub fn mux16(a: [bool; 16], b: [bool; 16], sel: bool) -> [bool; 16] {
    [
        mux(a[0], b[0], sel),
        mux(a[1], b[1], sel),
        mux(a[2], b[2], sel),
        mux(a[3], b[3], sel),
        mux(a[4], b[4], sel),
        mux(a[5], b[5], sel),
        mux(a[6], b[6], sel),
        mux(a[7], b[7], sel),
        mux(a[8], b[8], sel),
        mux(a[9], b[9], sel),
        mux(a[10], b[10], sel),
        mux(a[11], b[11], sel),
        mux(a[12], b[12], sel),
        mux(a[13], b[13], sel),
        mux(a[14], b[14], sel),
        mux(a[15], b[15], sel),
    ]
}

#[rustfmt::skip]
pub fn mux4way16(a: [bool; 16], b: [bool; 16], c: [bool; 16], d: [bool; 16], sel: [bool; 2]) -> [bool; 16] {
    mux16(mux16(a, b, sel[0]), mux16(c, d, sel[0]), sel[1])
}

#[rustfmt::skip]
pub fn mux8way16(a: [bool; 16], b: [bool; 16], c: [bool; 16], d: [bool; 16], e: [bool; 16], f: [bool; 16], g: [bool; 16], h: [bool; 16], sel: [bool; 3]) -> [bool; 16] {
    mux16(mux4way16(a, b, c, d, [sel[0], sel[1]]), mux4way16(e, f, g, h, [sel[0], sel[1]]), sel[2])
}

pub fn dmux(input: bool, sel: bool) -> [bool; 2] {
    [and(input, not(sel)), and(input, sel)]
}

//pub fn dmux4way(input: bool, sel: [bool; 2]) -> [bool; 4] {
//  []
//}
