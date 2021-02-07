use super::Style;

pub trait ComponentClone {
    fn clone_box(&self) -> Box<dyn Component>;
}

pub trait ComponentContent {
    fn contents(&self) -> &str { "" }
}

pub trait Component: ComponentContent + ComponentClone {
    fn siblings(&self) -> &Vec<Box<dyn Component>> where Self: Sized;
    fn siblings_mut(&mut self) -> &mut Vec<Box<dyn Component>> where Self: Sized;
}

#[macro_export]
macro_rules! component {
    ($svis:vis struct $name:ident { $($fvis:vis $fname:ident: $fty:ty),* $(,)? }) => {
        $svis struct $name {
            siblings: Vec<Box<dyn Component>>,
            style: Style,
            $($fvis $fname: $fty),*
        }

        impl $crate::chat::Component for $name {
            fn siblings(&self) -> &Vec<Box<dyn Component>> {
                self.siblings.as_ref()
            }

            fn siblings_mut(&mut self) -> &mut Vec<Box<dyn Component>> {
                self.siblings.as_mut()
            }
        }
    };
}

component!(pub struct TextComponent {
    test: u8,
});

impl ComponentContent for TextComponent {}

impl ComponentClone for TextComponent {
    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(Self {
            siblings: vec![],
            style: Style::default(),
            test: self.test,
        })
    }
}
