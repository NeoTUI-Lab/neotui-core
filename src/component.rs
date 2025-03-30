use crate::event::Event;

#[derive(Debug)]
pub enum EventResult {
    Consumed,
    Ignored,
    RequestRedraw,
}

pub trait Component {
    fn render(&self, buffer: &mut crate::renderer::ScreenBuffer, area: crate::layout::Rect);
    fn on_event(&mut self, event: Event) -> EventResult;
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Event;

    struct Dummy;

    impl Component for Dummy {
        fn render(&self) {
            println!("Rendering dummy");
        }

        fn on_event(&mut self, event: Event) -> EventResult {
            match event {
                Event::Key('x') => EventResult::Consumed,
                _ => EventResult::Ignored,
            }
        }
    }

    #[test]
    fn test_dummy_component_event() {
        let mut dummy = Dummy;
        let result = dummy.on_event(Event::Key('x'));
        assert!(matches!(result, EventResult::Consumed));

        let result = dummy.on_event(Event::Key('z'));
        assert!(matches!(result, EventResult::Ignored));
    }
}
