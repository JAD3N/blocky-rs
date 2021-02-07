pub mod chat;

mod test {
    #[test]
    fn chat_components() {
        use crate::chat::TextColor;
        use blocky::ChatFormatting;

        let black: TextColor = ChatFormatting::BLACK.into();

        println!("testing 123: {}", black);
    }
}