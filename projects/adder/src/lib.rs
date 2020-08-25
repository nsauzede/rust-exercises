#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test panic");
    }
}
