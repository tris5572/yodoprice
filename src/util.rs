// 省略された文字列を返す。
pub fn omitted_string(input: &str) -> String {
    let end = input.char_indices().nth(25);
    match end {
        Some(x) => format!("{}...", &input[0..x.0]),
        None => input[0..].to_string(),
    }
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
}
