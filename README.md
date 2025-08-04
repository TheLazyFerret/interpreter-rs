
# interpreter-rs

Very simple interpreter made in rust. Inspired by assembly (specially mips32), though not being a simulator.

## Usage
The program will read a file and run the instructions imperatively. If during the execution of an instruction there is an error, the program will stop its execution.  

Does not support inline comments.  

I am not glad at all with the implementation, due that in order to find the labels I need to watch all the document, feels like cheating :\(

### Labels
Labels are set with ```@[A-Z]```. You can use any name that contains only caps letters.  

The interpreter will search for the label ```@MAIN```, and will start the execution there  

### Registers
The simulator stores 32 registers, which can be accessed with ```$[reg]```.  
- Register 0 maintains the value zero and cannot be changed.

### Instructions implemented
- ```LI $[reg] [Imm]``` -> $reg = Imm
- ```MOVE $[reg0] $[reg1]``` -> $reg0 = $reg1
- ```ADD $[reg0] $[reg1] $[reg2]``` -> $reg0 = $reg1 + $reg2
- ```SUB $[reg0] $[reg1] $[reg2]``` -> $reg0 = $reg1 - $reg2
- ```MUL $[reg0] $[reg1] $[reg2]``` -> $reg0 = $reg1 * $reg2
- ```DIV $[reg0] $[reg1] $[reg2]``` -> $reg0 = $reg1 / $reg2
- ```REM $[reg0] $[reg1] $[reg2]``` -> $reg0 = $reg1 % $reg2
- ```PRINT $[reg]``` -> print $reg
- ```JUMP @[label]``` -> set instruction counter to label's one.
- ```BEQ|BNE|BLT|BLE|BGT|BGE $[reg] $[reg] @[label]``` -> set instruction counter to label's one if condition is true.
- ```EXIT``` -> terminates the execution.
- ```SKIP``` -> skip the line. Equivalente to // or an empty line.
- ```PUSH $[reg]``` -> push in the stack the value in the $\[reg\].
- ```POP $[reg]``` -> put in the $\[reg\] the value in stack's pop. 

## ToDo
- [x]  Basic usage (basic arithmetic, basic data transfer).
- [x]  Basic system calls (print, exit)
- [x]  Unconditional jumps.
- [x]  Conditional jumps.
- [x]  Stack operations.

## Requirements
- [rustc](https://www.rust-lang.org/) >= 1.80.0
- [regex](https://crates.io/crates/regex) >= 1.11.1

## License
[MIT](LICENSE)
