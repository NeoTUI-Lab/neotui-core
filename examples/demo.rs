use neotui_core::components::{Label, VBox};
use neotui_core::component::Component;
use neotui_core::event::Event;
use neotui_core::renderer::Renderer;

fn main() {
    let label1 = Box::new(Label::new("NeoTUI"));
    let label2 = Box::new(Label::new("Modern Terminal UI"));
    let label3 = Box::new(Label::new("Rendered via ScreenBuffer"));

    let mut vbox = VBox::with_children(vec![label1, label2, label3]);

    let mut renderer = Renderer::new(80, 24);
    renderer.clear();

    let area = renderer.area();
    vbox.render(renderer.buffer_mut(), area);

    renderer.flush();

    let result = vbox.on_event(Event::Key('a'));
    println!("\nEvent result: {:?}", result);
}
