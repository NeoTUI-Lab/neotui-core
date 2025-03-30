use neotui_core::components::{Label, VBox};
use neotui_core::component::Component;
use neotui_core::event::Event;

fn main() {
    let label1 = Box::new(Label::new("📦 NeoTUI"));
    let label2 = Box::new(Label::new("🧱 Terminal UI reativa"));
    let label3 = Box::new(Label::new("🚀 Renderizado via VBox"));

    let mut vbox = VBox::with_children(vec![label1, label2, label3]);

    println!("=== DEMO: VBox + Labels ===");
    println!("(render simulado via println!)\n");

    vbox.render();

    println!("\nEvento enviado: Key('a')");
    let result = vbox.on_event(Event::Key('a'));
    println!("Resultado do evento: {:?}", result);
}
