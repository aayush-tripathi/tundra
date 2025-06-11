use crate::bytecode::value::Value;

/// A very simple register allocator using a free-list (LIFO),
pub struct RegisterAllocator {
    head: Option<usize>,          // start of the free list
    next: [Option<usize>; 1024],  // next[i] = the register after i in the free list
    prev: [Option<usize>; 1024],  // prev[i] = the register before i
    in_use: [bool; 1024],         // whether i is currently allocated
    values: [Value; 1024],        // store contents
    allocation_stack: Vec<usize>, // stack of allocated registers
}

impl RegisterAllocator {
    /// Create 1024 registers, initially all free
    pub fn new() -> Self {
        let mut next = [None; 1024];
        let mut prev = [None; 1024];
        for i in 0..1024 - 1 {
            next[i] = Some(i + 1);
            prev[i + 1] = Some(i);
        }
        RegisterAllocator {
            head: Some(0),
            next,
            prev,
            in_use: [false; 1024],
            values: core::array::from_fn(|_| Value::none()),
            allocation_stack: Vec::new(),
        }
    }

    /// Reserve a specific register so `allocate` won’t hand it out.
    pub fn reserve(&mut self, reg: usize) {
        if self.in_use[reg] {
            panic!("Cannot reserve an in-use register {}", reg);
        }
        if let Some(prev_reg) = self.prev[reg] {
            self.next[prev_reg] = self.next[reg];
        } else {
            self.head = self.next[reg];
        }
        if let Some(next_reg) = self.next[reg] {
            self.prev[next_reg] = self.prev[reg];
        }
        self.next[reg] = None;
        self.prev[reg] = None;
    }

    /// Read-only access to the Value in slot `reg`.
    pub fn get_value(&self, reg: usize) -> &Value {
        &self.values[reg]
    }

    /// Allocate one register and associate it with `value`.
    pub fn allocate(&mut self, v: Value) -> Result<usize, String> {
        let reg = self.head.ok_or("Out of registers")?;
        self.head = self.next[reg];
        if let Some(next_reg) = self.head {
            self.prev[next_reg] = None;
        }
        self.in_use[reg] = true;
        self.values[reg] = v;
        self.next[reg] = None;
        self.prev[reg] = None;
        self.allocation_stack.push(reg);
        Ok(reg)
    }

    /// Free a previously-allocated register.
    pub fn free(&mut self, reg: usize) {
        if !self.in_use[reg] {
            return;
        }
        self.in_use[reg] = false;
        self.values[reg] = Value::none();
        if let Some(old_head) = self.head {
            self.prev[old_head] = Some(reg);
        }
        self.next[reg] = self.head;
        self.prev[reg] = None;
        if let Some(pos) = self.allocation_stack.iter().rposition(|&r| r == reg) {
            self.allocation_stack.remove(pos);
        }
        self.head = Some(reg);
    }

    /// Returns the most-recently allocated register
    pub fn last_allocated(&self) -> Option<usize> {
        self.allocation_stack.last().cloned()
    }
}
