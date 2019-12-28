
#[cfg(test)]
mod tests {
    use hex;
    use crate::evm_word::evm_word::{EVMWord, SignedEvmWord};

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

        let (a, b, v) = SignedEvmWord::egcd(
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