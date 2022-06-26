# gbz80

This is my attempt at making an assembler for the Gameboy, in Rust.
It takes an assembly file as input and outputs a binary that an emulator can run.

## Build

```
cargo build --release
```

## Usage

```
gbz80 ./asm/hello/hello.gb.asm -o ./hello.gb
```

## Features

- A stripped down version of the syntax from the z80
- Constants
- Build-time expressions
- Macros
- Error reporting

An hello world example is available in './asm/hello'

## Reference

The instruction set is available [here](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html) or in './gen/instructions/opcode.html'.  
Some alternative mnemonics from the bottom of the page are used and replace all the other options, such that a plus or minus sign is never used within an instruction.

## Structure of the project

- ./.vim  Syntax highlighting for '*.gb.asm' files in vim.
- ./asm   Examples and tests in assembly.
- ./gen   Code generation for the lexer.
- ./sh    Shell scripts
- ./src   Source code for the assembler.

Build the docs with

```
cargo doc --open
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


