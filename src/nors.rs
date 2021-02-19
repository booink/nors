use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

mod counter;
mod counter_type;
mod file_type;
mod output_type;
pub mod result_type;
use crate::nors::counter::{
    BytesCounter, CSVCrateCounter, Counter, FillBufferCounter, LinesCounter, ReadLineCounter,
};
pub use crate::nors::counter_type::CounterType;
use crate::nors::file_type::FileType;
pub use crate::nors::output_type::OutputType;
use crate::nors::result_type::ResultType;

#[derive(Serialize, Debug)]
pub struct ResultsByCountType {
    pub results: HashMap<ResultType, u64>,
}

impl ResultsByCountType {
    pub fn print(&self, output_type: OutputType) {
        output_type.print(self);
    }

    pub fn print_by_count_type(&self, result_type: ResultType) {
        println!("{:?}", self.results.get(&result_type).unwrap());
    }
}

pub struct Nors<'a> {
    path: &'a str,
    file_type: FileType,
}

impl<'a> Nors<'a> {
    pub fn new(path: &'a str) -> Self {
        Nors {
            path,
            file_type: FileType::from_path(path),
        }
    }

    pub fn file_type(&mut self, t: FileType) {
        self.file_type = t;
    }

    fn reader(&self) -> anyhow::Result<BufReader<File>> {
        let f = File::open(self.path)?;
        Ok(BufReader::new(f))
    }

    pub fn count(&self) -> anyhow::Result<ResultsByCountType> {
        self.count_by_type(CounterType::default())
    }

    pub fn count_by_type(&self, counter_type: CounterType) -> anyhow::Result<ResultsByCountType> {
        let reader = self.reader()?;
        let results = match counter_type {
            CounterType::FillBuffer => FillBufferCounter::count(&self.file_type, reader),
            CounterType::ReadLine => ReadLineCounter::count(&self.file_type, reader),
            CounterType::Lines => LinesCounter::count(&self.file_type, reader),
            CounterType::Bytes => BytesCounter::count(&self.file_type, reader),
            CounterType::CSVCrate => CSVCrateCounter::count(&self.file_type, reader),
        };
        Ok(ResultsByCountType { results })
    }
}
