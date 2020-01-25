use alloc::{string::String, vec::Vec, boxed::Box};
use crate::vm_error::{VmResult, VmError};
use u256::u256::{U256bytes, U256};
use crate::opcode::Opcode;
use crate::eei_common::EEI;
use crate::stack::EVMStack;
use crate::memory::{WMemory, EVMMemory};
cfg_if::cfg_if! {
    if #[cfg(target = "wasm32-unknown-unknown")] {
        use crate::eei::{debug};
    }
}

pub struct Interpreter {
    pub stack: EVMStack<U256>,
    pub memory: Box<dyn WMemory<U256>>,
    pub wasm_mem: Box<dyn WMemory<U256>>,
    pub bytecode: Vec<u8>,
    pub eei: Box<dyn EEI>,
    pub pc: usize
}

impl Interpreter {
    pub fn new(bytecode: Vec<u8>, eei: Box<dyn EEI>) -> Self {
        Interpreter {
            stack: EVMStack::new(),
            memory: Box::new(EVMMemory::new()),
            wasm_mem: Box::new(EVMMemory::new()),
            pc: 0,
            bytecode,
            eei
        }
    }

    pub fn execute(&mut self) -> VmResult<()> {
        loop {
            match self.step() {
                Err(e) => { return Err(e); },
                _ => {}
            }
        }
    }

    fn step(&mut self) -> VmResult<()> {
        let pc = self.pc;
        self.pc += 1;

        #[cfg(target = "wasm32-unknown-unknown")]
        debug::log_debug_local(pc as i32);

        let opcode = match pc >= self.bytecode.len() {
            false => match Opcode::from_u8(self.bytecode[pc]) {
                Some(c) => Ok(c),
                None => Err(
                    VmError::InvalidOpCode(
                        String::from("Invalid opcode")
                    )
                )
            },
            true => Err(
                VmError::InvalidOpCode(
                    String::from("Invalid opcode")
                )
            )
        }?;
        match opcode {
            Opcode::STOP => Err(VmError::Stop(String::from("stop"))),
            Opcode::ADD => self.add(),
            Opcode::SUB => self.sub(),
            Opcode::MUL => self.mul(),
            Opcode::DIV => self.div(),
            Opcode::LT => self.lt(),
            Opcode::GT => self.gt(),
            Opcode::EQ => self.eq(),
            Opcode::MOD => self.modulo(),
            Opcode::ADDMOD => self.addmod(),
            Opcode::MULMOD => self.mulmod(),
            Opcode::ISZERO => self.is_zero(),
            Opcode::NOT => self.not(),
            Opcode::OR => self.or(),
            Opcode::XOR => self.xor(),
            Opcode::AND => self.and(),
            Opcode::SHL => self.shl(),
            Opcode::SHR => self.shr(),
            Opcode::POP => self.pop(),
            Opcode::MLOAD => self.mload(),
            Opcode::MSTORE => self.mstore(),
            Opcode::MSTORE8 => self.mstore8(),
            Opcode::MSIZE => self.msize(),
            Opcode::PC => self.pc(),
            Opcode::JUMP => self.jump(),
            Opcode::JUMPI => self.jumpi(),
            Opcode::ADDRESS => self.address(),
            Opcode::JUMPDEST => Ok(()),
            Opcode::SLOAD => self.sload(),
            Opcode::SSTORE => self.sstore(),
            Opcode::RETURN => self.ret(),
            Opcode::REVERT => self.revert(),
            push_like if push_like >= Opcode::PUSH1 && push_like <= Opcode::PUSH32 => {
                let push_amt = (push_like as u8 - Opcode::PUSH1 as u8 + 1) as usize;
                self.push(push_amt)
            },
            dup_like if dup_like >= Opcode::DUP1 && dup_like <= Opcode::DUP16 => {
                let dup_pos = (dup_like as u8 - Opcode::DUP1 as u8 + 1) as usize;
                self.dup(dup_pos)
            },
            swap_like if swap_like >= Opcode::SWAP1 && swap_like <= Opcode::SWAP16 => {
                let swap_pos = (swap_like as u8 - Opcode::SWAP1 as u8 + 1) as usize;
                self.swap(swap_pos)
            },
            _ => {
                unimplemented!();
            }
        }
    }

