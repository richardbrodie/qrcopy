#[allow(dead_code)]

const NUMERIC: &[u8] = &[48, 49, 50, 51, 52, 53, 54, 55, 56, 57];
const ALPHANUMERIC: &[u8] = &[
    32, 36, 37, 42, 43, 45, 46, 47, 58, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
];

fn main() {
    let data = "Hello world";
    let qr = QrCode::new(data.as_bytes());
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
    fn new(data: &[u8]) -> Self {
        let mode = identify_data_mode(data);
        Self {
            data: data.into(),
            mode,
            ..Default::default()
        }
    }
}

fn identify_data_mode(data: &[u8]) -> DataMode {
    if data.iter().all(|e| NUMERIC.contains(e)) {
        DataMode::Numeric
    } else if data
        .iter()
        .all(|e| NUMERIC.contains(e) | ALPHANUMERIC.contains(e))
    {
        DataMode::Alphanumeric
    } else {
        DataMode::Text
    }
}
