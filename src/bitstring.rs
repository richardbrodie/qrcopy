const USIZE_BITS: usize = std::mem::size_of::<usize>() * 8;

#[derive(Debug)]
pub struct Bitstring {
    len: usize,
    data: Vec<usize>,
}
impl Bitstring {
    pub fn new() -> Self {
        Self {
            len: 0,
            data: vec![0],
        }
    }

    pub fn push(&mut self, packet: usize, bits: usize) {
        (0..bits)
            .rev()
            .for_each(|i| self.set((packet >> i) & 1 == 1))
    }

    pub fn set(&mut self, bit: bool) {
        if self.len == 64 {
            self.data.push(0);
        }
        if bit {
            let idx = self.data.len() - 1;
            let shift = USIZE_BITS - (self.len % USIZE_BITS) - 1;
            self.data[idx] = self.data[idx] | 1 << shift;
        }
        self.len += 1;
    }

    pub fn data(&self) -> &[usize] {
        self.data.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::bitstring::Bitstring;

    #[test]
    fn test_add() {
        let num = (68 - 66) / 64 + 1;
        assert_eq!(num, 1);
    }

    #[test]
    fn test_bitstring_set() {
        let mut bs = Bitstring::new();
        bs.set(true);
        assert_eq!(bs.data[0], 0b1 << 63);
    }

    #[test]
    fn test_bitstring_push() {
        let mut bs = Bitstring::new();
        bs.push(0b100100, 60); // <54 x 0><100100>0000
        bs.push(0b100100, 6); // <54 x 0><100100><1|00100>
        assert_eq!(bs.data[0], 0b1001001001);
        assert_eq!(bs.data[1], 0b0);
    }
}
