use std::fmt::Debug;

#[derive(Debug)]
pub struct RingBuffer<T> {
    storage: Vec<Option<T>>,
    read_idx: usize,
    write_idx: usize,
    full: bool,
}

#[derive(Debug, PartialEq)]
pub struct Full;

impl<T: Clone + Debug> RingBuffer<T> {
    pub fn new(capacity: usize) -> Result<Self, &'static str> {
        if capacity <= 0 {
            return Err("capacity must be non-negative value");
        }
        return Ok(Self {
            storage: vec![None; capacity],
            read_idx: 0,
            write_idx: 0,
            full: false,
        });
    }

    pub fn offer(&mut self, item: T) -> Result<(), Full> {
        if self.is_full() {
            return Err(Full);
        }
        self.storage[self.write_idx] = Some(item);
        self.write_idx = self.advance_idx(self.write_idx);

        if self.write_idx == self.read_idx {
            self.full = true;
        }

        // all okay here
        return Ok(());
    }

    pub fn poll(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.full = false;
        let ridx = self.read_idx;

        let val = &self.storage.get(ridx);

        self.read_idx = self.advance_idx(self.read_idx);

        if let Some(&ref a) = val {
            let a = a.clone();
            self.storage[ridx] = None;
            return a;
        }

        return None;
    }

    /// helper functions
    fn advance_idx(&self, idx: usize) -> usize {
        (idx + 1) % self.storage.len()
    }

    fn is_full(&self) -> bool {
        self.full
    }

    fn is_empty(&self) -> bool {
        self.write_idx == self.read_idx && !self.full
    }
}
