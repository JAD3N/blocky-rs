use blocky::net::AsJson;
use blocky::net::chat::{Style, ClickEvent, ClickAction, TextColor};

fn main() {
    let mut style = Style::default();
    style.bold = Some(true);
    style.click_event = Some(ClickEvent::new(
        ClickAction::OpenUrl,
        "this is a test!",
    ));

    let json: serde_json::Value = style.as_json();

    println!("json: {} {:?}", json, TextColor::parse("dark_purple"));
}