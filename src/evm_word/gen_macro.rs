

macro_rules! gen_uword {
    (
     $struct_name:ident, // Struct name
     $struct_type:ty, // Struct main data type
     $word_len:expr, // Amount of words in struct
     $zero_val:expr, // Zero value for struct word type
     $usable_bit_len:expr, // Usable bit length - should be half of struct_type
     $word_len_bytes:expr // Amount of bytes in generated word struct usize
    ) => {
        #[derive(Copy, Clone, Debug)]
        pub struct $struct_name {
            data: [$struct_type; $word_len]
        }

        impl $struct_name {
            pub fn from_bytes(bytes: [u8; $word_len_bytes]) -> Self {
                let mut data = [$zero_val; $word_len];
                for word_num in 0..$word_len {
                    let mut val: $struct_type = 0;
                    // for now we assume that we use only half of word
                    // which is pretty bad and should be optimized
                    let bytes_per_word = $word_len_bytes / $word_len;
                    let range = Range { start: 0, end: bytes_per_word }; 
                    for byte_num in range.into_iter().rev() {
                        val += (bytes[(word_num * bytes_per_word) + byte_num] as $struct_type) << (((bytes_per_word - byte_num - 1) * 8) as $struct_type);
                    }
                    data[word_num] = val;
                }
                $struct_name {
                    data
                }
            }
            
            pub fn bit_mask() -> $struct_type {
                (1 << $usable_bit_len) - 1
            }

            pub fn to_bytes(self) -> [u8; $word_len_bytes] {
                let mut bytes = [0u8; $word_len_bytes];
                let bytes_per_word = $word_len_bytes / $word_len;
                for word_num in 0..$word_len {
                    let be_bytes: &[u8] = &self.data[word_num].to_be_bytes()[$usable_bit_len as usize / 8..];
                    for byte_num in 0..bytes_per_word {
                        bytes[word_num * bytes_per_word + byte_num] = be_bytes[byte_num];
                    }
                }
                bytes
            }

            pub fn zero() -> Self {
                $struct_name {
                    data: [$zero_val; $word_len]
                }
            }

            pub fn one() -> Self {
                let mut data = [$zero_val; $word_len];
                data[data.len() - 1] = 1;
                $struct_name {
                    data
                }
            }

            pub fn is_zero(&self) -> bool {
                let mut is_z = true;
                let mut curr_word = 0;
                while is_z && curr_word < $word_len {
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
                    *word = *word ^ $struct_name::bit_mask();
                }
            }
        }

        impl PartialEq for $struct_name {
            fn eq(&self, other: &$struct_name) -> bool {
                for (idx, word) in self.data.iter().enumerate() {
                    if *word != other.data[idx] {
                        return false;
                    }
                }
                true
            }
        }

        impl PartialOrd for $struct_name {
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

        impl AddAssign<&Self> for $struct_name {
            fn add_assign(&mut self, rhs: &Self) {
                let items = self.data.iter_mut().zip(rhs.data.iter());
                let mut carry = 0;
                for (a, b) in items.rev() {
                    let r = *a + *b + carry;
                    carry = r >> $usable_bit_len;
                    *a = r & $struct_name::bit_mask();

                }
            }
        }

        impl Add<&Self> for $struct_name {
            type Output = $struct_name;
            fn add(mut self, rhs: &Self) -> Self::Output {
                self += rhs;
                self
            }
        }

        impl Add for $struct_name {
            type Output = $struct_name;
            fn add(self, rhs: Self) -> Self::Output {
                self + &rhs
            }
        }

        impl SubAssign<&Self> for $struct_name {
            fn sub_assign(&mut self, rhs: &Self) {
                self.twos_compliment();
                *self += rhs;
                self.twos_compliment();
            }
        }

        impl Sub<&Self> for $struct_name {
            type Output = $struct_name;
            fn sub(mut self, rhs: &Self) -> Self::Output {
                self -= rhs;
                self
            }
        }

        impl Sub for $struct_name {
            type Output = $struct_name;
            fn sub(self, rhs: Self) -> Self::Output {
                self - &rhs
            }
        }

        impl Shr<usize> for $struct_name {
            type Output = $struct_name;
            fn shr(mut self, rhs: usize) -> Self::Output {
                self >>= rhs;
                self
            }
        }

        impl ShrAssign<usize> for $struct_name {
            fn shr_assign(&mut self, rhs: usize) {
                for word in self.data.iter_mut() {
                    *word = *word >> rhs as $struct_type;
                }
            }
        }

        impl Shl<usize> for $struct_name {
            type Output = $struct_name;
            fn shl(mut self, rhs: usize) -> Self::Output {
                self <<= rhs;
                self
            }
        }

        impl ShlAssign<usize> for $struct_name {
            fn shl_assign(&mut self, rhs: usize) {
                for word in self.data.iter_mut() {
                    *word = *word << rhs as $struct_type;
                }
            }
        }
    }
}

macro_rules! gen_sword {
    (
     $struct_name:ident, // Struct name
     $struct_type:ty // Struct main data type
    ) => {
        #[derive(PartialEq, Copy, Clone, Debug)]
        pub struct $struct_name {
            word: $struct_type,
            negative: bool
        }
        
        impl $struct_name {
            pub fn from_word(word: $struct_type, positive: bool) -> Self {
                $struct_name {
                    word,
                    negative: !positive
                }
            }
        
            pub fn to_abs_word(self) -> (bool, $struct_type) {
                (self.negative, self.word)
            }
        
            pub fn one() -> Self {
                $struct_name {
                    word: <$struct_type>::one(),
                    negative: false
                }
            }
        
            pub fn zero() -> Self {
                $struct_name {
                    word: <$struct_type>::zero(),
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

            pub fn egcd(
                mut x: $struct_name,
                mut y: $struct_name
            ) -> ($struct_name, $struct_name, $struct_name) {
                let mut g = 0;
                while x.is_even() && y.is_even() {
                    x >>= 1;
                    y >>= 1;
                    g += 1;
                }

                let mut a = $struct_name::one();
                let mut b = $struct_name::zero();
                let mut c = $struct_name::zero();
                let mut d = $struct_name::one();

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
        }
        
        impl From<$struct_type> for $struct_name {
            fn from(word: $struct_type) -> Self { $struct_name::from_word(word, true) }
        }
        
        impl Into<$struct_type> for $struct_name {
            fn into(self) -> $struct_type {
                self.word
            }
        }
        
        impl PartialOrd for $struct_name {
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
        
        impl AddAssign<&Self> for $struct_name {
            fn add_assign(&mut self, rhs: &Self) {
                match (self.negative, rhs.negative) {
                    (true, false) => {
                        if self.word <= rhs.word {
                            self.negative = false;
                            self.word.twos_compliment();
                            self.word += &rhs.word;
                            self.word += &<$struct_type>::one();
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
                            self.word += &<$struct_type>::one();
                        }
                    },
                    (_, _) => { self.word += &rhs.word; }
                }
            }
        }
        
        impl Add<&Self> for $struct_name {
            type Output = $struct_name;
            fn add(mut self, rhs: &Self) -> Self::Output {
                self += rhs;
                self
            }
        }
        
        impl Add for $struct_name {
            type Output = $struct_name;
            fn add(self, rhs: Self) -> Self::Output {
                self + &rhs
            }
        }
        
        impl SubAssign<&Self> for $struct_name {
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
                            self.word -= &<$struct_type>::one();
                            self.word.twos_compliment();
                        } else {
                            self.word -= &rhs.word;
                        }
                    },
                    (false, false) => {
                        if rhs.word > self.word {
                            self.negative = true;
                            self.word -= &rhs.word;
                            self.word -= &<$struct_type>::one();
                            self.word.twos_compliment();
                        } else {
                            self.word -= &rhs.word;
                        }
                    }
                }
            }
        }
        
        impl Sub<&Self> for $struct_name {
            type Output = $struct_name;
            fn sub(mut self, rhs: &Self) -> Self::Output {
                self -= rhs;
                self
            }
        }
        
        impl Sub for $struct_name {
            type Output = $struct_name;
            fn sub(self, rhs: Self) -> Self::Output {
                self - &rhs
            }
        }
        
        impl Shr<usize> for $struct_name {
            type Output = $struct_name;
            fn shr(mut self, rhs: usize) -> Self::Output {
                self >>= rhs;
                self
            }
        }
        
        impl ShrAssign<usize> for $struct_name {
            fn shr_assign(&mut self, rhs: usize) {
                self.word >>= rhs;
            }
        }
        
        impl Shl<usize> for $struct_name {
            type Output = $struct_name;
            fn shl(mut self, rhs: usize) -> Self::Output {
                self <<= rhs;
                self
            }
        }
        
        impl ShlAssign<usize> for $struct_name {
            fn shl_assign(&mut self, rhs: usize) {
                self.word <<= rhs;
            }
        }
    }
}

#[macro_export]
macro_rules! gen_word {
    (
     $struct_name:ident, // Struct name
     $struct_type:ty, // Struct main data type
     $word_len:expr, // Amount of words in struct
     $zero_val:expr, // Zero value for struct word type
     $usable_bit_len:expr, // Usable bit length - should be half of struct_type
     $word_len_bytes:expr,
     $signed_struct_name:ident // name of signed struct
    ) => {
        use core::{
            ops::{
                Add, Sub, AddAssign, SubAssign, Shr, ShrAssign,
                Shl, ShlAssign, Range
            },
            cmp::{PartialEq, PartialOrd, Ordering},
            marker::{Copy},
            clone::Clone,
            iter::{IntoIterator}
        };
        gen_uword!($struct_name, $struct_type, $word_len, $zero_val, $usable_bit_len, $word_len_bytes);
        gen_sword!($signed_struct_name, $struct_name);
    }
}