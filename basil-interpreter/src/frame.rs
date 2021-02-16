use basil_frontend::span::Span;

pub struct Frame {
    name: String,
    current_span: Span,
}

impl Frame {
    pub fn new(name: String, current_span: Span) -> Self {
        Frame { name, current_span }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_current_span(&mut self, next_span: &Span) {
        self.current_span = next_span.clone();
    }

    pub fn current_span(&self) -> &Span {
        &self.current_span
    }
}
