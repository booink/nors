use super::*;
use std::io::Read;

pub struct BytesCounter;

impl Counter for BytesCounter {
    fn count_by_plain_text(reader: BufReader<File>) -> HashMap<ResultType, u64> {
        let mut l = 0;
        let lf = &b'\n';
        for byte in reader.bytes() {
            if &byte.unwrap() == lf {
                l += 1;
            }
        }
        vec![(ResultType::Lines, l)]
            .into_iter()
            .collect::<HashMap<_, _>>()
    }

    fn count_by_csv(reader: BufReader<File>) -> HashMap<ResultType, u64> {
        let mut l = 0; // Linesのカウンター
        let mut r = 0; // Recordsのカウンター
                       // １行内の double quotation の数が奇数かどうか
                       // 偶数ならレコードが１行で構成されている
                       // 奇数なら改行などで複数行にまたがっている
                       // 次の奇数の行がレコードの終わりの行
        let mut q = true;
        let lf = &b'\n';
        let quotation = &b'"';
        let escape = &b'\\';
        let mut escaped = false;

        /*
        // デバッグ用
        let mut q_count = 0;
        let mut record_ends = Vec::new();
        let mut invalids = Vec::new();
        */

        for b in reader.bytes() {
            let b = &b.unwrap();
            if b == lf {
                l += 1; // 改行があれば Lines をインクリメント
                if q {
                    /*
                    record_ends.push(l);
                    if q_count != 10 {
                        invalids.push(l);
                    }
                    q_count = 0;
                    */
                    // double quotation が偶数個なら Records をインクリメント
                    r += 1;
                }
            } else if b == quotation && !escaped {
                // double quotationで前の文字がescapeじゃないときはクオートと判断
                q = !q;
                //q_count += 1;
            }
            escaped = if b == escape {
                // バックスラッシュが２つ並んだ場合はescapeを解除
                // 一つのときは true
                !escaped
            } else {
                false
            }
        }
        if !q {
            // 最後のレコードが中途半端に終わっている場合は、１レコードあると見なす
            r += 1;
        }

        //println!("even: {:?}", lines);
        //println!("odd: {:?}", lines2);
        vec![(ResultType::Lines, l), (ResultType::CsvRecords, r)]
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
            single_line::line_1(BytesCounter::count_by_csv);
            single_line::line_10(BytesCounter::count_by_csv);
            multi_line::record_1(BytesCounter::count_by_csv);
            multi_line::record_10(BytesCounter::count_by_csv);
            includes_double_quotations_escape::line_1(BytesCounter::count_by_csv);
            includes_double_quotations_escape::line_10(BytesCounter::count_by_csv);
            includes_double_quotations_escape::record_1(BytesCounter::count_by_csv);
            includes_double_quotations_escape::record_10(BytesCounter::count_by_csv);
            includes_backslash_escape::line_1(BytesCounter::count_by_csv);
            includes_backslash_escape::line_10(BytesCounter::count_by_csv);
            includes_backslash_escape::record_1(BytesCounter::count_by_csv);
            includes_backslash_escape::record_10(BytesCounter::count_by_csv);
        }
    }
}
