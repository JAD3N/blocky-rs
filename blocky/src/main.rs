use blocky::net::AsJson;
use blocky::net::chat::{Style, ClickEvent, ClickAction, TextColor, TextComponent, Component};

fn main() {
    let mut component = TextComponent::new("this is a test!");
    component.style_mut().bold = Some(true);
    component.style_mut().click_event = Some(ClickEvent::new(
        ClickAction::OpenUrl,
        "this is a test!",
    ));

    println!("json: {}", component.as_json());
}