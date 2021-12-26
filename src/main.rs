use std::fmt;

use qr_spec::{
    count_length, identify_data_mode, DataMode, ErrorCorrectionLevel, ALPHANUMERIC_TABLE,
};

#[allow(dead_code)]
mod qr_spec;

fn main() {
    let data = "Hello, World!";
    // let data = "HELLO WORLD";
    // let data = "1123581321";
    let qr = QrCode::new(data.as_bytes());
    qr.encode();
}

#[derive(Debug)]
struct Bitstring {
    data: Vec<bool>,
}
impl fmt::Display for Bitstring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self
            .data
            .iter()
            .map(|e| if *e { "1" } else { "0" })
            .collect();
        write!(f, "{}", s)
    }
}
impl Bitstring {
    fn push(&mut self, packet: usize, bits: usize) {
        (0..bits)
            .rev()
            .for_each(|i| self.set((packet >> i) & 1 == 1))
    }

    fn set(&mut self, bit: bool) {
        self.data.push(bit)
    }
}

#[derive(Debug, Default)]
struct QrCode {
    data: Vec<u8>,
    mode: DataMode,
    _err_mode: ErrorCorrectionLevel,
    version: u8,
}
impl QrCode {
    fn new(data: &[u8]) -> Self {
        let mode = identify_data_mode(data);
        Self {
            data: data.into(),
            mode,
            ..Default::default()
        }
    }

    fn encode(&self) {
        let mut bitstring = Bitstring { data: vec![] };

        let mode = self.mode.encode();

        for i in (0..4).rev() {
            bitstring.set((mode >> i) & 1 == 1);
        }

        let char_count = self.data.len();
        let char_count_bitlength = count_length(self.mode, self.version);

        bitstring.push(char_count, char_count_bitlength);

        self.encode_data(&mut bitstring);
        println!("{}", bitstring);
    }

    fn encode_data(&self, bitstring: &mut Bitstring) {
        match self.mode {
            DataMode::Numeric => encode_numeric(&self.data, bitstring),
            DataMode::Alphanumeric => encode_alphanumeric(&self.data, bitstring),
            DataMode::Text => encode_text(&self.data, bitstring),
            DataMode::Kanji => panic!("kanji not supported"),
        };
    }
}

fn encode_numeric(data: &[u8], bitstring: &mut Bitstring) {
    data.chunks(3).for_each(|c| {
        let n: usize = std::str::from_utf8(c).unwrap().parse().unwrap();
        if n >= 100 {
            bitstring.push(n, 10);
        } else if n >= 10 {
            bitstring.push(n, 7);
        } else {
            bitstring.push(n, 4);
        }
    });
}

fn encode_alphanumeric(data: &[u8], bitstring: &mut Bitstring) {
    data.chunks(2).for_each(|c| match c {
        [a, b] => {
            let n = ALPHANUMERIC_TABLE[*a as usize] as usize * 45usize + *b as usize;
            bitstring.push(n.into(), 11);
        }
        [a] => {
            let n = ALPHANUMERIC_TABLE[*a as usize].into();
            bitstring.push(n, 6);
        }
        _ => panic!("wtf, can't be here"),
    });
}

fn encode_text(data: &[u8], bitstring: &mut Bitstring) {
    data.iter().for_each(|c| bitstring.push(*c as usize, 8));
}
