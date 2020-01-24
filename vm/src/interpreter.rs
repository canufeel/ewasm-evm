use alloc::{string::String, vec::Vec, boxed::Box};
use crate::run_state::RunState;
use crate::vm_error::{VmResult, VmError};
use u256::u256::{U256bytes, U256};
use crate::opcode::Opcode;
use crate::eei_common::EEI;
cfg_if::cfg_if! {
    if #[cfg(target = "wasm32-unknown-unknown")] {
        use crate::eei::{debug};
    }
}

pub struct Interpreter {
    run_state: RunState
}

impl Interpreter {
    pub fn new(bytecode: Vec<u8>, eei: Box<dyn EEI>) -> Self {
        Interpreter {
            run_state: RunState::new(bytecode, eei)
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
        let pc = self.run_state.pc;
        self.run_state.pc += 1;

        #[cfg(target = "wasm32-unknown-unknown")]
        debug::log_debug_local(pc as i32);

        let opcode = match Opcode::from_u8(self.run_state.bytecode[pc]) {
            Some(c) => Ok(c),
            None => Err(
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
            Opcode::RETURN => self.ret(),
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
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(a + b)?;
        Ok(())
    }

    fn sub(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(a - b)?;
        Ok(())
    }

    fn mul(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(a * b)?;
        Ok(())
    }

    fn div(&mut self) -> VmResult<()> {
        let mut a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        let res = if b.is_zero() {
            b
        } else {
            a /= &b;
            a
        };
        self.run_state.stack.push(res)?;
        Ok(())
    }

    fn modulo(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(
            match b.is_zero() {
                true => b,
                false => a % b,
            }
        )?;
        Ok(())
    }

    fn addmod(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        let c = self.run_state.stack.pop()?;
        self.run_state.stack.push(
            match c.is_zero() {
                true => c,
                false => (a + b) % c,
            }
        )?;
        Ok(())
    }

    fn mulmod(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        let c = self.run_state.stack.pop()?;
        self.run_state.stack.push(
            match c.is_zero() {
                true => c,
                false => (a * b) % c,
            }
        )?;
        Ok(())
    }

    fn lt(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push((a < b).into())?;
        Ok(())
    }

    fn gt(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push((a > b).into())?;
        Ok(())
    }

    fn eq(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push((a == b).into())?;
        Ok(())
    }

    fn is_zero(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        self.run_state.stack.push(a.is_zero().into())?;
        Ok(())
    }

    fn shl(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(b << a)?;
        Ok(())
    }

    fn shr(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(b >> a)?;
        Ok(())
    }

    fn and(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(a & b)?;
        Ok(())
    }

    fn or(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(a | b)?;
        Ok(())
    }

    fn xor(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(a ^ b)?;
        Ok(())
    }

    fn not(&mut self) -> VmResult<()> {
        let mut a = self.run_state.stack.pop()?;
        a.twos_compliment();
        self.run_state.stack.push(a)?;
        Ok(())
    }

    fn pop(&mut self) -> VmResult<()> {
        self.run_state.stack.pop()?;
        Ok(())
    }

    fn push(&mut self, amt: usize) -> VmResult<()> {
        let mut word = U256bytes::default();
        let len = word.len();
        let start_idx = self.run_state.pc;
        for i in 0..amt {
            let byte = self.run_state.bytecode[start_idx + (amt - 1) - i];
            word[len - i - 1] = byte;
        }
        self.run_state.pc += amt;
        self.run_state.stack.push(word.into())?;
        Ok(())
    }

    fn dup(&mut self, pos: usize) -> VmResult<()> {
        self.run_state.stack.dup(pos)?;
        Ok(())
    }

    fn swap(&mut self, pos: usize) -> VmResult<()> {
        self.run_state.stack.swap(pos)?;
        Ok(())
    }

    fn mstore(&mut self) -> VmResult<()> {
        let offset = self.run_state.stack.pop()?;
        let word = self.run_state.stack.pop()?;
        let word_size: usize = word.clone().into();
        let size = self.run_state.memory.size();
        if size < offset {
            self.run_state.memory.grow(32);
        }
        let val: U256bytes = word.into();
        self.run_state.memory.store(offset, &val, 32);
        Ok(())
    }

    fn mstore8(&mut self) -> VmResult<()> {
        let offset = self.run_state.stack.pop()?;
        let word = self.run_state.stack.pop()?;
        let size = self.run_state.memory.size();
        if size < offset {
            self.run_state.memory.grow(1);
        }
        let val: U256bytes = word.into();
        let mut default_val = U256bytes::default();
        default_val[default_val.len() - 1] = val[val.len() - 1] & 0xff;
        self.run_state.memory.store(offset, &default_val, 1);
        Ok(())
    }

    fn mload(&mut self) -> VmResult<()> {
        let offset = self.run_state.stack.pop()?;
        match self.run_state.memory.load(offset) {
            None => Err(VmError::StackUnderflow(String::from("Stack underflow"))),
            Some(value) => {
                self.run_state.stack.push(value.into())?;
                Ok(())
            }
        }
    }

    fn msize(&mut self) -> VmResult<()> {
        let size = self.run_state.memory.size();
        self.run_state.stack.push(size)?;
        Ok(())
    }

    fn pc(&mut self) -> VmResult<()> {
        let pc = U256::from(self.run_state.pc);
        self.run_state.stack.push(pc)?;
        Ok(())
    }

    fn _jump(&mut self, size_target: usize) -> VmResult<()> {
        if size_target < self.run_state.bytecode.len() {
            return Err(VmError::InvalidJump(String::from("Invalid jump")))
        }

        match Opcode::from_u8(self.run_state.bytecode[size_target]) {
            Some(Opcode::JUMPDEST) => {},
            _ => { return Err(VmError::InvalidJump(String::from("Invalid jump"))) }
        };
        self.run_state.pc = size_target;
        Ok(())
    }

    fn jump(&mut self) -> VmResult<()> {
        let target = self.run_state.stack.pop()?;
        self._jump(target.into())
    }

    fn jumpi(&mut self) -> VmResult<()> {
        let target = self.run_state.stack.pop()?;
        let condition = self.run_state.stack.pop()?;
        match condition.is_zero() {
            true => Ok(()),
            false => self._jump(target.into())
        }
    }

    fn ret(&mut self) -> VmResult<()> {
        let offset = self.run_state.stack.pop()?;
        let len = self.run_state.stack.pop()?;
        match self.run_state.memory.address_to_memptr(offset) {
            Some(offset_ptr) => {
                self.run_state.eei.finish(offset_ptr, len.into());
                Ok(())
            },
            None => Err(VmError::OutOfRange(String::from("Memory address invalid")))
        }
    }

    fn address(&mut self) -> VmResult<()> {
        let addr = self.run_state.eei.get_address();
        self.run_state.stack.push(addr)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eei::EeiMock;

    #[test]
    fn smoke_stack_push() {
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

        let actual = match interpreter.run_state.stack.pop() {
            Ok(v) => v,
            Err(_) => U256::default()
        };
        let expected = U256::from(a as usize);
        assert_eq!(actual, expected);
    }

    #[test]
    fn smoke_stack_mul() {
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

        let actual = match interpreter.run_state.stack.pop() {
            Ok(v) => v,
            Err(_) => U256::default()
        };
        let expected = U256::from(a as usize * b as usize);
        assert_eq!(actual, expected);
    }

    #[test]
    fn smoke_stack_mul_swap() {
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

        let actual_stack_top = match interpreter.run_state.stack.pop() {
            Ok(v) => v,
            Err(_) => U256::default()
        };
        let expected_stack_top = U256::from(a as usize * b as usize);
        assert_eq!(actual_stack_top, expected_stack_top);
        let actual_stack_bottom = match interpreter.run_state.stack.pop() {
            Ok(v) => v,
            Err(_) => U256::default()
        };
        let expected_stack_bottom = U256::from(a as usize);
        assert_eq!(actual_stack_bottom, expected_stack_bottom);
    }

    #[test]
    fn smoke_mul_and_memory() {
        let a = 25;
        let b = 26;
        let bytecode:[u8; 10] = [0x60, a, 0x60, b, 0x02, 0x60, 0x0, 0x90, 0x52, 0];
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

        let mem_word: U256 = match interpreter.run_state.memory.load(U256::default()) {
            None => U256bytes::default().into(),
            Some(value) => U256bytes::from(value).into()
        };
        assert_eq!(mem_word, U256::from(a as usize * b as usize));
    }
}