    fn add(&mut self) -> VmResult<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        self.stack.push(a + b)?;
        Ok(())
    }

    fn sub(&mut self) -> VmResult<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        self.stack.push(a - b)?;
        Ok(())
    }

    fn mul(&mut self) -> VmResult<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        self.stack.push(a * b)?;
        Ok(())
    }

    fn div(&mut self) -> VmResult<()> {
        let mut a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let res = if b.is_zero() {
            b
        } else {
            a /= &b;
            a
        };
        self.stack.push(res)?;
        Ok(())
    }

    fn modulo(&mut self) -> VmResult<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        self.stack.push(
            match b.is_zero() {
                true => b,
                false => a % b,
            }
        )?;
        Ok(())
    }

    fn addmod(&mut self) -> VmResult<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let c = self.stack.pop()?;
        self.stack.push(
            match c.is_zero() {
                true => c,
                false => (a + b) % c,
            }
        )?;
        Ok(())
    }

    fn mulmod(&mut self) -> VmResult<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let c = self.stack.pop()?;
        self.stack.push(
            match c.is_zero() {
                true => c,
                false => (a * b) % c,
            }
        )?;
        Ok(())
    }

    fn lt(&mut self) -> VmResult<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        self.stack.push((a < b).into())?;
        Ok(())
    }

    fn gt(&mut self) -> VmResult<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        self.stack.push((a > b).into())?;
        Ok(())
    }

    fn eq(&mut self) -> VmResult<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        self.stack.push((a == b).into())?;
        Ok(())
    }

    fn is_zero(&mut self) -> VmResult<()> {
        let a = self.stack.pop()?;
        self.stack.push(a.is_zero().into())?;
        Ok(())
    }

    fn shl(&mut self) -> VmResult<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        self.stack.push(b << a)?;
        Ok(())
    }

    fn shr(&mut self) -> VmResult<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        self.stack.push(b >> a)?;
        Ok(())
    }

    fn and(&mut self) -> VmResult<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        self.stack.push(a & b)?;
        Ok(())
    }

    fn or(&mut self) -> VmResult<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        self.stack.push(a | b)?;
        Ok(())
    }

    fn xor(&mut self) -> VmResult<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        self.stack.push(a ^ b)?;
        Ok(())
    }

    fn not(&mut self) -> VmResult<()> {
        let mut a = self.stack.pop()?;
        a.twos_compliment();
        self.stack.push(a)?;
        Ok(())
    }

    fn pop(&mut self) -> VmResult<()> {
        self.stack.pop()?;
        Ok(())
    }

    fn push(&mut self, amt: usize) -> VmResult<()> {
        let mut word = U256bytes::default();
        let len = word.len();
        let start_idx = self.pc;
        for i in 0..amt {
            let byte = self.bytecode[start_idx + (amt - 1) - i];
            word[len - i - 1] = byte;
        }
        self.pc += amt;
        self.stack.push(word.into())?;
        Ok(())
    }

    fn dup(&mut self, pos: usize) -> VmResult<()> {
        self.stack.dup(pos)?;
        Ok(())
    }

    fn swap(&mut self, pos: usize) -> VmResult<()> {
        self.stack.swap(pos)?;
        Ok(())
    }

    fn mstore(&mut self) -> VmResult<()> {
        let offset = self.stack.pop()?;
        let word = self.stack.pop()?;
        let size: usize = self.memory.size().into();
        let word_size = 32;
        let offset_size: usize = offset.clone().into();
        if size < offset_size + word_size {
            let grow_size = offset_size + word_size - size;
            self.memory.grow(grow_size);
        }
        let val: U256bytes = word.into();
        self.memory.store(offset, &val, word_size);
        Ok(())
    }

    fn mstore8(&mut self) -> VmResult<()> {
        let offset = self.stack.pop()?;
        let word = self.stack.pop()?;
        let size = self.memory.size();
        if size < offset {
            self.memory.grow(1);
        }
        let val: U256bytes = word.into();
        let mut default_val = U256bytes::default();
        default_val[default_val.len() - 1] = val[val.len() - 1] & 0xff;
        self.memory.store(offset, &default_val, 1);
        Ok(())
    }

    fn mload(&mut self) -> VmResult<()> {
        let offset = self.stack.pop()?;
        match self.memory.load(offset) {
            None => Err(VmError::StackUnderflow(String::from("Stack underflow"))),
            Some(value) => {
                self.stack.push(value.into())?;
                Ok(())
            }
        }
    }

    fn msize(&mut self) -> VmResult<()> {
        let size = self.memory.size();
        self.stack.push(size)?;
        Ok(())
    }

    fn pc(&mut self) -> VmResult<()> {
        let pc = U256::from(self.pc);
        self.stack.push(pc)?;
        Ok(())
    }

    fn _jump(&mut self, size_target: usize) -> VmResult<()> {
        if size_target < self.bytecode.len() {
            return Err(VmError::InvalidJump(String::from("Invalid jump")))
        }

        match Opcode::from_u8(self.bytecode[size_target]) {
            Some(Opcode::JUMPDEST) => {},
            _ => { return Err(VmError::InvalidJump(String::from("Invalid jump"))) }
        };
        self.pc = size_target;
        Ok(())
    }

    fn jump(&mut self) -> VmResult<()> {
        let target = self.stack.pop()?;
        self._jump(target.into())
    }

    fn jumpi(&mut self) -> VmResult<()> {
        let target = self.stack.pop()?;
        let condition = self.stack.pop()?;
        match condition.is_zero() {
            true => Ok(()),
            false => self._jump(target.into())
        }
    }

    fn ret(&mut self) -> VmResult<()> {
        let offset = self.stack.pop()?;
        let len = self.stack.pop()?;
        match self.memory.address_to_memptr(offset) {
            Some(offset_ptr) => {
                self.eei.finish(offset_ptr, len.into());
                Ok(())
            },
            None => Err(VmError::OutOfRange(String::from("Memory address invalid")))
        }
    }

    fn revert(&mut self) -> VmResult<()> {
        let offset = self.stack.pop()?;
        let len = self.stack.pop()?;
        match self.memory.address_to_memptr(offset) {
            Some(offset_ptr) => {
                self.eei.revert(offset_ptr, len.into());
                Ok(())
            },
            None => Err(VmError::OutOfRange(String::from("Memory address invalid")))
        }
    }

    fn sstore(&mut self) -> VmResult<()> {
        let key = self.stack.pop()?;
        let value = self.stack.pop()?;

        let size: usize = self.wasm_mem.size().into();
        let word_len = 32;
        let required_size = word_len * 2;
        if size < required_size {
            self.wasm_mem.grow(required_size);
        }
        let key_bytes: U256bytes = key.into();
        let val_bytes: U256bytes = value.into();
        let key_offset = U256::default();
        let val_offset = U256::from(word_len);
        self.wasm_mem.store(key_offset.clone(), &key_bytes, word_len);
        self.wasm_mem.store(val_offset.clone(), &val_bytes, word_len);
        match (
            self.wasm_mem.address_to_memptr(key_offset),
            self.wasm_mem.address_to_memptr(val_offset)
        ) {
            (Some(key_ptr), Some(val_ptr)) => {
                self.eei.sstore(key_ptr, val_ptr);
                Ok(())
            },
            (_, _) => Err(VmError::OutOfRange(String::from("Memory address invalid")))
        }
    }

    fn sload(&mut self) -> VmResult<()> {
        let key = self.stack.pop()?;

        let size: usize = self.wasm_mem.size().into();
        let word_len = 32;
        let required_size = word_len * 2;
        if size < required_size {
            self.wasm_mem.grow(required_size);
        }
        let key_bytes: U256bytes = key.into();
        let key_offset = U256::default();
        let result_offset = U256::from(word_len);
        self.wasm_mem.store(key_offset.clone(), &key_bytes, word_len);
        match (
            self.wasm_mem.address_to_memptr(key_offset),
            self.wasm_mem.address_to_memptr(result_offset.clone())
        ) {
            (Some(key_ptr), Some(result_ptr)) => {
                self.eei.sload(key_ptr, result_ptr);
                match self.wasm_mem.load(result_offset) {
                    Some(result_bytes) => {
                        let sload_result = U256::from(result_bytes);
                        self.stack.push(sload_result)?;
                        Ok(())
                    },
                    None => Err(VmError::OutOfRange(String::from("Memory address invalid")))
                }
            },
            (_, _) => Err(VmError::OutOfRange(String::from("Memory address invalid")))
        }
    }

    fn address(&mut self) -> VmResult<()> {
        let addr = self.eei.get_address();
        self.stack.push(addr)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eei::EeiMock;
    use core::slice;

    #[test]
    fn stack_push() {
        let a = 25;
        let bytecode:[u8; 3] = [0x60, a, 0];
        let bytecode_vec = bytecode.iter().map(|a| *a).collect();
        let eei = EeiMock::new();
        let mut interpreter = Interpreter::new(
            bytecode_vec,
            Box::new(eei)
        );
        assert_eq!(0, match interpreter.execute() {
            Err(_) => 0,
            Ok(_) => 1
        });

        let actual = match interpreter.stack.pop() {
            Ok(v) => v,
            Err(_) => U256::default()
        };
        let expected = U256::from(a as usize);
        assert_eq!(actual, expected);
    }

    #[test]
    fn stack_mul() {
        let a = 25;
        let b = 26;
        let bytecode:[u8; 6] = [0x60, a, 0x60, b, 0x02, 0];
        let bytecode_vec = bytecode.iter().map(|a| *a).collect();
        let eei = EeiMock::new();
        let mut interpreter = Interpreter::new(
            bytecode_vec,
            Box::new(eei)
        );
        assert_eq!(0, match interpreter.execute() {
            Err(_) => 0,
            Ok(_) => 1
        });

        let actual = match interpreter.stack.pop() {
            Ok(v) => v,
            Err(_) => U256::default()
        };
        let expected = U256::from(a as usize * b as usize);
        assert_eq!(actual, expected);
    }

    #[test]
    fn stack_mul_swap() {
        let a = 25;
        let b = 26;
        let bytecode:[u8; 9] = [0x60, a, 0x60, b, 0x02, 0x60, a, 0x90, 0];
        let bytecode_vec = bytecode.iter().map(|a| *a).collect();
        let eei = EeiMock::new();
        let mut interpreter = Interpreter::new(
            bytecode_vec,
            Box::new(eei)
        );
        assert_eq!(0, match interpreter.execute() {
            Err(_) => 0,
            Ok(_) => 1
        });

        let actual_stack_top = match interpreter.stack.pop() {
            Ok(v) => v,
            Err(_) => U256::default()
        };
        let expected_stack_top = U256::from(a as usize * b as usize);
        assert_eq!(actual_stack_top, expected_stack_top);
        let actual_stack_bottom = match interpreter.stack.pop() {
            Ok(v) => v,
            Err(_) => U256::default()
        };
        let expected_stack_bottom = U256::from(a as usize);
        assert_eq!(actual_stack_bottom, expected_stack_bottom);
    }

    #[test]
    fn stack_mul_and_memory() {
        let a = 25;
        let b = 26;
        let bytecode:[u8; 9] = [0x60, a, 0x60, b, 0x02, 0x60, 0x0, 0x52, 0];
        let bytecode_vec = bytecode.iter().map(|a| *a).collect();
        let eei = EeiMock::new();
        let mut interpreter = Interpreter::new(
            bytecode_vec,
            Box::new(eei)
        );
        assert_eq!(0, match interpreter.execute() {
            Err(_) => 0,
            Ok(_) => 1
        });

        let mem_word: U256 = match interpreter.memory.load(U256::default()) {
            None => U256bytes::default().into(),
            Some(value) => U256bytes::from(value).into()
        };
        assert_eq!(mem_word, U256::from(a as usize * b as usize));
    }

    #[test]
    fn stack_mul_memory_and_ret() {
        let a = 25;
        let b = 26;
        let return_data_size = 0x20;
        let bytecode:[u8; 13] = [0x60, a, 0x60, b, 0x02, 0x60, 0x0, 0x52, 0x60, return_data_size, 0x60, 0x0, 0xf3];
        let bytecode_vec = bytecode.iter().map(|a| *a).collect();
        let eei = EeiMock::new();
        let mut interpreter = Interpreter::new(
            bytecode_vec,
            Box::new(eei)
        );
        assert_eq!(0, match interpreter.execute() {
            Err(_) => 0,
            Ok(_) => 1
        });

        let Interpreter { eei: actual_eei, .. } = interpreter;
        let eei_instance: &EeiMock = match actual_eei.as_any().downcast_ref::<EeiMock>() {
            Some(inst) => inst,
            None => panic!("&EEI isn't a EeiMock!"),
        };
        let &EeiMock {
            return_data_size: actual_return_data_size,
            return_data_ptr
        } = eei_instance;
        assert_eq!(actual_return_data_size, return_data_size as usize);
        let return_data: &mut [u8] = unsafe { slice::from_raw_parts_mut(return_data_ptr, actual_return_data_size) };
        let mut bytes = U256bytes::default();
        for (idx, value) in return_data.iter().enumerate() {
            bytes[idx] = *value;
        }
        assert_eq!(U256::from(bytes), U256::from(a as usize * b as usize));
    }

    #[test]
    fn stack_memory() {
        let a = 25;
        let bytecode:[u8; 6] = [0x60, a, 0x60, 0x0, 0x52, 0];
        let bytecode_vec = bytecode.iter().map(|a| *a).collect();
        let eei = EeiMock::new();
        let mut interpreter = Interpreter::new(
            bytecode_vec,
            Box::new(eei)
        );
        assert_eq!(0, match interpreter.execute() {
            Err(_) => 0,
            Ok(_) => 1
        });

        let mem_word: U256 = match interpreter.memory.load(U256::default()) {
            None => U256bytes::default().into(),
            Some(value) => U256bytes::from(value).into()
        };
        assert_eq!(mem_word, U256::from(a as usize));
    }
}