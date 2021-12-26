const NUMERIC: &[u8] = &[48, 49, 50, 51, 52, 53, 54, 55, 56, 57];
const ALPHANUMERIC: &[u8] = &[
    32, 36, 37, 42, 43, 45, 46, 47, 58, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
];

// Simple lookup table based on this: https://www.thonky.com/qr-code-tutorial/alphanumeric-table
// table[ACSII_value] = QR_value
pub const ALPHANUMERIC_TABLE: &[u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    36, 0, 0, 0, 37, 38, 0, 0, 0, 0, 39, 40, 0, 41, 42, 43, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 44, 0, 0,
    0, 0, 0, 0, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35,
];

#[derive(Debug, Clone, Copy)]
pub enum DataMode {
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
    pub fn encode(&self) -> u8 {
        match self {
            Self::Numeric => 1,
            Self::Alphanumeric => 2,
            Self::Text => 4,
            Self::Kanji => 8,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorCorrectionLevel {
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

// static functions
pub fn count_length(mode: DataMode, version: u8) -> usize {
    if version <= 9 {
        match mode {
            DataMode::Numeric => 10,
            DataMode::Alphanumeric => 9,
            _ => 8,
        }
    } else if version <= 26 {
        match mode {
            DataMode::Numeric => 12,
            DataMode::Alphanumeric => 11,
            DataMode::Text => 16,
            DataMode::Kanji => 10,
        }
    } else {
        match mode {
            DataMode::Numeric => 14,
            DataMode::Alphanumeric => 13,
            DataMode::Text => 16,
            DataMode::Kanji => 12,
        }
    }
}

pub fn identify_data_mode(data: &[u8]) -> DataMode {
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
