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

    #[test]
    fn test_simple_error() {
        assert_eq!(parse("-ERROR\r\n"), Ok(("", RespType::Error("ERROR"))));
        assert!(parse("-ERROR\n\r").is_err());
    }

    #[test]
    fn test_parse_integer() {
        assert_eq!(parse(":1000\r\n"), Ok(("", RespType::Integer(1000))));
        assert_eq!(parse(":-1000\r\n"), Ok(("", RespType::Integer(-1000))));
    }

    #[test]
    fn test_parse_bulk_string() {
        assert_eq!(
            parse("$5\r\nhello\r\n"),
            Ok(("", RespType::BulkString("hello")))
        );

        assert_eq!(
            parse("$0\r\n\r\n"),
            Ok(("", RespType::BulkString("")))
        );
    }
}
