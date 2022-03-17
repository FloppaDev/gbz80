
#[cfg(debug_assertions)]
/// Prints the title for compilation stage.
pub fn title(title: &str) {
    let decoration = "=".repeat(79);
    println!("\n{}\n\t\t\t\t{}\n{}\n", decoration, title, decoration);
}

#[cfg(target_family="unix")]
/// Ansi escapes for colors.
mod values {
    pub const BASE: &'static str = "\x1b[0m";
    pub const OK: &'static str = "\x1b[32m";
    pub const ERR: &'static str = "\x1b[31m";
    pub const INFO: &'static str = "\x1b[93m";
}

#[cfg(not(target_family="unix"))]
/// Assumes that colors will not work.
mod values {
    pub const BASE: &'static str = "";
    pub const OK: &'static str = "";
    pub const ERR: &'static str = "";
    pub const INFO: &'static str = "";
}

/// Creates a `Strip` object.
pub fn strip() -> Strip {
    Strip{ value: "".into() }
}

/// Provides a builder pattern for chaining different colors.
pub struct Strip {
    value: String,
}

impl Strip {

    /// Colors text in white.
    pub fn base(mut self, text: &str) -> Self {
        self.value.push_str(text);
        self
    }

    /// Colors text in green.
    pub fn ok(mut self, text: &str) -> Self {
        self.value.push_str(values::OK);
        self.value.push_str(text);
        self.value.push_str(values::BASE);
        self
    }

    /// Colors text in red.
    pub fn err(mut self, text: &str) -> Self {
        self.value.push_str(values::ERR);
        self.value.push_str(text);
        self.value.push_str(values::BASE);
        self
    }

    /// Colors text in yellow.
    pub fn info(mut self, text: &str) -> Self {
        self.value.push_str(values::INFO);
        self.value.push_str(text);
        self.value.push_str(values::BASE);
        self
    }

    /// Drops the `Strip` and returns its `String` value.
    pub fn end(mut self) -> String {
        self.value
    }

}
