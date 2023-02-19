# gbz80

This is my attempt at making an assembler for the Gameboy, in Rust.
It takes an assembly file as input and outputs a binary that an emulator can run.  
  
I made this mostly to learn and experiment with various things :)

## Build

```
cargo build --release
```

## Usage

```
gbz80 ./asm/hello/hello.gb.asm -o ./hello.gb
```
`-o [FILE]` Set the output destination (required).  
`-D [SYMBOLS]` Define symbols for conditional compilation.

## Features

- A stripped down version of the syntax from the z80
- Constants
- Build-time expressions
- Macros
- Error reporting

An hello world example is available in `asm/hello`.

## Reference

The instruction set is available [here](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html) or in `gen/instructions/opcode.html`.  
Some alternative mnemonics from the bottom of the page are used and replace all the other options, such that a plus or minus sign is never used within an instruction.

## Structure of the project

- `.vim`: Syntax highlighting for '*.gb.asm' files in vim.
- `asm`: Examples and tests in assembly.
- `gen`: Code generation for the lexer.
  - `gen/instructions`: Generates the instructions list.
  - `gen/lex`: Generates syntax rules.
  - `gen/image`: Image conversion tool.
- `sh`    Scripts and tools
- `src`   Source code and tests for the assembler.

Build the docs with:
```
cargo doc --open
```
Or run in debug mode, it will show the result of every step in the process:
```
cargo r ./asm/hello/hello.gb.asm -o ./hello.gb
```

## Unimplemented

- Memory banks
- Multiple source files

## Sources

- Assembly tutorials:   
    - https://www.chibiakumas.com   
- Instruction set:  
    - https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html  
- Cpu manual:  
    - http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf  
- Memory banks in roms:  
    - https://www.reddit.com/r/EmuDev/comments/dyqz7f/gb_file_mbc_formatting_for_game_boy_emulation/
- ROM analyzer:
    - https://www.romhacking.net/utilities/1343/
- Xxd (hex viewer):
    - https://linux.die.net/man/1/xxd
- Hello World:
    - https://www.chibiakumas.com/z80/helloworld.php
- Snake:
    - https://github.com/theisolinearchip/gb_raw_snake
