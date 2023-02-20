
/// Runs tests for various stages of the assembler.
mod stages;

use std::fs::File;
use std::io::Read;

/// Random number between 0 and max.
pub fn urand(max: usize) -> usize {
    let mut buffer = [0u8; 4];
    let mut f = File::open("/dev/urandom").unwrap();
    f.read_exact(&mut buffer).unwrap();

    let mut rand = buffer[0] as usize;
    rand += (buffer[1] as usize) << 8;
    rand += (buffer[2] as usize) << 16;
    rand += (buffer[3] as usize) << 24;

    rand % max
}

pub fn rand_file() -> String {
    rand(&mut [0u8; 10_000])
}

pub fn rand_word() -> String {
    rand(&mut [0u8; 10])
}

/// Create a string from random bytes.
pub fn rand(buffer: &mut [u8]) -> String {
    let mut f = File::open("/dev/urandom").unwrap();
    f.read_exact(buffer).unwrap();

    let mut input = String::with_capacity(buffer.len());

    for c in buffer {
        let mut c = *c as char;

        if c == '\0' {
            c = ' ';
        }

        input.push(c);
    }

    input
}
