use std::str::FromStr;

#[derive(Debug)]
pub enum CounterType {
    FillBuffer,
    ReadLine,
    Lines,
    Bytes,
    CSVCrate,
}

impl FromStr for CounterType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fill_buffer" => Ok(Self::FillBuffer),
            "read_line" => Ok(Self::ReadLine),
            "lines" => Ok(Self::Lines),
            "bytes" => Ok(Self::Bytes),
            "csv_crate" => Ok(Self::CSVCrate),
            _ => Err(()),
        }
    }
}

impl Default for CounterType {
    fn default() -> Self {
        Self::FillBuffer
    }
}
