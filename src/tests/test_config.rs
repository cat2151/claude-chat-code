#[cfg(test)]
mod config_tests {
    use crate::config::parse_duration;
    use std::time::Duration;

    #[test]
    fn parse_ms_suffix() {
        assert_eq!(parse_duration("500ms"), Some(Duration::from_millis(500)));
        assert_eq!(parse_duration("1500ms"), Some(Duration::from_millis(1500)));
        assert_eq!(parse_duration("100ms"), Some(Duration::from_millis(100)));
    }

    #[test]
    fn parse_s_suffix_integer() {
        assert_eq!(parse_duration("1s"), Some(Duration::from_millis(1000)));
        assert_eq!(parse_duration("2s"), Some(Duration::from_millis(2000)));
    }

    #[test]
    fn parse_s_suffix_float() {
        assert_eq!(parse_duration("0.5s"), Some(Duration::from_millis(500)));
        assert_eq!(parse_duration("1.5s"), Some(Duration::from_millis(1500)));
    }

    #[test]
    fn parse_with_whitespace() {
        assert_eq!(parse_duration("  500ms  "), Some(Duration::from_millis(500)));
        assert_eq!(parse_duration(" 1s "),      Some(Duration::from_millis(1000)));
    }

    #[test]
    fn parse_invalid_returns_none() {
        assert_eq!(parse_duration(""), None);
        assert_eq!(parse_duration("fast"), None);
        assert_eq!(parse_duration("500"), None);
    }
}
