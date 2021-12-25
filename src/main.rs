#[allow(dead_code)]

const NUMERIC: &[u8] = &[48, 49, 50, 51, 52, 53, 54, 55, 56, 57];
const ALPHANUMERIC: &[u8] = &[
    32, 36, 37, 42, 43, 45, 46, 47, 58, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
];

fn main() {
    let data = "HELLO WORLD";
    let qr = QrCode::new(data.as_bytes());
    qr.encode();
}

#[derive(Debug)]
struct Bitstring {
    data: Vec<bool>,
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

        dbg!(bitstring);
    }
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
