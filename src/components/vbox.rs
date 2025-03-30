use crate::component::{Component, EventResult};
use crate::event::Event;
use crate::layout::Rect;

/// Um componente que empilha seus filhos verticalmente.
pub struct VBox {
    pub children: Vec<Box<dyn Component>>,
}

impl VBox {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn with_children(children: Vec<Box<dyn Component>>) -> Self {
        Self { children }
    }

    pub fn add_child(&mut self, child: Box<dyn Component>) {
        self.children.push(child);
    }
}

impl Component for VBox {
    fn render(&self) {
        let root_area = Rect::new(0, 0, 80, 24); // área de exemplo (fixa por enquanto)
        let areas = root_area.split_vertically(self.children.len());

        for (child, area) in self.children.iter().zip(areas.iter()) {
            println!("--- Rendering child at {:?}", area);
            child.render();
        }
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        for child in self.children.iter_mut() {
            let result = child.on_event(event.clone());
            if let EventResult::Consumed = result {
                return EventResult::Consumed;
            }
        }
        EventResult::Ignored
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::Label;
    use crate::event::Event;

    #[test]
    fn test_vbox_render() {
        let label1 = Box::new(Label::new("One"));
        let label2 = Box::new(Label::new("Two"));

        let vbox = VBox::with_children(vec![label1, label2]);
        vbox.render(); // Simplesmente imprime por enquanto
    }

    #[test]
    fn test_vbox_event_propagation() {
        struct Echo;
        impl Component for Echo {
            fn render(&self) {}
            fn on_event(&mut self, event: Event) -> EventResult {
                if let Event::Key('x') = event {
                    EventResult::Consumed
                } else {
                    EventResult::Ignored
                }
            }
        }

        let echo = Box::new(Echo);
        let mut vbox = VBox::with_children(vec![echo]);

        let result = vbox.on_event(Event::Key('x'));
        assert!(matches!(result, EventResult::Consumed));

        let result = vbox.on_event(Event::Key('z'));
        assert!(matches!(result, EventResult::Ignored));
    }
}
