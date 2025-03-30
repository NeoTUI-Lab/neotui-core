use crate::component::{Component, EventResult};
use crate::event::Event;

pub struct Label {
    pub text: String,
}

impl Label {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self {
            text: text.into(),
        }
    }
}

impl Component for Label {
    fn render(&self) {
        println!("{}", self.text);
    }

    fn on_event(&mut self, _event: Event) -> EventResult {
        EventResult::Ignored
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Event;

    #[test]
    fn test_label_creation() {
        let label = Label::new("Hello, NeoTUI!");
        assert_eq!(label.text, "Hello, NeoTUI!");
    }

    #[test]
    fn test_label_event_is_ignored() {
        let mut label = Label::new("Test");
        let result = label.on_event(Event::Key('a'));
        assert!(matches!(result, EventResult::Ignored));
    }
}
