use serde::Serialize;
use std::str::FromStr;

#[derive(Serialize, Debug, PartialEq, Eq, Hash)]
pub enum ResultType {
    Lines,
    CsvRecords,
}

impl FromStr for ResultType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lines" | "l" => Ok(ResultType::Lines),
            "records" | "r" => Ok(ResultType::CsvRecords),
            _ => Err(()),
        }
    }
}
