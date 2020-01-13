use alloc::{string::String};
use crate::run_state::RunState;
use crate::vm_error::{VmResult, VmError};
use u256::u256::{U256bytes, U256};
use crate::opcode::Opcode;

pub struct Interpreter {
    run_state: RunState
}

impl Interpreter {
    pub fn new(bytecode: &[u8]) -> Self {
        Interpreter {
            run_state: RunState::new(bytecode)
        }
    }

    pub fn execute(&mut self) -> VmResult<()> {
        let pc = self.run_state.pc;
        self.run_state.pc += 1;
        let opcode = match Opcode::from_u8(self.run_state.bytecode[pc]) {
            Some(c) => Ok(c),
            None => Err(
                VmError::InvalidOpCode(
                    String::from("Invalid opcode")
                )
            )
        }?;
        match opcode {
            Opcode::ADD => self.add(),
            Opcode::SUB => self.sub(),
            Opcode::MUL => self.mul(),
            Opcode::DIV => self.div(),
            Opcode::LT => self.lt(),
            Opcode::GT => self.gt(),
            Opcode::EQ => self.eq(),
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
            Opcode::MSIZE => self.msize(),
            Opcode::PC => self.pc(),
            Opcode::JUMP => self.jump(),
            Opcode::JUMPI => self.jumpi(),
            Opcode::JUMPDEST => Ok(()),
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

    pub fn add(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(a + b);
        Ok(())
    }

    pub fn sub(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(a - b);
        Ok(())
    }

    pub fn mul(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(a * b);
        Ok(())
    }

    pub fn div(&mut self) -> VmResult<()> {
        let mut a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        let res = if b.is_zero() {
            b
        } else {
            a /= &b;
            a
        };
        self.run_state.stack.push(res);
        Ok(())
    }

    pub fn lt(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push((a < b).into());
        Ok(())
    }

    pub fn gt(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push((a > b).into());
        Ok(())
    }

    pub fn eq(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push((a == b).into());
        Ok(())
    }

    pub fn is_zero(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        self.run_state.stack.push(a.is_zero().into());
        Ok(())
    }

    pub fn shl(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(b << a);
        Ok(())
    }

    pub fn shr(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(b >> a);
        Ok(())
    }

    pub fn and(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(a & b);
        Ok(())
    }

    pub fn or(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(a | b);
        Ok(())
    }

    pub fn xor(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(a ^ b);
        Ok(())
    }

    pub fn not(&mut self) -> VmResult<()> {
        let mut a = self.run_state.stack.pop()?;
        a.twos_compliment();
        self.run_state.stack.push(a);
        Ok(())
    }

    pub fn pop(&mut self) -> VmResult<()> {
        self.run_state.stack.pop()?;
        Ok(())
    }

    fn push(&mut self, amt: usize) -> VmResult<()> {
        let mut word = U256bytes::default();
        let len = word.len();
        let start_idx = self.run_state.pc;
        for i in 0..amt {
            word[len - i] = self.run_state.bytecode[start_idx + amt - i];
        }
        self.run_state.stack.push(word.into());
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

    pub fn mstore(&mut self) -> VmResult<()> {
        let offset = self.run_state.stack.pop()?;
        let word = self.run_state.stack.pop()?;
        let size = self.run_state.memory.size();
        if size < offset {
            self.run_state.memory.grow(offset.clone());
        }
        let val: U256bytes = word.into();
        self.run_state.memory.store(offset, &val, 32);
        Ok(())
    }

    pub fn mload(&mut self) -> VmResult<()> {
        let offset = self.run_state.stack.pop()?;
        match self.run_state.memory.load(offset) {
            None => Err(VmError::StackUnderflow(String::from("Stack underflow"))),
            Some(value) => {
                self.run_state.stack.push(value.into());
                Ok(())
            }
        }
    }

    fn msize(&mut self) -> VmResult<()> {
        let size = self.run_state.memory.size();
        self.run_state.stack.push(size);
        Ok(())
    }

    fn pc(&mut self) -> VmResult<()> {
        let pc = U256::from(self.run_state.pc);
        self.run_state.stack.push(pc);
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
}