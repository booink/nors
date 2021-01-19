use super::*;
use std::io::BufRead;

pub struct ReadLineCounter;

impl Counter for ReadLineCounter {
    fn count_by_plain_text(mut reader: BufReader<File>) -> HashMap<ResultType, u64> {
        let mut buffer = String::new();
        let mut l = 0;
        while reader.read_line(&mut buffer).unwrap() > 0 {
            l += 1;
            buffer.clear();
        }
        vec![(ResultType::Lines, l)]
            .into_iter()
            .collect::<HashMap<_, _>>()
    }

    fn count_by_csv(mut reader: BufReader<File>) -> HashMap<ResultType, u64> {
        let mut buffer = String::new();
        let mut l = 0; // Linesのカウンター
        let mut r = 0; // Recordsのカウンター
        let mut multi = false; // 複数行のレコードの走査中か
                               //let mut lines = Vec::new();
                               //let mut lines2 = Vec::new();

        //let mut a = 0;
        //let mut invalid_count = 0;
        while reader.read_line(&mut buffer).unwrap() > 0 {
            l += 1;
            let dq_count = buffer.matches('\"').count();
            let qq_count = buffer.matches("\\\"").count();
            let dq = (dq_count - qq_count) as u8;

            if Self::is_even(dq) {
                //lines.push(l);
                // double quotation の個数から、エスケープされた double quotationの数を引いて、
                // 純粋な double quotation の個数が偶数だったら１行で完結したレコード
                //                                 奇数だったら複数行
                if multi {
                    //a += dq;
                    // double quotationが偶数個の行で、複数行に跨るレコードの場合は、
                    // 途中の行に double quotation で囲まれた値があるはずなので何もしない
                } else {
                    if dq != 10 {
                        //invalid_count += 1;
                        //println!("dq: {}; dq_count: {}, qq_count: {}", dq, dq_count, qq_count);
                    }
                    //a = 0;
                    // double quotationが偶数個の行で、１行のレコード
                    multi = false;
                    r += 1;
                }
            } else if multi {
                if dq != 10 {
                    //invalid_count += 1;
                    //println!("dq: {}; dq_count: {}, qq_count: {}", dq, dq_count, qq_count);
                }
                //a = 0;
                //lines2.push(l);
                // double quotationの数が奇数で複数行のレコードの走査中なら、１レコードの終わり
                r += 1;
                multi = false;
            } else {
                //a += dq;
                // double quotationの数が奇数なので複数行のレコードの走査開始
                multi = true;
            }
            buffer.clear();
        }
        //println!("invalid_count: {}", invalid_count);
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
            single_line::line_1(ReadLineCounter::count_by_csv);
            single_line::line_10(ReadLineCounter::count_by_csv);
            multi_line::record_1(ReadLineCounter::count_by_csv);
            multi_line::record_10(ReadLineCounter::count_by_csv);
            includes_double_quotations_escape::line_1(ReadLineCounter::count_by_csv);
            includes_double_quotations_escape::line_10(ReadLineCounter::count_by_csv);
            includes_double_quotations_escape::record_1(ReadLineCounter::count_by_csv);
            includes_double_quotations_escape::record_10(ReadLineCounter::count_by_csv);
            includes_backslash_escape::line_1(ReadLineCounter::count_by_csv);
            includes_backslash_escape::line_10(ReadLineCounter::count_by_csv);
            includes_backslash_escape::record_1(ReadLineCounter::count_by_csv);
            includes_backslash_escape::record_10(ReadLineCounter::count_by_csv);
        }
    }
}
