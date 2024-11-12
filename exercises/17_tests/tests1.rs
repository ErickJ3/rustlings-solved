// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use crate::is_even;
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.

    #[test]
    fn you_can_assert() {
        // TODO: Test the function `is_even` with some values.
        assert_eq!(true, is_even(10));
        assert_eq!(false, is_even(1));
        assert_eq!(true, is_even(2));
        assert_eq!(false, is_even(3));
        assert_eq!(true, is_even(4));
        assert_eq!(false, is_even(5));
    }
}
