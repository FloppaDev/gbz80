
/// Read instructions from source.
#[cfg(not(test))]
mod instructions;

/// Read instructions from source.
#[cfg(test)]
pub mod instructions;

/// Writes values from defines and markers.
pub mod constants;

/// Provides informations for each instruction in source.
pub mod ops;

/// Write the output of the assembler.
pub mod encode;

