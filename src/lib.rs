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

        assert_eq!(parse("$0\r\n\r\n"), Ok(("", RespType::BulkString(""))));
    }

    #[test]
    fn test_parse_array() {
        let input = "*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n";
        let expected = RespType::Array(vec![
            RespType::BulkString("foo"),
            RespType::BulkString("bar"),
        ]);
        let result = parse(input);
        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_nested_array() {
        let input = "*2\r\n*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$3\r\nbaz\r\n";
        let expected = RespType::Array(vec![
            RespType::Array(vec![
                RespType::BulkString("foo"),
                RespType::BulkString("bar"),
            ]),
            RespType::BulkString("baz"),
        ]);
        let result = parse(input);
        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_array_with_different_types() {
        let input = "*3\r\n+OK\r\n:1000\r\n$-1\r\n";
        let expected = RespType::Array(vec![
            RespType::SimpleString("OK"),
            RespType::Integer(1000),
            RespType::Null,
        ]);
        let result = parse(input);
        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_empty_array() {
        let input = "*0\r\n";
        let expected = RespType::Array(vec![]);
        let result = parse(input);
        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_null() {
        let input = "_\r\n";
        let expected = RespType::Null;
        let result = parse(input);
        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_boolean() {
        assert_eq!(parse("#t\r\n"), Ok(("", RespType::Boolean(true))));
        assert_eq!(parse("#f\r\n"), Ok(("", RespType::Boolean(false))));
        assert!(parse("#s\r\n").is_err());
    }

    #[test]
    fn test_parse_double() {
        assert_eq!(
            parse(",1.23\r\n"),
            Ok(("", RespType::Double("1.23".to_string())))
        );
    }
}
