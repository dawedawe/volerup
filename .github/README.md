***Please note: This is only a mirror repository. Development takes place on [codeberg](https://codeberg.org/dawe/volerup).***

---

# volerup

TUI application and library implementing the Vole machine language from the book *Computer Science: An Overview* by Glenn Brookshear and Dennis Brylow.

## CLI Usage

```shell
volerup [path_to_file]
```

## TUI Usage

| Key     | Action                                                      |
|---------|-------------------------------------------------------------|
| r       | Load the program into memory and reset the state of the CPU |
| p       | Do a fetch, decode, execute cycle                           |
| P       | Run loaded program to completion                            |
| Tab     | Switch focus to the next control                            |
| ↑ / ↓   | Scroll up/down                                              |
| ?       | Toggle the help screen with the list of CPU instructions    |
| Esc / q | Quit the program                                            |

## Example

An example program that stores `0x34` into memory cell 23.

```
0x14
0x02
0x34
0x17
0xC0
0x00
```

## Instructions

The simulated CPU has 16 general-purpose registers, each can hold 1 byte.  
The main memory consists 256 cells, each can hold 1 byte.

A Vole instruction is 2 two bytes long.
The first 4 bits identify the opcode for the instruction.  
The last 12 bits define the operands.

`0x1RXY` - `LOAD` memory cell `XY` into register `R`  
`0x2RXY` - `LOAD` value `XY` into register `R`  
`0x3RXY` - `STORE` value in register `R` in memory cell `XY`  
`0x40RS` - `MOVE` register `R` to register `S`  
`0x5RST` - `ADD` registers `R` and `S` as integers, store the result in register `T`  
`0x6RST` - `ADD` registers `R` and `S` as floats, store the result in register `T`  
`0x7RST` - `OR` registers `R` and `S`, store the result in register `T`  
`0x8RST` - `AND` registers `R` and `S`, store the result in register `T`  
`0x9RST` - `XOR` registers `R` and `S`, store the result in register `T`  
`0xAR0X` - `ROTATE` register `R` `X` times to the right  
`0xBRXY` - `JUMP` to instruction at memory cell `XY` if register `R` equals register `0`  
`0xC000` - `HALT` the execution  

If you focus on the `Program` listing, you can edit the instructions.  
You can add comments to your code with `//`.  
Load the program into memory to run it.

![show.gif](../volerup/vhs/show.gif)
