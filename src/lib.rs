pub mod parser;

#[cfg(test)]
mod tests {
    use parser::{parse, RespType};

    use super::*;

    #[test]
    fn test_simple_string() {
        assert_eq!(parse("+OK\r\n"), Ok(("", RespType::SimpleString("OK"))));
        assert!(parse("+O\nK\r\n").is_err());
        assert!(parse("+OK\n\r").is_err());
    }
}
