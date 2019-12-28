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

    pub fn div(&mut self) -> VmResult<()> {
        let a = self.run_state.stack.pop()?;
        let b = self.run_state.stack.pop()?;
        let res = if b.is_zero() {
            b
        } else {
            a / b
        };
        self.run_state.stack.push(res);
        Ok(())
    }
    */
}