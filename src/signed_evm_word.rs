use crate::evm_word::EVMWord;
use core::{
    ops::{Add, AddAssign, Sub, SubAssign, Shr, ShrAssign, Shl, ShlAssign},
    cmp::{PartialEq, PartialOrd, Ordering},
    marker::{Copy},
    clone::Clone
};

/*
* Since SignedEvmWord is internal implementation detail meant only for
* some specialised computations which require tracking of number sign
* we do not care to turn the value upside down on sign change so i.e.
* value like (-0x01) would be represented in turn as two's compliment
* of this number (0xff) with negative attribute set to true. This fact
* requires us to have careful implementation of arithmetic operations for
* proper support of cases like (-0xff) - (-0xfe) in this case (-0xff) == -1
* and (-0xfe) == -2 and the result of such operation would be (0x01).
*/
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct SignedEvmWord {
    word: EVMWord,
    negative: bool
}

impl SignedEvmWord {
    pub fn from_word(word: EVMWord, positive: bool) -> Self {
        SignedEvmWord {
            word,
            negative: !positive
        }
    }

    pub fn to_abs_word(self) -> (bool, EVMWord) {
        (self.negative, self.word)
    }

    pub fn one() -> Self {
        SignedEvmWord {
            word: EVMWord::one(),
            negative: false
        }
    }

    pub fn zero() -> Self {
        SignedEvmWord {
            word: EVMWord::zero(),
            negative: false
        }
    }

    pub fn is_zero(&self) -> bool {
        self.word.is_zero()
    }

    pub fn is_odd(&self) -> bool {
        self.word.is_odd()
    }

    pub fn is_even(&self) -> bool {
        self.word.is_even()
    }
}

impl From<EVMWord> for SignedEvmWord {
    fn from(word: EVMWord) -> Self { SignedEvmWord::from_word(word, true) }
}

impl Into<EVMWord> for SignedEvmWord {
    fn into(self) -> EVMWord {
        self.word
    }
}

impl PartialOrd for SignedEvmWord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.negative, other.negative) {
            (true, false) => Some(Ordering::Greater),
            (false, true) => Some(Ordering::Less),
            (false, false) => self.word.partial_cmp(&other.word),
            (true, true) => match self.word.partial_cmp(&other.word) {
                Some(Ordering::Greater) => Some(Ordering::Less),
                Some(Ordering::Less) => Some(Ordering::Greater),
                other => other
            }
        }
    }
}

impl AddAssign<&Self> for SignedEvmWord {
    fn add_assign(&mut self, rhs: &Self) {
        match (self.negative, rhs.negative) {
            (true, false) => {
                if self.word <= rhs.word {
                    self.negative = false;
                    self.word.twos_compliment();
                    self.word += &rhs.word;
                    self.word += &EVMWord::one();
                } else {
                    self.word -= &rhs.word;
                }
            },
            (false, true) => {
                if rhs.word <= self.word {
                    // self.negative = false - already non-negative
                    self.word -= &rhs.word;
                } else {
                    self.negative = true;
                    self.word -= &rhs.word;
                    self.word.twos_compliment();
                    self.word += &EVMWord::one();
                }
            },
            (_, _) => { self.word += &rhs.word; }
        }
    }
}

