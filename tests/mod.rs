extern crate stb_truetype;

mod tests {
    use stb_truetype;

    #[test]
    fn is_font() {
        assert!(stb_truetype::is_font(b"OTTO"));
        assert!(!stb_truetype::is_font(b""));
    }
}
