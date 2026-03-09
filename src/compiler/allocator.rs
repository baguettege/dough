pub(super) struct Allocator {
    next: u8,
    free_list: Vec<u8>
}

impl Allocator {
    pub(super) fn new() -> Self {
        Self {
            next: 0,
            free_list: Vec::new()
        }
    }

    pub(super) fn alloc(&mut self) -> u8 {
        match self.free_list.pop() {
            None => {
                let reg = self.next;
                self.next += 1;
                reg
            },
            Some(reg) => reg
        }
    }

    pub(super) fn alloc_range(&mut self, count: u8) -> u8 {
        let first = self.next;
        self.next += count;
        first
    }

    pub(super) fn free(&mut self, reg: u8) {
        self.free_list.push(reg);
    }

    pub(super) fn reg_count(&self) -> usize {
        self.next as usize
    }
}