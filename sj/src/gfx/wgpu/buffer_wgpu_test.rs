#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        //このテストを失敗させる
        panic!("Make this test fail");
    }
}
