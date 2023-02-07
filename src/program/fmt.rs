
#![allow(dead_code)]

#[cfg(debug_assertions)]
/// Prints the title for compilation stage.
pub fn title(title: &str) {
    let decoration = "=".repeat(79);
    println!("\n{decoration}\n\t\t\t\t{title}\n{decoration}\n");
}

#[cfg(target_family="unix")]
/// Ansi escapes for colors.
mod values {
    pub const BASE: &str = "\x1b[0m";
    pub const OK: &str = "\x1b[32;1m";
    pub const ERR: &str = "\x1b[31;1m";
    pub const INFO: &str = "\x1b[93;1m";
    pub const FAINT: &str = "\x1b[2m";
    pub const BOLD: &str = "\x1b[1m";
}

#[cfg(not(target_family="unix"))]
/// Assumes that these will not work.
mod values {
    pub const BASE: &str = "";
    pub const OK: &str = "";
    pub const ERR: &str = "";
    pub const INFO: &str = "";
}

/// Creates a `Strip` object.
pub const fn strip() -> Strip {
    Strip{ value: String::new() }
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

    /// Appends text when in debug mode, with base color.
    pub fn debug(mut self, text: &str) -> Self {
        if cfg!(debug_assertions) {
            self.value.push_str(text);
        }

        self
    }

    /// Appends dimmed text.
    pub fn faint(mut self, text: &str) -> Self {
        self.value.push_str(values::FAINT);
        self.value.push_str(text);
        self.value.push_str(values::BASE);

        self
    }

    /// Appends bold text.
    pub fn bold(mut self, text: &str) -> Self {
        self.value.push_str(values::BOLD);
        self.value.push_str(text);
        self.value.push_str(values::BASE);

        self
    }

    /// Consumes the `Strip` and returns its `String` value.
    // "constant functions cannot evaluate destructors"
    #[allow(clippy::missing_const_for_fn)]
    pub fn read(self) -> String {
        self.value
    }

}
