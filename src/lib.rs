extern crate byteorder;

use byteorder::{ByteOrder, LittleEndian};

fn buffer() -> [u8; 8] { [0u8; 8] }

pub fn db(n: f64) -> [u32; 2] {
    let mut buf = buffer();
    LittleEndian::write_f64(&mut buf, n);
    [
        LittleEndian::read_u32(&buf[0..4]),
        LittleEndian::read_u32(&buf[4..8])
    ]
}

pub fn pack(lo: u32, hi: u32) -> f64 {
    let mut buf = buffer();
    LittleEndian::write_u32(&mut buf[0..4], lo);
    LittleEndian::write_u32(&mut buf[4..8], hi);
    LittleEndian::read_f64(&buf)
}

pub fn lo(n: f64) -> u32 {
    let mut buf = buffer();
    LittleEndian::write_f64(&mut buf, n);
    LittleEndian::read_u32(&buf[0..4])
}

pub fn hi(n: f64) -> u32 {
    let mut buf = buffer();
    LittleEndian::write_f64(&mut buf, n);
    LittleEndian::read_u32(&buf[4..8])
}

pub fn sign(n: f64) -> u32 {
    hi(n) >> 31
}

pub fn exponent(n: f64) -> i32 {
    let b = hi(n);
    (((b << 1) >> 21) as i32) - 1023
}

pub fn fraction(n: f64) -> [u32; 2] {
    let l = lo(n);
    let h = hi(n);
    let mut b = h & ((1 << 20) - 1);
    if (h & 0x7ff00000) != 0 {
        b += 1 << 20
    }
    [l, b]
}

pub fn denormalized(n: f64) -> bool {
    let h = hi(n);
    (h & 0x7ff00000) == 0
}

#[cfg(test)]
mod double_bits_test {
    #[test]
    fn test_double_bits() {
        use super::{db, pack, lo, hi, sign, fraction, denormalized, exponent};
        assert_eq!(lo(1.0), 0);
        assert_eq!(hi(1.0), 0x3ff00000);
        assert_eq!(pack(0, 0x3ff00000), 1.0);
        assert_eq!(db(1.0), [0, 0x3ff00000]);

        assert_eq!(fraction(1.), [0, 1 << 20]);
        assert_eq!(exponent(1.), 0);
        assert_eq!(sign(1.), 0);
        assert_eq!(sign(-1.), 1);
        assert_eq!(exponent(0.5), -1);

        assert!(denormalized(2f64.powi(-1024)));
        assert!(!denormalized(1.));
        assert!(denormalized(2f64.powi(-1023)));
        assert!(!denormalized(2f64.powi(-1022)));
    }
}