use super::*;

pub struct CSVCrateCounter;

impl Counter for CSVCrateCounter {
    fn count_by_plain_text(reader: BufReader<File>) -> HashMap<ResultType, u64> {
        Self::count_by_csv(reader)
    }

    fn count_by_csv(reader: BufReader<File>) -> HashMap<ResultType, u64> {
        let mut r = 0;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .escape(Some(b'\\'))
            .from_reader(reader);
        for _ in reader.records() {
            r += 1;
        }
        vec![(ResultType::CSVRecords, r)]
            .into_iter()
            .collect::<HashMap<_, _>>()
    }
}
