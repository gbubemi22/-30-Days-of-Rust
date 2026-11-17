/// Returns the square of `number`.
///
/// # Examples
///
/// ```
/// assert_eq!(quality_kit::square(4), 16);
/// ```
pub fn square(number: i32) -> i32 {
    number * number
}

#[cfg(test)]
mod tests {
    use super::square;

    #[test]
    fn squares_zero_and_positive_values() {
        assert_eq!(square(0), 0);
        assert_eq!(square(5), 25);
    }
}
