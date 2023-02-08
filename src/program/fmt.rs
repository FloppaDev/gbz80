
use values::*;

#[cfg(debug_assertions)]
/// Prints the title for compilation stage.
pub fn title(title: &str) {
    let decoration = "=".repeat(80);
    println!("\n{decoration}\n\t\t\t\t{title}\n{decoration}\n");
}

/// Prepends a newline if text is not empty.
pub fn ln_if(text: &str) -> String {
    return if text.is_empty() { String::new() }else{ format!("\n{text}") };
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
    pub const FAINT: &str = "";
    pub const BOLD: &str = "";
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

    /// Consumes the `Strip` and returns its `String` value.
    // "constant functions cannot evaluate destructors"
    #[allow(clippy::missing_const_for_fn)]
    pub fn read(self) -> String { self.value }

    pub fn base(self, text: &str) -> Self { self.push(BASE).push(text) }
    pub fn ok(self, text: &str) -> Self { self.wrap(text, OK) }
    pub fn err(self, text: &str) -> Self { self.wrap(text, ERR) }
    pub fn info(self, text: &str) -> Self { self.wrap(text, INFO) }
    pub fn faint(self, text: &str) -> Self { self.wrap(text, FAINT) }
    pub fn bold(self, text: &str) -> Self { self.wrap(text, BOLD) }

    /// Appends text when in debug mode, with base color.
    pub fn debug(self, text: &str) -> Self {
        return if cfg!(debug_assertions) { self.base(text) }else{ self };
    }

    fn push(mut self, text: &str) -> Self {
        self.value.push_str(text); 
        self
    }

    fn wrap(self, text: &str, with: &str) -> Self {
        self.push(with).push(text).push(BASE)        
    }

}
