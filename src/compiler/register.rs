use crate::bytecode::value::Value;

#[derive(Debug)]
pub struct Register{
    pub value:Value,
    pub index:usize,
    pub free:bool,
}
impl Register {
    pub fn new(index: usize) -> Self {
        Register {
            value: Value::none(),
            index,
            free: true,
        }
    }
}
pub struct RegisterAllocator {
    pub free_registers: [Register; 128], 
}

impl RegisterAllocator {
    pub fn new() -> Self {
        let mut registers = core::array::from_fn(|i| Register::new(i));
        for (i, reg) in registers.iter_mut().enumerate() {
            *reg = Register::new(i);
        }
        RegisterAllocator {
            free_registers: registers,
        }
    }

    pub fn allocate(&mut self, value: Value) -> Result<usize, String> {
        if let Some(reg) = self.free_registers.iter_mut().find(|r| r.free) {
            reg.free = false;
            reg.value = value;
            Ok(reg.index)
        } else {
            Err("Out of registers".to_string())
        }
    }


    pub fn free(&mut self, reg_index: usize) {
        if reg_index < self.free_registers.len() {
            self.free_registers[reg_index].value=Value::none();
            self.free_registers[reg_index].free = true;
        }
    }
}
