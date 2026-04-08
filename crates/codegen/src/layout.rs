use crate::slot;

pub(crate) struct Layout {
    slots: slot::Table,
    global_count: usize,
    entry_local_count: usize,
    fn_local_counts: Vec<usize>, // indexed by `Slot::Fn(idx)`
}

impl Layout {
    pub(crate) fn new(
        slots: slot::Table,
        global_count: usize,
        entry_local_count: usize,
        fn_local_counts: Vec<usize>
    ) -> Self {
        Self { slots, global_count, entry_local_count, fn_local_counts }
    }

    pub(crate) fn slots(&self) -> &slot::Table {
        &self.slots
    }

    pub(crate) fn global_count(&self) -> usize {
        self.global_count
    }

    pub(crate) fn entry_local_count(&self) -> usize {
        self.entry_local_count
    }

    pub(crate) fn fn_local_counts(&self) -> &[usize] {
        &self.fn_local_counts
    }
}
