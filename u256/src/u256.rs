use core::{
    ops::{
        Add, Sub, AddAssign, SubAssign, Shr, ShrAssign, Div, DivAssign,
        Shl, ShlAssign, Range
    },
    cmp::{PartialEq, PartialOrd, Ordering},
    clone::Clone,
    iter::{IntoIterator}
};
use alloc::{collections::VecDeque};
use crate::s256::S256;

const WORD_LENGTH: usize = 8;
const USABLE_BIT_LENGTH: u32 = 32;
const BIT_MASK: u32 = ((1u64 << USABLE_BIT_LENGTH as u64) - 1) as u32;
const BYTES_PER_WORD: usize = 4;
const BITS_IN_BYTE: usize = 8;
const BYTES_WORD_LENGTH: usize = 32;

#[derive(Clone, Debug)]
pub struct U256 {
    data: VecDeque<u32>
}

impl U256 {
    pub fn from_bytes(bytes: [u8; BYTES_WORD_LENGTH]) -> Self {
        let mut data = VecDeque::with_capacity(WORD_LENGTH);
        for word_num in 0..WORD_LENGTH {
            let mut val: u32 = 0;
            for byte_num in (Range { start: 0, end: BYTES_PER_WORD }).into_iter().rev() {
                val += (bytes[(word_num * BYTES_PER_WORD) + byte_num] as u32) << (((BYTES_PER_WORD - byte_num - 1) * BITS_IN_BYTE) as u32);
            }
            data.push_back(val);
        }
        U256 {
            data
        }
    }

    pub fn to_bytes(&self) -> [u8; BYTES_WORD_LENGTH] {
        let mut bytes = [0u8; BYTES_WORD_LENGTH];
        for word_num in 0..WORD_LENGTH {
            let be_bytes: &[u8] = &self.data[word_num].to_be_bytes()[..];
            for byte_num in 0..BYTES_PER_WORD {
                bytes[word_num * BYTES_PER_WORD + byte_num] = be_bytes[byte_num];
            }
        }
        bytes
    }

    pub fn zero() -> Self {
        let mut data = VecDeque::with_capacity(WORD_LENGTH);
        for _ in 0..WORD_LENGTH {
            data.push_back(0u32);
        }
        U256 {
            data
        }
    }

    pub fn one() -> Self {
        let mut data = VecDeque::with_capacity(WORD_LENGTH);
        for i in 0..WORD_LENGTH {
            if i == WORD_LENGTH - 1 {
                data.push_back(1);
            } else {
                data.push_back(0u32);
            }
        }
        U256 {
            data
        }
    }

    pub fn is_zero(&self) -> bool {
        let mut is_z = true;
        let mut curr_word = 0;
        while is_z && curr_word < WORD_LENGTH {
            if self.data[curr_word] != 0 {
                is_z = false;
                break;
            }
            curr_word+= 1;
        }
        is_z
    }

    pub fn is_odd(&self) -> bool {
        !self.is_even()
    }

    pub fn is_even(&self) -> bool {
        (self.data[self.data.len() - 1] & 1) != 1
    }

    pub fn twos_compliment(&mut self) {
        for word in self.data.iter_mut() {
            *word = *word ^ BIT_MASK;
        }
    }

    pub fn mult_inverse(&self) -> Option<Self> {
        let bit_length = WORD_LENGTH * USABLE_BIT_LENGTH as usize;
        let mut ext_one = Self::one();
        ext_one.data.push_front(0);
        let mut ext_zero = Self::zero();
        ext_zero.data.push_front(0);
        let mut mask = ext_one.clone();
        mask <<= bit_length;
        let mut copy = self.clone();
        copy.data.push_front(0);
        let (_, b, v) = S256::egcd(
            mask.clone().into(),
            copy.into(),
        );
        let res = match v == ext_one.into() {
            true if b > ext_zero.into() => {
                let (_, res) = b.to_abs_word();
                Some(res)
            },
            true => {
                let signed_mask: S256 = mask.into();
                let (_, res) = (b + signed_mask).to_abs_word();
                Some(res)
            },
            false => None
        };
        match res {
            Some(mut word) => {
                word.data.pop_front();
                Some(word)
            },
            x => x,
        }
    }
}

impl From<bool> for U256 {
    fn from(val: bool) -> Self {
        match val {
            true => U256::one(),
            false => U256::zero()
        }
    }
}

impl PartialEq for U256 {
    fn eq(&self, other: &U256) -> bool {
        for (idx, word) in self.data.iter().enumerate() {
            if *word != other.data[idx] {
                return false;
            }
        }
        true
    }
}