impl Add<&Self> for SignedEvmWord {
    type Output = SignedEvmWord;
    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Add for SignedEvmWord {
    type Output = SignedEvmWord;
    fn add(self, rhs: Self) -> Self::Output {
        self + &rhs
    }
}

impl SubAssign<&Self> for SignedEvmWord {
    fn sub_assign(&mut self, rhs: &Self) {
        match (self.negative, rhs.negative) {
            (true, false) => {
                self.word += &rhs.word;
            },
            (false, true) => {
                self.word += &rhs.word;
            },
            (true, true) => {
                if rhs.word >= self.word {
                    self.negative = false;
                    self.word -= &rhs.word;
                    self.word -= &EVMWord::one();
                    self.word.twos_compliment();
                } else {
                    self.word -= &rhs.word;
                }
            },
            (false, false) => {
                if rhs.word > self.word {
                    self.negative = true;
                    self.word -= &rhs.word;
                    self.word -= &EVMWord::one();
                    self.word.twos_compliment();
                } else {
                    self.word -= &rhs.word;
                }
            }
        }
    }
}

impl Sub<&Self> for SignedEvmWord {
    type Output = SignedEvmWord;
    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Sub for SignedEvmWord {
    type Output = SignedEvmWord;
    fn sub(self, rhs: Self) -> Self::Output {
        self - &rhs
    }
}

impl Shr<usize> for SignedEvmWord {
    type Output = SignedEvmWord;
    fn shr(mut self, rhs: usize) -> Self::Output {
        self >>= rhs;
        self
    }
}

impl ShrAssign<usize> for SignedEvmWord {
    fn shr_assign(&mut self, rhs: usize) {
        self.word >>= rhs;
    }
}

impl Shl<usize> for SignedEvmWord {
    type Output = SignedEvmWord;
    fn shl(mut self, rhs: usize) -> Self::Output {
        self <<= rhs;
        self
    }
}

impl ShlAssign<usize> for SignedEvmWord {
    fn shl_assign(&mut self, rhs: usize) {
        self.word <<= rhs;
    }
}


pub fn egcd(
    mut x: SignedEvmWord,
    mut y: SignedEvmWord
) -> (SignedEvmWord, SignedEvmWord, SignedEvmWord) {
    let mut g = 0;
    while x.is_even() && y.is_even() {
        x >>= 1;
        y >>= 1;
        g += 1;
    }

    let mut a = SignedEvmWord::one();
    let mut b = SignedEvmWord::zero();
    let mut c = SignedEvmWord::zero();
    let mut d = SignedEvmWord::one();

    let mut u = x.clone();
    let mut v = y.clone();
    while !u.is_zero() {
        while u.is_even() {
            u >>= 1;
            if a.is_odd() || b.is_odd() {
                a = a + &y;
                b = b - &x;
            }
            a >>= 1;
            b >>= 1;
        }

        while v.is_even() {
            v >>= 1;
            if c.is_odd() || d.is_odd() {
                c = c + &y;
                d = d - &x;
            }
            c >>= 1;
            d >>= 1;
        }

        match u.partial_cmp(&v) {
            Some(Ordering::Less) => {
                v -= &u;
                c -= &a;
                d -= &b;
            }
            _ => {
                u -= &v;
                a -= &c;
                b -= &d;
            }
        }
    }

    (c, d, v << g)
}


#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn positive_to_neg_add() {
        let mut x_slice: [u8; 32] = [0u8; 32];
        let mut y_slice: [u8; 32] = [0u8; 32];
        let mut exp_slice: [u8; 32] = [0u8; 32];
        let xp = &hex::decode("00000000000000000000000000000000000000000000000000000000000002b5").unwrap()[0..32];
        let yp = &hex::decode("0000000000000000000000000000000000000000000000000000000000000261").unwrap()[0..32];
        let exp_p = &hex::decode("0000000000000000000000000000000000000000000000000000000000000054").unwrap()[0..32];
        for i in 0..x_slice.len() {
            x_slice[i] = xp[i];
            y_slice[i] = yp[i];
            exp_slice[i] = exp_p[i];
        }
        let a: SignedEvmWord = SignedEvmWord::from_word(
            EVMWord::from_bytes(x_slice),
            true
        );
        let b: SignedEvmWord = SignedEvmWord::from_word(
            EVMWord::from_bytes(y_slice),
            false
        );
        let exp: SignedEvmWord = SignedEvmWord::from_word(
            EVMWord::from_bytes(exp_slice),
            true
        );

        let r = b + a;
        assert_eq!(exp, r);
    }

    #[test]
    fn positive_to_neg_add_rev() {
        let mut x_slice: [u8; 32] = [0u8; 32];
        let mut y_slice: [u8; 32] = [0u8; 32];
        let mut exp_slice: [u8; 32] = [0u8; 32];
        let xp = &hex::decode("0000000000000000000000000000000000000000000000000000000000000261").unwrap()[0..32];
        let yp = &hex::decode("00000000000000000000000000000000000000000000000000000000000002b5").unwrap()[0..32];
        let exp_p = &hex::decode("0000000000000000000000000000000000000000000000000000000000000054").unwrap()[0..32];
        for i in 0..x_slice.len() {
            x_slice[i] = xp[i];
            y_slice[i] = yp[i];
            exp_slice[i] = exp_p[i];
        }
        let a: SignedEvmWord = SignedEvmWord::from_word(
            EVMWord::from_bytes(x_slice),
            true
        );
        let b: SignedEvmWord = SignedEvmWord::from_word(
            EVMWord::from_bytes(y_slice),
            false
        );
        let exp: SignedEvmWord = SignedEvmWord::from_word(
            EVMWord::from_bytes(exp_slice),
            false
        );

        let r = a + b;
        assert_eq!(exp, r);
    }

    #[test]
    fn neg_to_zero_add() {
        let mut x_slice: [u8; 32] = [0u8; 32];
        let mut y_slice: [u8; 32] = [0u8; 32];
        let mut exp_slice: [u8; 32] = [0u8; 32];
        let xp = &hex::decode("00000000000000000000000000000000000000000000000000000000000002b5").unwrap()[0..32];
        let yp = &hex::decode("00000000000000000000000000000000000000000000000000000000000002b5").unwrap()[0..32];
        let exp_p = &hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap()[0..32];
        for i in 0..x_slice.len() {
            x_slice[i] = xp[i];
            y_slice[i] = yp[i];
            exp_slice[i] = exp_p[i];
        }
        let a: SignedEvmWord = SignedEvmWord::from_word(
            EVMWord::from_bytes(x_slice),
            true
        );
        let b: SignedEvmWord = SignedEvmWord::from_word(
            EVMWord::from_bytes(y_slice),
            false
        );
        let exp: SignedEvmWord = SignedEvmWord::from_word(
            EVMWord::from_bytes(exp_slice),
            true
        );

        let r = b + a;
        assert_eq!(exp, r);
    }

