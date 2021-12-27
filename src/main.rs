use qr_spec::{
    count_length, identify_data_mode, DataMode, ErrorCorrectionLevel, ALPHANUMERIC_TABLE,
};

use crate::bitstring::Bitstring;

mod bitstring;
#[allow(dead_code)]
mod qr_spec;

fn main() {
    // let data = "Hello, World!";
    let data = "HELLO WORLD";
    // let data = "1123581321";
    let qr = QrCode::new(data.as_bytes());
    qr.encode();
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

    fn encode(&self) -> Bitstring {
        let mut bitstring = Bitstring::new();

        let mode = self.mode.encode();

        for i in (0..4).rev() {
            bitstring.set((mode >> i) & 1 == 1);
        }

        let char_count = self.data.len();
        let char_count_bitlength = count_length(self.mode, self.version);

        bitstring.push(char_count, char_count_bitlength);

        self.encode_data(&mut bitstring);
        bitstring
    }

    fn encode_data(&self, bitstring: &mut Bitstring) {
        match self.mode {
            DataMode::Numeric => encode_numeric(&self.data),
            DataMode::Alphanumeric => encode_alphanumeric(&self.data),
            DataMode::Text => encode_text(&self.data),
            DataMode::Kanji => panic!("kanji not supported"),
        }
        .iter()
        .for_each(|f| bitstring.push(f.0, f.1));
    }
}

#[derive(PartialEq, Debug)]
struct BitsWithLength(usize, usize);

fn encode_numeric(data: &[u8]) -> Vec<BitsWithLength> {
    data.chunks(3)
        .map(|c| {
            let n: usize = std::str::from_utf8(c).unwrap().parse().unwrap();
            if n >= 100 {
                BitsWithLength(n, 10)
            } else if n >= 10 {
                BitsWithLength(n, 7)
            } else {
                BitsWithLength(n, 4)
            }
        })
        .collect()
}

fn encode_alphanumeric(data: &[u8]) -> Vec<BitsWithLength> {
    data.chunks(2)
        .map(|c| match c {
            [a, b] => {
                let n = (ALPHANUMERIC_TABLE[*a as usize] as usize * 45usize)
                    + ALPHANUMERIC_TABLE[*b as usize] as usize;
                BitsWithLength(n.into(), 11)
            }
            [a] => {
                let n = ALPHANUMERIC_TABLE[*a as usize].into();
                BitsWithLength(n, 6)
            }
            _ => panic!("wtf, can't be here"),
        })
        .collect()
}

fn encode_text(data: &[u8]) -> Vec<BitsWithLength> {
    data.iter()
        .map(|c| BitsWithLength(*c as usize, 8))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{encode_alphanumeric, encode_numeric, BitsWithLength, QrCode};

    #[test]
    fn test_alphanumeric_qr() {
        let data = "HELLO WORLD";
        let qr = QrCode::new(data.as_bytes());
        let bs = qr.encode();
        let goal1: usize = 0b0010_000001011_01100001011_01111000110_10001011100_10110111000_1001101;
        let goal2: usize = 0b0100_001101000_00000000000_00000000000_00000000000_00000000000_0000000;
        assert_eq!(bs.data()[0], goal1);
        assert_eq!(bs.data()[1], goal2);
    }

    #[test]
    fn test_alphanumeric_h() {
        let data = "H";
        let res = encode_alphanumeric(data.as_bytes());
        let goal: usize = 0b010001;
        assert_eq!(res[0], BitsWithLength(goal, 6));
    }

    #[test]
    fn test_alphanumeric_he() {
        let data = "HE";
        let res = encode_alphanumeric(data.as_bytes());
        let goal: usize = 0b01100001011;
        assert_eq!(res[0], BitsWithLength(goal, 11));
    }

    #[test]
    fn test_alphanumeric_hell() {
        let data = "HELL";
        let res: Vec<_> = encode_alphanumeric(data.as_bytes());
        let goal: usize = 0b01100001011;
        assert_eq!(res[0], BitsWithLength(goal, 11));
    }

    #[test]
    fn test_numeric_1() {
        let data = "1";
        let res = encode_numeric(data.as_bytes());
        let goal: usize = 0b1;
        assert_eq!(res[0], BitsWithLength(goal, 4));
    }

    #[test]
    fn test_numeric_12() {
        let data = "12";
        let res = encode_numeric(data.as_bytes());
        let goal: usize = 0b1100;
        assert_eq!(res[0], BitsWithLength(goal, 7));
    }

    #[test]
    fn test_numeric_123() {
        let data = "123";
        let res = encode_numeric(data.as_bytes());
        let goal: usize = 0b1111011;
        assert_eq!(res[0], BitsWithLength(goal, 10));
    }

    #[test]
    fn test_numeric_999() {
        let data = "999";
        let res = encode_numeric(data.as_bytes());
        let goal: usize = 0b1111100111;
        assert_eq!(res[0], BitsWithLength(goal, 10));
    }

    #[test]
    fn test_numeric_1000() {
        let data = "1000";
        let res = encode_numeric(data.as_bytes());
        assert_eq!(res[0], BitsWithLength(0b1100100, 10));
        assert_eq!(res[1], BitsWithLength(0b0, 4));
    }
}
