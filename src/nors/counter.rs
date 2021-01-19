pub mod bytes;
pub mod csv_crate;
pub mod fill_buffer;
pub mod lines;
pub mod read_line;

pub use bytes::BytesCounter;
pub use csv_crate::CSVCrateCounter;
pub use fill_buffer::FillBufferCounter;
pub use lines::LinesCounter;
pub use read_line::ReadLineCounter;

use crate::nors::file_type::FileType;
use crate::nors::result_type::ResultType;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub trait Counter {
    fn count(file_type: &FileType, reader: BufReader<File>) -> HashMap<ResultType, u64> {
        match file_type {
            FileType::PlainText => Self::count_by_plain_text(reader),
            FileType::CSV => Self::count_by_csv(reader),
        }
    }
    fn count_by_plain_text(reader: BufReader<File>) -> HashMap<ResultType, u64>;
    fn count_by_csv(reader: BufReader<File>) -> HashMap<ResultType, u64>;
    fn is_odd(n: u8) -> bool {
        n & 1 != 0
    }
    fn is_even(n: u8) -> bool {
        !Self::is_odd(n)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::path::Path;

    pub fn reader(file_name: &str) -> BufReader<File> {
        let path = Path::new("test_resources/").join(file_name);
        let f = File::open(path).expect("File open error.");
        BufReader::new(f)
    }

    pub mod single_line {
        use super::*;

        pub fn line_1<F>(func: F)
        where
            F: Fn(BufReader<File>) -> HashMap<ResultType, u64>,
        {
            let result = func(reader("l1-r1.csv"));
            assert_eq!(result[&ResultType::Lines], 1);
            assert_eq!(result[&ResultType::CSVRecords], 1);
        }

        pub fn line_10<F>(func: F)
        where
            F: Fn(BufReader<File>) -> HashMap<ResultType, u64>,
        {
            let result = func(reader("l10-r10.csv"));
            assert_eq!(result[&ResultType::Lines], 10);
            assert_eq!(result[&ResultType::CSVRecords], 10);
        }
    }

    pub mod multi_line {
        use super::*;

        pub fn record_1<F>(func: F)
        where
            F: Fn(BufReader<File>) -> HashMap<ResultType, u64>,
        {
            let result = func(reader("l2-r1.csv"));
            assert_eq!(result[&ResultType::Lines], 2);
            assert_eq!(result[&ResultType::CSVRecords], 1);
        }

        pub fn record_10<F>(func: F)
        where
            F: Fn(BufReader<File>) -> HashMap<ResultType, u64>,
        {
            let result = func(reader("l20-r10.csv"));
            assert_eq!(result[&ResultType::Lines], 20);
            assert_eq!(result[&ResultType::CSVRecords], 10);
        }
    }

    pub mod includes_double_quotations_escape {
        use super::*;

        pub fn line_1<F>(func: F)
        where
            F: Fn(BufReader<File>) -> HashMap<ResultType, u64>,
        {
            let result = func(reader("l1-r1-dq.csv"));
            assert_eq!(result[&ResultType::Lines], 1);
            assert_eq!(result[&ResultType::CSVRecords], 1);
        }

        pub fn line_10<F>(func: F)
        where
            F: Fn(BufReader<File>) -> HashMap<ResultType, u64>,
        {
            let result = func(reader("l10-r10-dq.csv"));
            assert_eq!(result[&ResultType::Lines], 10);
            assert_eq!(result[&ResultType::CSVRecords], 10);
        }

        pub fn record_1<F>(func: F)
        where
            F: Fn(BufReader<File>) -> HashMap<ResultType, u64>,
        {
            let result = func(reader("l2-r1-dq.csv"));
            assert_eq!(result[&ResultType::Lines], 2);
            assert_eq!(result[&ResultType::CSVRecords], 1);
        }

        pub fn record_10<F>(func: F)
        where
            F: Fn(BufReader<File>) -> HashMap<ResultType, u64>,
        {
            let result = func(reader("l20-r10-dq.csv"));
            assert_eq!(result[&ResultType::Lines], 20);
            assert_eq!(result[&ResultType::CSVRecords], 10);
        }
    }

    pub mod includes_backslash_escape {
        use super::*;

        pub fn line_1<F>(func: F)
        where
            F: Fn(BufReader<File>) -> HashMap<ResultType, u64>,
        {
            let result = func(reader("l1-r1-bs.csv"));
            assert_eq!(result[&ResultType::Lines], 1);
            assert_eq!(result[&ResultType::CSVRecords], 1);
        }

        pub fn line_10<F>(func: F)
        where
            F: Fn(BufReader<File>) -> HashMap<ResultType, u64>,
        {
            let result = func(reader("l10-r10-bs.csv"));
            assert_eq!(result[&ResultType::Lines], 10);
            assert_eq!(result[&ResultType::CSVRecords], 10);
        }

        pub fn record_1<F>(func: F)
        where
            F: Fn(BufReader<File>) -> HashMap<ResultType, u64>,
        {
            let result = func(reader("l2-r1-bs.csv"));
            assert_eq!(result[&ResultType::Lines], 2);
            assert_eq!(result[&ResultType::CSVRecords], 1);
        }

        pub fn record_10<F>(func: F)
        where
            F: Fn(BufReader<File>) -> HashMap<ResultType, u64>,
        {
            let result = func(reader("l20-r10-bs.csv"));
            assert_eq!(result[&ResultType::Lines], 20);
            assert_eq!(result[&ResultType::CSVRecords], 10);
        }
    }
}
