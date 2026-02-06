## [0.1.1] - 2026-02-06

### 🚀 Features

- Support parsing instructions in 2 byte form like 0xab12

### 📚 Documentation

- Use table to describe TUI usage
- Update docs for 2 byte form
- Add git-cliff cliff.toml

### ⚙️ Miscellaneous Tasks

- Bump volerup to version 0.1.1
## [0.1.0] - 2025-12-14

### 🚀 Features

- Initial import
- Add Cpu::init() function
- Implement floating
- Add instruction register and refactor
- Implement run
- Add ratatui frontend
- Add registers, memory and program
- Enumerate lists
- Impl Display for Opcodes and show them
- Add scrolling
- Add and improve README files
- Add help footer, go green
- Add editor for program
- Add editor for program
- Improve program parsing to allow empty lines
- Support multiple values per line
- Support reading a program from file via args
- Add help screen
- When showing the help screen, don't quit on q/Esc but toggle the screen
- Handle illegal instruction more gracefully
- Show list indexes as hex, improve layout
- Support running program to completion
- Support comments in programs
- Highlight the modified register or memory cell
- Show cycle counter
- Show register and memory values in base 10
- Add vhs

### 🐛 Bug Fixes

- Restrain visibility to crate
- Fix block title of program
- Align left controls vertically with list controls
- Fix Floating::encode() for 0.0

### 💼 Other

- Add github CI

### 🚜 Refactor

- Move lib code to vole.rs
- Rename struct fields
- Rename voleru to volerup
- Provide default style via function

### 📚 Documentation

- Document TUI usage, Vole instructions and the simulated hardware
- Document the vole-rs API
- Fix some instruction explainations
- Improve README files

### ⚙️ Miscellaneous Tasks

- Add woodpecker ci
