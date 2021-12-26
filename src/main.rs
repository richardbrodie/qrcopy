use std::fmt;

#[allow(dead_code)]

const NUMERIC: &[u8] = &[48, 49, 50, 51, 52, 53, 54, 55, 56, 57];
const ALPHANUMERIC: &[u8] = &[
    32, 36, 37, 42, 43, 45, 46, 47, 58, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
];

// Simple lookup table based on this: https://www.thonky.com/qr-code-tutorial/alphanumeric-table
// table[ACSII_value] = QR_value
const ALPHANUMERIC_TABLE: &[u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    36, 0, 0, 0, 37, 38, 0, 0, 0, 0, 39, 40, 0, 41, 42, 43, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 44, 0, 0,
    0, 0, 0, 0, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35,
];

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
    fn pad(&mut self, bit: bool, bits: usize) {
        (0..bits).for_each(|_| self.set(bit))
    }

    fn push(&mut self, packet: usize, bits: usize) {
        (0..bits)
            .rev()
            .for_each(|i| self.set((packet >> i) & 1 == 1))
    }

    fn set(&mut self, bit: bool) {
        self.data.push(bit)
    }
}

#[derive(Debug)]
enum DataMode {
    Numeric,
    Alphanumeric,
    Text,
    Kanji,
}
impl Default for DataMode {
    fn default() -> Self {
        Self::Text
    }
}
impl DataMode {
    fn encode(&self) -> u8 {
        match self {
            Self::Numeric => 1,
            Self::Alphanumeric => 2,
            Self::Text => 4,
            Self::Kanji => 8,
        }
    }
}

#[derive(Debug)]
enum ErrorCorrectionLevel {
    Low,
    Medium,
    Quartile,
    High,
}
impl Default for ErrorCorrectionLevel {
    fn default() -> Self {
        Self::Low
    }
}

#[derive(Debug, Default)]
struct QrCode {
    data: Vec<u8>,
    mode: DataMode,
    err_mode: ErrorCorrectionLevel,
    version: u8,
}
impl QrCode {
    fn count_length(&self) -> usize {
        if self.version <= 9 {
            match self.mode {
                DataMode::Numeric => 10,
                DataMode::Alphanumeric => 9,
                _ => 8,
            }
        } else if self.version <= 26 {
            match self.mode {
                DataMode::Numeric => 12,
                DataMode::Alphanumeric => 11,
                DataMode::Text => 16,
                DataMode::Kanji => 10,
            }
        } else {
            match self.mode {
                DataMode::Numeric => 14,
                DataMode::Alphanumeric => 13,
                DataMode::Text => 16,
                DataMode::Kanji => 12,
            }
        }
    }

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
        let char_count_bitlength = self.count_length();

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

fn identify_data_mode(data: &[u8]) -> DataMode {
    if data.iter().all(|e| NUMERIC.contains(e)) {
        println!("numeric");
        DataMode::Numeric
    } else if data
        .iter()
        .all(|e| NUMERIC.contains(e) | ALPHANUMERIC.contains(e))
    {
        println!("alphanumeric");
        DataMode::Alphanumeric
    } else {
        println!("text");
        DataMode::Text
    }
}