    #[test]
    fn positive_only_sub() {
        let mut x_slice: [u8; 32] = [0u8; 32];
        let mut y_slice: [u8; 32] = [0u8; 32];
        let mut exp_slice: [u8; 32] = [0u8; 32];
        let xp = &hex::decode("00000000000000000000000000000000000000000000000000000000000002b5").unwrap()[0..32];
        let yp = &hex::decode("0000000000000000000000000000000000000000000000000000000000000261").unwrap()[0..32];
        let exp_p = &hex::decode("0000000000000000000000000000000000000000000000000000000000000054").unwrap()[0..32];
        for i in 0..x_slice.len() {
            x_slice[i] = xp[i];
            y_slice[i] = yp[i];
            exp_slice[i] = exp_p[i];
        }
        let a: SignedEvmWord = EVMWord::from_bytes(x_slice).into();
        let b: SignedEvmWord = EVMWord::from_bytes(y_slice).into();
        let exp: SignedEvmWord = EVMWord::from_bytes(exp_slice).into();

        let r = a - b;
        assert_eq!(exp, r);
    }

    #[test]
    fn positive_to_neg_sub() {
        let mut x_slice: [u8; 32] = [0u8; 32];
        let mut y_slice: [u8; 32] = [0u8; 32];
        let mut exp_slice: [u8; 32] = [0u8; 32];
        let xp = &hex::decode("00000000000000000000000000000000000000000000000000000000000002b5").unwrap()[0..32];
        let yp = &hex::decode("0000000000000000000000000000000000000000000000000000000000000261").unwrap()[0..32];
        let exp_p = &hex::decode("0000000000000000000000000000000000000000000000000000000000000054").unwrap()[0..32];
        for i in 0..x_slice.len() {
            x_slice[i] = xp[i];
            y_slice[i] = yp[i];
            exp_slice[i] = exp_p[i];
        }
        let a: SignedEvmWord = EVMWord::from_bytes(x_slice).into();
        let b: SignedEvmWord = EVMWord::from_bytes(y_slice).into();
        let exp: SignedEvmWord = SignedEvmWord::from_word(
            EVMWord::from_bytes(exp_slice),
            false
        );

        let r = b - a;
        assert_eq!(exp, r);
    }

    #[test]
    fn neg_to_pos_sub() {
        let mut x_slice: [u8; 32] = [0u8; 32];
        let mut y_slice: [u8; 32] = [0u8; 32];
        let mut exp_slice: [u8; 32] = [0u8; 32];
        let xp = &hex::decode("8000000000000000000000000000000000000000000000000000000000000000").unwrap()[0..32];
        let yp = &hex::decode("80000000000000000000000000000000000000000000000000000000000000ff").unwrap()[0..32];
        let exp_p = &hex::decode("00000000000000000000000000000000000000000000000000000000000000ff").unwrap()[0..32];
        for i in 0..x_slice.len() {
            x_slice[i] = xp[i];
            y_slice[i] = yp[i];
            exp_slice[i] = exp_p[i];
        }
        let a: SignedEvmWord = SignedEvmWord::from_word(
            EVMWord::from_bytes(x_slice),
            false
        );
        let b: SignedEvmWord = SignedEvmWord::from_word(
            EVMWord::from_bytes(y_slice),
            false
        );
        let exp: SignedEvmWord = SignedEvmWord::from_word(
            EVMWord::from_bytes(exp_slice),
            true
        );

        let r = a - b;
        assert_eq!(exp, r);
    }

    #[test]
    fn test_egcd() {
        let mut x: [u8; 32] = [0u8; 32];
        let mut y: [u8; 32] = [0u8; 32];
        let xp = &hex::decode("00000000000000000000000000000000000000000000000000000000000002b5").unwrap()[0..32];
        let yp = &hex::decode("0000000000000000000000000000000000000000000000000000000000000261").unwrap()[0..32];
        for (idx, (p, k)) in (x.iter_mut().zip(y.iter_mut())).enumerate() {
            *p = xp[idx];
            *k = yp[idx];
        }

        let (a, b, v) = egcd(
            EVMWord::from_bytes(x).into(),
            EVMWord::from_bytes(y).into(),
        );
        let vp_expected = &hex::decode("0000000000000000000000000000000000000000000000000000000000000015").unwrap()[0..32];
        let ap_expected = &hex::decode("00000000000000000000000000000000000000000000000000000000000000b5").unwrap()[0..32];
        let bp_expected = &hex::decode("00000000000000000000000000000000000000000000000000000000000000ce").unwrap()[0..32];
        let mut v_exp: [u8; 32] = [0u8; 32];
        let mut a_exp: [u8; 32] = [0u8; 32];
        let mut b_exp: [u8; 32] = [0u8; 32];
        for i in 0..v_exp.len() {
            v_exp[i] = vp_expected[i];
            a_exp[i] = ap_expected[i];
            b_exp[i] = bp_expected[i];
        }
        let v_word = EVMWord::from_bytes(v_exp);
        let b_word = EVMWord::from_bytes(b_exp);
        let a_word = EVMWord::from_bytes(a_exp);
        assert_eq!(v_word, v.into());
        assert_eq!(b_word, b.into());
        assert_eq!(a_word, a.into());
    }
}