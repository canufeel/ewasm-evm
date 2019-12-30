use crate::run_state::RunState;
use crate::vm_error::VmResult;

pub struct OpCodeRunner {
    run_state: RunState
}

impl OpCodeRunner {
    pub fn new() -> Self {
        OpCodeRunner {
            run_state: RunState::new()
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

    /*
    pub fn mul(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        self.run_state.stack.push(a * b);
        Ok(())
    }
    */

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

    pub fn pop(&mut self) -> VmResult<()> {
        self.run_state.stack.pop()?;
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
}