impl PartialOrd for U256 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for (idx, word) in self.data.iter().enumerate() {
            match word.partial_cmp(&other.data[idx]) {
                Some(Ordering::Equal) => {},
                x => { return x }
            }
        }
        Some(Ordering::Equal)
    }
}

impl AddAssign<&Self> for U256 {
    fn add_assign(&mut self, rhs: &Self) {
        let items = self.data.iter_mut().zip(rhs.data.iter());
        let mut carry = 0;
        for (a, b) in items.rev() {
            let r = *a as u64 + *b as u64 + carry;
            carry = r >> USABLE_BIT_LENGTH as u64;
            *a = (r & BIT_MASK as u64) as u32;

        }
    }
}

impl Add<&Self> for U256 {
    type Output = U256;
    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Add for U256 {
    type Output = U256;
    fn add(self, rhs: Self) -> Self::Output {
        self + &rhs
    }
}

impl SubAssign<&Self> for U256 {
    fn sub_assign(&mut self, rhs: &Self) {
        self.twos_compliment();
        *self += rhs;
        self.twos_compliment();
    }
}

impl Sub<&Self> for U256 {
    type Output = U256;
    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Sub for U256 {
    type Output = U256;
    fn sub(self, rhs: Self) -> Self::Output {
        self - &rhs
    }
}

/*
* The assumption here is that rhs is not more than 32 bits.
* For u256 base arithmetic it means that if value is gt 256 the result
* of such computation would be 0 anyway so there is no need to take the rest
* of the word into account.
*/
impl ShrAssign for U256 {
    fn shr_assign(&mut self, rhs: Self) {
        let max_shift = USABLE_BIT_LENGTH * WORD_LENGTH;
        let mut shift = 0;
        for (idx, item) in rhs.data.iter().enumerate() {
            if idx < rhs.data.len() - 1 && item != 0 {
                shift = max_shift;
                break;
            } else if idx == rhs.data.len() - 1 {
                shift = match *item < max_shift {
                    true => *item,
                    false => max_shift
                };
            }
        }
        self >>= shift;
    }
}

impl Shr for U256 {
    type Output = U256;
    fn shr(mut self, rhs: Self) -> Self::Output {
        self >>= rhs;
        self
    }
}

impl Shr<usize> for U256 {
    type Output = U256;
    fn shr(mut self, rhs: usize) -> Self::Output {
        self >>= rhs;
        self
    }
}

impl ShrAssign<usize> for U256 {
    fn shr_assign(&mut self, rhs: usize) {
        let r = rhs as u32 % USABLE_BIT_LENGTH;
        let s = ((rhs as u32 - r) / USABLE_BIT_LENGTH) as usize;

        if s > self.data.len() {
            for word in self.data.iter_mut() {
                *word = 0;
            }
        } else if s != 0 {
            let len = self.data.len();
            for i in (0..len).rev() {
                if s < len - i {
                    self.data[i + s] = self.data[i];
                }
            }

            for v in 0..s {
                self.data[v] = 0;
            }
        }

        if r != 0 && !(s > self.data.len()) {
            let carry_mask = (BIT_MASK ^ ((BIT_MASK >> (USABLE_BIT_LENGTH - r)) << (USABLE_BIT_LENGTH - r))) as u32;
            let mut carry = 0;
            for word in self.data.iter_mut() {
                let word_copy = *word;
                *word = (carry << (USABLE_BIT_LENGTH - r)) | (word_copy >> r);
                carry = word_copy & carry_mask;
            }
        }
    }
}

/*
* The assumption here is that rhs is not more than 32 bits.
* For u256 base arithmetic it means that if value is gt 256 the result
* of such computation would be 0 anyway so there is no need to take the rest
* of the word into account.
*/
impl ShlAssign for U256 {
    fn shl_assign(&mut self, rhs: Self) {
        let max_shift = USABLE_BIT_LENGTH * WORD_LENGTH;
        let mut shift = 0;
        for (idx, item) in rhs.data.iter().enumerate() {
            if idx < rhs.data.len() - 1 && item != 0 {
                shift = max_shift;
                break;
            } else if idx == rhs.data.len() - 1 {
                shift = match *item < max_shift {
                    true => *item,
                    false => max_shift
                };
            }
        }
        self <<= shift;
    }
}

impl Shl for U256 {
    type Output = U256;
    fn shl(mut self, rhs: Self) -> Self::Output {
        self <<= rhs;
        self
    }
}

impl Shl<usize> for U256 {
    type Output = U256;
    fn shl(mut self, rhs: usize) -> Self::Output {
        self <<= rhs;
        self
    }
}

impl ShlAssign<usize> for U256 {
    fn shl_assign(&mut self, rhs: usize) {
        let r = rhs as u32 % USABLE_BIT_LENGTH;
        let s = ((rhs as u32 - r) / USABLE_BIT_LENGTH) as usize;

        if r != 0 {
            let carry_mask = ((BIT_MASK >> (USABLE_BIT_LENGTH - r)) << (USABLE_BIT_LENGTH - r)) as u32;
            let mut carry = 0;
            for word in self.data.iter_mut().rev() {
                let new_carry = *word & carry_mask;
                let c = (*word - new_carry) << r;
                *word = c | carry;
                carry = new_carry >> (USABLE_BIT_LENGTH - r);
            }
        }

        if s != 0 {
            let len = self.data.len();
            for i in 0..len {
                if s < i {
                    self.data[i - s] = self.data[i];
                }
            }

            for v in 0..s {
                self.data[len - 1 - v] = 0;
            }
        }
    }
}

impl DivAssign<&Self> for U256 {
    fn div_assign(&mut self, rhs: &Self) {
        if &*self < rhs {
            *self = U256::zero();
        } else {
            let (_, b, _) = S256::egcd(
                self.clone().into(),
                rhs.clone().into()
            );
            *self = b.into();
        }
    }
}

impl Div<&Self> for U256 {
    type Output = U256;
    fn div(mut self, rhs: &Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl Div for U256 {
    type Output = U256;
    fn div(mut self, rhs: Self) -> Self::Output {
        self /= &rhs;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn from() {
        let mut a = [0u8; 32];
        for i in 0..a.len() {
            a[i] = i as u8;
        }
        let word = U256::from_bytes(a.clone());
        let expected = [66051, 67438087, 134810123, 202182159, 269554195, 336926231, 404298267, 471670303];
        assert_eq!(word.data, expected);
    }

    #[test]
    fn from_to() {
        let mut a = [0u8; 32];
        for i in 0..a.len() {
            a[i] = i as u8;
        }
        let word = U256::from_bytes(a.clone());
        let b = word.to_bytes();
        assert_eq!(a, b);
    }

    #[test]
    fn non_modular_add() {
        let val_one = U256::from_bytes(
            [127, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        let val_two = U256::from_bytes(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        let result = val_one + val_two;
        let expected = [128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 254];
        assert_eq!(result.to_bytes(), expected);
    }

    #[test]
    fn modular_add() {
        let val_one = U256::from_bytes(
            [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        let val_two = U256::from_bytes(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        let result = val_one + val_two;
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 254];
        assert_eq!(result.to_bytes(), expected);
    }

    #[test]
    fn non_modular_sub() {
        let val_one = U256::from_bytes(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        let val_two = U256::from_bytes(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        let result = val_one - val_two;
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(result.to_bytes(), expected);
    }

    #[test]
    fn modular_sub() {
        let val_one = U256::from_bytes(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        let val_two = U256::from_bytes(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        let result = val_two - val_one;
        let expected = [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(result.to_bytes(), expected);
    }

    #[test]
    fn shl_carry() {
        let mut x_slice: [u8; 32] = [0u8; 32];
        let mut exp_slice: [u8; 32] = [0u8; 32];
        let xp = &hex::decode("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()[0..32];
        let exp_p = &hex::decode("fffffffffffffffffffffffffffffffffffffffffffffffffffffff800000000").unwrap()[0..32];
        for i in 0..x_slice.len() {
            x_slice[i] = xp[i];
            exp_slice[i] = exp_p[i];
        }
        let a = U256::from_bytes(x_slice);
        let exp = U256::from_bytes(exp_slice);
        assert_eq!(a << 35, exp);
    }

    #[test]
    fn shr_carry() {
        let mut x_slice: [u8; 32] = [0u8; 32];
        let mut exp_slice: [u8; 32] = [0u8; 32];
        let xp = &hex::decode("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()[0..32];
        let exp_p = &hex::decode("000000000fffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()[0..32];
        for i in 0..x_slice.len() {
            x_slice[i] = xp[i];
            exp_slice[i] = exp_p[i];
        }
        let a = U256::from_bytes(x_slice);
        let exp = U256::from_bytes(exp_slice);
        assert_eq!(a >> 35, exp);
    }

    #[test]
    fn mult_inv() {
        let mut x_slice: [u8; 32] = [0u8; 32];
        let mut exp_slice: [u8; 32] = [0u8; 32];
        let xp = &hex::decode("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()[0..32];
        let exp_p = &hex::decode("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()[0..32];
        for i in 0..x_slice.len() {
            x_slice[i] = xp[i];
            exp_slice[i] = exp_p[i];
        }
        let a = U256::from_bytes(x_slice);
        let exp = U256::from_bytes(exp_slice);
        let res = a.mult_inverse();
        assert_eq!(res, Some(exp));
    }

}