use colored::{
    Color,
    Colorize,
};

#[derive(
    Eq,
    Copy,
    Debug,
    Clone,
    PartialEq,
)] pub enum GbOutputKind {
    Info,
    Error,
    Warning,
    Success,
} impl GbOutputKind {
    fn formatted(&self) -> (fn(&str), &'static str, Color) {
        match self {
            GbOutputKind::Info    =>
                (
                    |m| println!("{}", m),
                    " [I]: ",
                    Color::Blue,
                ),
            GbOutputKind::Error   =>
                (
                    |m| eprintln!("{}", m),
                    " [E]: ",
                    Color::Red,
                ),
            GbOutputKind::Warning =>
                (
                    |m| eprintln!("{}", m),
                    " [W]: ",
                    Color::Yellow,
                ),
            GbOutputKind::Success =>
                (
                    |m| println!("{}", m),
                    " [S]: ",
                    Color::Green,
                ),
        }
    }
}

pub trait GbOutputT: Send + Sync {
    fn kind(&self) -> Option<GbOutputKind>;
    fn message(&self) -> &str;
    fn print(&self) {
        if let Some( (s, p, c) ) = self.kind()
        .map(|k| k.formatted()) {
            s(
                &format!("{}{}", p, self.message(),)
                .color(c).to_string()
            );
        }
    }
}

#[derive(
    Debug,
    Clone,
)] pub struct GbSingleOutput {
    kind: GbOutputKind,
    message: String,
} impl GbSingleOutput {
    pub fn new<M: Into<String>>
    (kind: GbOutputKind, message: M) -> GbSingleOutput {
        Self {
            kind,
            message: message.into(),
        }
    }
} impl GbOutputT for GbSingleOutput {
    fn kind(&self) -> Option<GbOutputKind> {
        Some(self.kind)
    }
    fn message(&self) -> &str {
        &self.message
    }
}

pub struct GbBufferedOutput {
    outputs: Vec<Box<dyn GbOutputT>>,
    message: String
} impl GbBufferedOutput {
    pub fn new() -> GbBufferedOutput {
        Self {
            outputs: vec![],
            message: String::from(
                "GbBufferedOutput(0 elements)"
            ),
        }
    }

    pub fn add<O: GbOutputT + 'static>(&mut self, output: O) {
        self.outputs.push(Box::new(output));
        self.message = format!(
            "GbBufferedOutput({} elements)",
            self.outputs.len(),
        )
    }
} impl Default for GbBufferedOutput {
    fn default() -> GbBufferedOutput {
        Self::new()
    }
} impl GbOutputT for GbBufferedOutput {
    fn kind(&self) -> Option<GbOutputKind> { None }
    fn message(&self) -> &str { &self.message }
    fn print(&self) {
        for o in &self.outputs {
            o.print();
        }
    }
}

pub enum GbOutput {
    Single(GbSingleOutput),
    Buffered(GbBufferedOutput),
} impl GbOutputT for GbOutput {
    fn kind(&self) -> Option<GbOutputKind> {
        match self {
            GbOutput::Single(s)   => s.kind(),
            GbOutput::Buffered(b) => b.kind(),
        }
    }
    fn message(&self) -> &str {
        match self {
            GbOutput::Single(s)   => s.message(),
            GbOutput::Buffered(b) => b.message(),
        }
    }
    fn print(&self) {
        match self {
            GbOutput::Single(s)   => s.print(),
            GbOutput::Buffered(b) => b.print(),
        }
    }
}
