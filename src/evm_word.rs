use core::{
    ops::{
        Add, Sub, AddAssign, SubAssign, Shr, ShrAssign,
        Shl, ShlAssign, Range
    },
    cmp::{PartialEq, PartialOrd, Ordering},
    clone::Clone,
    iter::{IntoIterator}
};
use alloc::{collections::VecDeque};

const WORD_LENGTH: usize = 8;
const USABLE_BIT_LENGTH: u32 = 32;
const BIT_MASK: u32 = ((1u64 << USABLE_BIT_LENGTH as u64) - 1) as u32;
const BYTES_PER_WORD: usize = 4;
const BITS_IN_BYTE: usize = 8;
const BYTES_WORD_LENGTH: usize = 32;

#[derive(Clone, Debug)]
pub struct EVMWord {
    data: VecDeque<u32>
}

impl EVMWord {
    pub fn from_bytes(bytes: [u8; BYTES_WORD_LENGTH]) -> Self {
        let mut data = VecDeque::with_capacity(WORD_LENGTH);
        for word_num in 0..WORD_LENGTH {
            let mut val: u32 = 0;
            for byte_num in (Range { start: 0, end: BYTES_PER_WORD }).into_iter().rev() {
                val += (bytes[(word_num * BYTES_PER_WORD) + byte_num] as u32) << (((BYTES_PER_WORD - byte_num - 1) * BITS_IN_BYTE) as u32);
            }
            data.push_back(val);
        }
        EVMWord {
            data
        }
    }

    pub fn to_bytes(self) -> [u8; BYTES_WORD_LENGTH] {
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
        EVMWord {
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
        EVMWord {
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
}

impl PartialEq for EVMWord {
    fn eq(&self, other: &EVMWord) -> bool {
        for (idx, word) in self.data.iter().enumerate() {
            if *word != other.data[idx] {
                return false;
            }
        }
        true
    }
}

impl PartialOrd for EVMWord {
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

impl AddAssign<&Self> for EVMWord {
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

impl Add<&Self> for EVMWord {
    type Output = EVMWord;
    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Add for EVMWord {
    type Output = EVMWord;
    fn add(self, rhs: Self) -> Self::Output {
        self + &rhs
    }
}

impl SubAssign<&Self> for EVMWord {
    fn sub_assign(&mut self, rhs: &Self) {
        self.twos_compliment();
        *self += rhs;
        self.twos_compliment();
    }
}

impl Sub<&Self> for EVMWord {
    type Output = EVMWord;
    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Sub for EVMWord {
    type Output = EVMWord;
    fn sub(self, rhs: Self) -> Self::Output {
        self - &rhs
    }
}

impl Shr<usize> for EVMWord {
    type Output = EVMWord;
    fn shr(mut self, rhs: usize) -> Self::Output {
        self >>= rhs;
        self
    }
}

impl ShrAssign<usize> for EVMWord {
    fn shr_assign(&mut self, rhs: usize) {
        for word in self.data.iter_mut() {
            *word = *word >> rhs as u32;
        }
    }
}

impl Shl<usize> for EVMWord {
    type Output = EVMWord;
    fn shl(mut self, rhs: usize) -> Self::Output {
        self <<= rhs;
        self
    }
}

impl ShlAssign<usize> for EVMWord {
    fn shl_assign(&mut self, rhs: usize) {
        for word in self.data.iter_mut() {
            *word = *word << rhs as u32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        let mut a = [0u8; 32];
        for i in 0..a.len() {
            a[i] = i as u8;
        }
        let word = EVMWord::from_bytes(a.clone());
        let expected = [66051, 67438087, 134810123, 202182159, 269554195, 336926231, 404298267, 471670303];
        assert_eq!(word.data, expected);
    }

    #[test]
    fn from_to() {
        let mut a = [0u8; 32];
        for i in 0..a.len() {
            a[i] = i as u8;
        }
        let word = EVMWord::from_bytes(a.clone());
        let b = word.to_bytes();
        assert_eq!(a, b);
    }

    #[test]
    fn non_modular_add() {
        let val_one = EVMWord::from_bytes(
            [127, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        let val_two = EVMWord::from_bytes(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        let result = val_one + val_two;
        let expected = [128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 254];
        assert_eq!(result.to_bytes(), expected);
    }

    #[test]
    fn modular_add() {
        let val_one = EVMWord::from_bytes(
            [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        let val_two = EVMWord::from_bytes(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        let result = val_one + val_two;
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 254];
        assert_eq!(result.to_bytes(), expected);
    }

    #[test]
    fn non_modular_sub() {
        let val_one = EVMWord::from_bytes(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        let val_two = EVMWord::from_bytes(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        let result = val_one - val_two;
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(result.to_bytes(), expected);
    }

    #[test]
    fn modular_sub() {
        let val_one = EVMWord::from_bytes(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        let val_two = EVMWord::from_bytes(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
        let result = val_two - val_one;
        let expected = [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(result.to_bytes(), expected);
    }

}