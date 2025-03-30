pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Renderer
    }

    pub fn render(&self) {
        println!("Rendering...");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_new() {
        let r = Renderer::new();
        r.render(); // Aqui validamos que não dá erro
    }
}
