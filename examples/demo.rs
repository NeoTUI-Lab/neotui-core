use neotui_core::components::{Label, VBox, Button};
use neotui_core::component::Component;
use neotui_core::event::{Event, SpecialKey};
use neotui_core::renderer::Renderer;

fn main() {
    let label1 = Box::new(Label::new("NeoTUI"));
    let label2 = Box::new(Label::new("Modern Terminal UI"));

    let button = Box::new(
        Button::new("Click me")
            .on_click(|| println!("[Button] Clicked!")),
    );

    let mut vbox = VBox::with_children(vec![label1, label2, button]);

    let mut renderer = Renderer::new(80, 24);
    renderer.clear();

    let area = renderer.area();
    vbox.render(renderer.buffer_mut(), area);
    renderer.flush();

    let result = vbox.on_event(Event::Special(SpecialKey::Enter));
    println!("\nEvent result: {:?}", result);

    println!("\n[Tab]");
    let _ = vbox.on_event(Event::Special(SpecialKey::Tab));
    renderer.clear();
    vbox.render(renderer.buffer_mut(), area);
    renderer.flush();

    println!("\n[Enter]");
    let result = vbox.on_event(Event::Special(SpecialKey::Enter));
    println!("Event result: {:?}", result);

}
