use crate::component::{Component, EventResult};
use crate::event::{Event, SpecialKey};
use crate::layout::Rect;
use crate::renderer::ScreenBuffer;

pub struct Button {
    pub label: String,
    pub focused: bool,
    pub on_click: Option<Box<dyn FnMut()>>,
}

impl Button {
    pub fn new<T: Into<String>>(label: T) -> Self {
        Self {
            label: label.into(),
            focused: false,
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
        buffer.draw_border(area);
    
        let label = if self.focused {
            format!("> {}", self.label)
        } else {
            self.label.clone()
        };
    
        let x = area.x + (area.width.saturating_sub(label.len() as u16)) / 2;
        let y = area.y + area.height / 2;
        buffer.draw_text(x, y, &label);
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

    fn focus(&mut self) {
        self.focused = true;
    }

    fn blur(&mut self) {
        self.focused = false;
    }

    fn is_focusable(&self) -> bool {
        true
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
