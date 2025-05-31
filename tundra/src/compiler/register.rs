use crate::bytecode::value::Value;

/// A very simple register allocator using a free-list (LIFO),
pub struct RegisterAllocator {
    /// Stack of free register indices
    free_registers:   Vec<usize>,
    /// Most-recently allocated registers 
    allocation_stack: Vec<usize>,
    /// Current Value stored in each register slot
    values:           Vec<Value>,
    /// Whether each slot is currently in use
    in_use:           Vec<bool>,
}

impl RegisterAllocator {
    /// Create 1024 registers, initially all free
    pub fn new() -> Self {
        let mut free_registers = Vec::with_capacity(1024);
        for i in (0..1024).rev() {
            free_registers.push(i);
        }
        Self {
            free_registers,
            allocation_stack: Vec::new(),
            values:           vec![Value::none(); 1024],
            in_use:           vec![false;    1024],
        }
    }

    /// Reserve a specific register so `allocate` won’t hand it out.
    pub fn reserve(&mut self, reg: usize) {
        if let Some(pos) = self.free_registers.iter().position(|&r| r == reg) {
            self.free_registers.remove(pos);
        } else {
            panic!("Tried to reserve register {} but it was not free", reg);
        }
    }

    /// Read-only access to the Value in slot `reg`.
    pub fn get_value(&self, reg: usize) -> &Value {
        &self.values[reg]
    }

    /// Allocate one register and associate it with `value`.
    pub fn allocate(&mut self, value: Value) -> Result<usize, String> {
        let reg = self.free_registers
            .pop()
            .ok_or_else(|| "Out of registers".to_string())?;
        debug_assert!(!self.in_use[reg], "Allocating a register already in use");
        self.in_use[reg]         = true;
        self.values[reg]         = value;
        self.allocation_stack.push(reg);

        Ok(reg)
    }

    /// Free a previously-allocated register.
    pub fn free(&mut self, reg: usize) {

        if self.in_use[reg] {
            self.in_use[reg] = false;
            self.values[reg] = Value::none();             
            self.free_registers.push(reg);                
            if let Some(pos) = self.allocation_stack.iter().rposition(|&r| r == reg) {
                self.allocation_stack.remove(pos);
            }
        }
    }
    /// Returns the most-recently allocated register
    pub fn last_allocated(&self) -> Option<usize> {
        self.allocation_stack.last().cloned()
    }
}