use super::*;
use std::io::BufRead;

pub struct FillBufferCounter;

impl Counter for FillBufferCounter {
    fn count_by_plain_text(mut reader: BufReader<File>) -> HashMap<ResultType, u64> {
        let mut buffer = reader.fill_buf().unwrap();
        let mut buf_len = buffer.len();
        let mut l = 0; // Linesのカウンター
        let lf = &b'\n';

        while buf_len > 0 {
            for b in buffer {
                if b == lf {
                    l += 1; // 改行があれば Lines をインクリメント
                }
            }
            reader.consume(buf_len);
            buffer = reader.fill_buf().unwrap();
            buf_len = buffer.len();
        }

        vec![(ResultType::Lines, l)]
            .into_iter()
            .collect::<HashMap<_, _>>()
    }

    fn count_by_csv(mut reader: BufReader<File>) -> HashMap<ResultType, u64> {
        let mut buffer = reader.fill_buf().unwrap();
        let mut buf_len = buffer.len();
        let mut l = 0; // Linesのカウンター
        let mut r = 0; // Recordsのカウンター
                       // １行内の double quotation の数が奇数かどうか
                       // 偶数ならレコードが１行で構成されている
                       // 奇数なら改行などで複数行にまたがっている
                       // 次の奇数の行がレコードの終わりの行
        let mut quoting = true; // quote の間かどうか
        let mut escaped = false; // 前の char が escape 文字かどうか
        let lf = &b'\n';
        let quotation = &b'"';
        let escape = &b'\\';

        // デバッグ用
        let mut q_count = 0;
        let mut record_ends = Vec::new();
        let mut invalids = Vec::new();

        while buf_len > 0 {
            for b in buffer {
                if b == lf {
                    l += 1; // 改行があれば Lines をインクリメント
                    if quoting {
                        if cfg!(debug_assertions) {
                            record_ends.push(l);
                            if q_count != 10 {
                                invalids.push(l);
                            }
                            q_count = 0;
                        }

                        r += 1; // double quotation が偶数個なら Records をインクリメント
                    }
                } else if b == quotation && !escaped {
                    // double quotationで前の文字がescapeじゃないときはクオートと判断
                    quoting = !quoting;
                    if cfg!(debug_assertions) {
                        q_count += 1;
                    }
                }
                escaped = if b == escape {
                    // バックスラッシュが２つ並んだ場合はescapeを解除
                    // 一つのときは true
                    !escaped
                } else {
                    false
                };
            }
            reader.consume(buf_len);
            buffer = reader.fill_buf().unwrap();
            buf_len = buffer.len();
        }
        if !quoting {
            // 最後のレコードが中途半端に終わっている場合は、１レコードあると見なす
            r += 1;
        }
        if cfg!(debug_assertions) {
            println!("{:?}", record_ends);
            println!("invalids: {:?}", invalids[0]);
        }

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
            single_line::line_1(FillBufferCounter::count_by_csv);
            single_line::line_10(FillBufferCounter::count_by_csv);
            multi_line::record_1(FillBufferCounter::count_by_csv);
            multi_line::record_10(FillBufferCounter::count_by_csv);
            includes_double_quotations_escape::line_1(FillBufferCounter::count_by_csv);
            includes_double_quotations_escape::line_10(FillBufferCounter::count_by_csv);
            includes_double_quotations_escape::record_1(FillBufferCounter::count_by_csv);
            includes_double_quotations_escape::record_10(FillBufferCounter::count_by_csv);
            includes_backslash_escape::line_1(FillBufferCounter::count_by_csv);
            includes_backslash_escape::line_10(FillBufferCounter::count_by_csv);
            includes_backslash_escape::record_1(FillBufferCounter::count_by_csv);
            includes_backslash_escape::record_10(FillBufferCounter::count_by_csv);
        }
    }
}
