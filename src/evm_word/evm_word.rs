use crate::{gen_word};

gen_word!(EVMWord, u64, 8, 0u64, 32, 32, SignedEvmWord);

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