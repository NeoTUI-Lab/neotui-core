use crate::component::{Component, EventResult};
use crate::event::{Event, SpecialKey};
use crate::layout::Rect;
use crate::renderer::ScreenBuffer;

pub struct Button {
    pub label: String,
    pub on_click: Option<Box<dyn FnMut()>>,
}

impl Button {
    pub fn new<T: Into<String>>(label: T) -> Self {
        Self {
            label: label.into(),
            on_click: None,
        }
    }

    pub fn on_click<F: FnMut() + 'static>(mut self, callback: F) -> Self {
        self.on_click = Some(Box::new(callback));
        self
    }
}

impl Component for Button {
    fn render(&self, buffer: &mut ScreenBuffer, area: Rect) {
        let x = area.x + (area.width.saturating_sub(self.label.len() as u16)) / 2;
        let y = area.y + area.height / 2;
        buffer.draw_text(x, y, &self.label);
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Special(SpecialKey::Enter) | Event::Key(' ') => {
                if let Some(cb) = self.on_click.as_mut() {
                    cb();
                }
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{Event, SpecialKey};

    #[test]
    fn test_button_triggers_on_click() {
        let mut triggered = false;
        let mut button = Button::new("Click me")
            .on_click(|| {
                println!("Button pressed!");
                triggered = true;
            });

        let result = button.on_event(Event::Special(SpecialKey::Enter));
        assert!(matches!(result, EventResult::Consumed));
        assert!(triggered);
    }
}
