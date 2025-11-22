/// Custom filters for Askama templates
/// Provides number formatting and other utility functions

/// Format a number with thousand separators
/// Example: 1000 -> "1,000"
pub fn format_number(num: &i32) -> askama::Result<String> {
    let num_str = num.to_string();
    let mut result = String::new();
    let mut count = 0;

    for c in num_str.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
        count += 1;
    }

    Ok(result.chars().rev().collect())
}

/// Format a number with thousand separators (i64 variant)
pub fn format_number_i64(num: &i64) -> askama::Result<String> {
    let num_str = num.to_string();
    let mut result = String::new();
    let mut count = 0;

    for c in num_str.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
        count += 1;
    }

    Ok(result.chars().rev().collect())
}

/// Format a number with thousand separators (usize variant)
pub fn format_number_usize(num: &usize) -> askama::Result<String> {
    let num_str = num.to_string();
    let mut result = String::new();
    let mut count = 0;

    for c in num_str.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
        count += 1;
    }

    Ok(result.chars().rev().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(&0).unwrap(), "0");
        assert_eq!(format_number(&100).unwrap(), "100");
        assert_eq!(format_number(&1000).unwrap(), "1,000");
        assert_eq!(format_number(&10000).unwrap(), "10,000");
        assert_eq!(format_number(&1000000).unwrap(), "1,000,000");
    }

    #[test]
    fn test_format_number_i64() {
        assert_eq!(format_number_i64(&0).unwrap(), "0");
        assert_eq!(format_number_i64(&1000).unwrap(), "1,000");
        assert_eq!(format_number_i64(&1000000).unwrap(), "1,000,000");
    }

    #[test]
    fn test_format_number_usize() {
        assert_eq!(format_number_usize(&0).unwrap(), "0");
        assert_eq!(format_number_usize(&1000).unwrap(), "1,000");
        assert_eq!(format_number_usize(&1000000).unwrap(), "1,000,000");
    }
}
