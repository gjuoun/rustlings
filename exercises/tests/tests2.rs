// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail!
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a hint.

#[derive(Debug, PartialEq)]
pub struct Jun(i64);

#[cfg(test)]
mod tests {
    use crate::Jun;

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(Jun(1), Jun(1));
    }
}
