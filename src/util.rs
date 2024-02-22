// 省略された文字列を返す。
pub fn omitted_string(input: &str) -> String {
    let end = input.char_indices().nth(25);
    match end {
        Some(x) => format!("{}...", &input[0..x.0]),
        None => input[0..].to_string(),
    }
}

/// 渡された数値にカンマを入れた文字列を返す。
pub fn commafy<T: Into<i128>>(value: T) -> String {
    // 参考 https://stackoverflow.com/questions/26998485/is-it-possible-to-print-a-number-formatted-with-thousand-separator-in-rust
    let val = value.into();
    let mut num = val
        .abs()
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",");
    if val < 0 {
        num = format!("-{num}");
    }
    num
}

#[cfg(test)]
mod test {
    use crate::util::*;

    #[test]
    fn omitted_string_test() {
        assert_eq!("あいうえお", omitted_string("あいうえお"));
        assert_eq!(
            "あいうえおかきくけこさしすせそたちつてと12345...",
            omitted_string("あいうえおかきくけこさしすせそたちつてと123456789")
        );
        assert_eq!(
            "あいうえおかきくけこさしすせそたちつてとなにぬねの...",
            omitted_string("あいうえおかきくけこさしすせそたちつてとなにぬねのまみむめも")
        );
        assert_eq!(
            "あいうえおかきくけこさしすせそたちつてとなにぬねの",
            omitted_string("あいうえおかきくけこさしすせそたちつてとなにぬねの")
        );
    }

    #[test]
    fn name() {
        assert_eq!(commafy(0_u32), "0");
        assert_eq!(commafy(1234), "1,234");
        assert_eq!(commafy(-1), "-1");
        assert_eq!(commafy(-100), "-100");
        assert_eq!(commafy(-1000), "-1,000");
    }
}
