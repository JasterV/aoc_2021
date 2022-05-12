use std::collections::HashSet;

pub struct DigitDisplay {
    wires: HashSet<u8>,
}

impl DigitDisplay {
    pub fn len(&self) -> usize {
        self.wires.len()
    }
}

impl From<&str> for DigitDisplay {
    fn from(seq: &str) -> Self {
        let wires: HashSet<u8> = HashSet::from_iter(seq.as_bytes().iter().cloned());
        Self { wires }
    }
}
