# Avalanche
Stack-based (no registers) VM. Written in Rust "just for fun"
## Introduction
VM has "infinity" 32-bit stack, but **has no registers**. All instructions (excluding *test*) focused on working with stack
### Instructions
*Spaces and line breaks are ignored*
push **num** - push 32-bit number on stack  

pop - remove 32-bit number from stack  

test - print "Hello, World!" message  

add - add two numbers on stack. Result will be at the top of the stack (*Removes input number*)  

printn - display the number at the top of the stack (*Removes input number*)  

**Author does not guarantee that this list is valid. Always check source code**
## Building
#### Any operating system
1. Clone the repo:
`git clone https://github.com/avalanche-lang/avalanche.git`
2. Enter repo folder:
`cd avalanche`
3. Run following command:
`cargo build --release`
Building result can be found at target/release
