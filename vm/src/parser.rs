use alloc::vec::Vec;
use crate::vm_error::{VmResult, VmError};
use crate::opcode::Opcode;
use alloc::string::String;
use hex;

pub struct OpcodeParser {
    bytecode: Vec<u8>,
    pc: usize,
}

impl OpcodeParser {
    pub fn new(bytecode: Vec<u8>) -> Self {
        OpcodeParser {
            bytecode,
            pc: 0
        }
    }

    pub fn parse(mut self) -> VmResult<String> {
        let mut bytecode_vec: Vec<String> = Vec::new();
        loop {
            let pc = self.pc;
            self.pc += 1;
            let opcode = match pc >= self.bytecode.len() {
                false => match Opcode::from_u8(self.bytecode[pc]) {
                    Some(c) => Ok(c),
                    None => Err(
                        VmError::InvalidOpCode(
                            String::from("Invalid opcode")
                        )
                    )
                },
                true => { break; }
            }?;
            match opcode {
                push_like if push_like >= Opcode::PUSH1 && push_like <= Opcode::PUSH32 => {
                    let push_amt = (push_like as u8 - Opcode::PUSH1 as u8 + 1) as usize;
                    bytecode_vec.push(String::from(push_like.to_str()));
                    let mut zrx = String::from("0x");
                    zrx.push_str(&hex::encode(&self.bytecode[self.pc..(self.pc + push_amt)]));
                    bytecode_vec.push(zrx);
                    self.pc += push_amt;
                },
                op => {
                    bytecode_vec.push(String::from(op.to_str()));
                }
            }
        }
        let bytecode_len = bytecode_vec.len();
        Ok(
            bytecode_vec
                .iter()
                .enumerate()
                .fold(
                    String::from(""),
                    |
                        mut acc,
                        (
                            idx,
                            item
                        )
                    | {
                        acc.push_str(item);
                        if idx < bytecode_len - 1 {
                            acc.push_str(" ");
                        }
                        acc
                    }
                )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    #[test]
    fn opcode_parser() -> VmResult<()> {
        let bytecode = vec![0x60, 25, 0x60, 26, 0x02, 0x60, 0x0, 0x52, 0x60, 0x20, 0x60, 0x0, 0xf3];
        let parser = OpcodeParser::new(bytecode);
        let result = parser.parse()?;
        let expected = String::from("PUSH1 0x19 PUSH1 0x1a MUL PUSH1 0x00 MSTORE PUSH1 0x20 PUSH1 0x00 RETURN");
        assert_eq!(result, expected);
        Ok(())
    }
}