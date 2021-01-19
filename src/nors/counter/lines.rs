use super::*;
use std::io::BufRead;

pub struct LinesCounter;

impl Counter for LinesCounter {
    fn count_by_plain_text(reader: BufReader<File>) -> HashMap<ResultType, u64> {
        vec![(ResultType::Lines, reader.lines().count() as u64)]
            .into_iter()
            .collect::<HashMap<_, _>>()
    }

    fn count_by_csv(reader: BufReader<File>) -> HashMap<ResultType, u64> {
        let mut l = 0; // Linesのカウンター
        let mut r = 0; // Recordsのカウンター
        let mut multi = false; // 複数行のレコードの走査中か
                               //let mut lines = Vec::new();
                               //let mut lines2 = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            l += 1;
            if Self::is_even((line.matches('\"').count() - line.matches("\\\"").count()) as u8) {
                //lines.push(l);
                // double quotation の個数から、エスケープされた double quotationの数を引いて、
                // 純粋な double quotation の個数が偶数だったら１行で完結したレコード
                //                                 奇数だったら複数行
                if !multi {
                    r += 1;
                }
            } else if multi {
                //lines2.push(l);
                // double quotationの数が奇数で複数行のレコードの走査中なら、１レコードの終わり
                r += 1;
                multi = false;
            } else {
                // double quotationの数が奇数なので複数行のレコードの走査開始
                multi = true;
            }
        }
        //println!("even: {:?}", lines);
        //println!("odd: {:?}", lines2);
        vec![(ResultType::Lines, l), (ResultType::CSVRecords, r)]
            .into_iter()
            .collect::<HashMap<_, _>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod count_by_csv {
        use super::*;
        use crate::nors::counter::tests::*;

        #[test]
        fn test() {
            single_line::line_1(LinesCounter::count_by_csv);
            single_line::line_10(LinesCounter::count_by_csv);
            multi_line::record_1(LinesCounter::count_by_csv);
            multi_line::record_10(LinesCounter::count_by_csv);
            includes_double_quotations_escape::line_1(LinesCounter::count_by_csv);
            includes_double_quotations_escape::line_10(LinesCounter::count_by_csv);
            includes_double_quotations_escape::record_1(LinesCounter::count_by_csv);
            includes_double_quotations_escape::record_10(LinesCounter::count_by_csv);
            includes_backslash_escape::line_1(LinesCounter::count_by_csv);
            includes_backslash_escape::line_10(LinesCounter::count_by_csv);
            includes_backslash_escape::record_1(LinesCounter::count_by_csv);
            includes_backslash_escape::record_10(LinesCounter::count_by_csv);
        }
    }
}
