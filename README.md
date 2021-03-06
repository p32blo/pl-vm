# pl-vm [![Build Status](https://travis-ci.com/p32blo/pl-vm.svg?token=ygk3UqtDM6juWT6yDp3H&branch=master)](https://travis-ci.com/p32blo/pl-vm)

An implementation of a stack-based Virtual Machine architecture with debug support


# About

This is an implementation of a VM with the Rust Language created as a learning resources for Compiler Design class. In the assignment the goal is to use a compiler generator to translate a simple imperative language into instruction of this VM. This VM helps validating the translation by running the generated output.

This implementation started as an exercise on the Rust Language but it can also serve the same purposes of the existing implementations. Furthermore, features such as the integrated debugger will provide a better and faster experience debugging both the generated code and its output.


# Documentation

The full architecture specification can be seen at the following links:

* [Portuguese](http://www.di.ubi.pt/~desousa/Compil/docpt.pdf)
* [French](http://www.di.ubi.pt/~desousa/Compil/doc.html)
* [English](#progress)


# Usage

To just launch the VM and run the instruction in a file use:

	$ pl-vm  <file>

To initiate the VM in the integrated interactive debug use:

	$ pl-vm -d <file>

In order to see all available debug commands and their description you can use the `help` command:

	(debug) help
	COMMANDS:
		r, run              Continue the execution
		s, step [NUMBER]    Step by NUMBER instructions. NUMBER defaults to 1
		n, next [NUMBER]    Show NUMBER instructions. NUMBER defaults to 1
		reg, registers      Print the current value for the registers
		st, stack           Print the current state of the stack
		c, code             Print the code that is being run
		l, labels           Print all labels found in the code
		h, help             Print this message
		q, quit             Exit from the debugger


## Error Messages

The following Execution Errors are the possible failure states of this `vm`:

* `Illegal Operand` - Triggered when the value(s) on the stack are not of the expected type
* `Segmentation Fault` - Triggered for access to an illegal area of the code, stack, or one of two heaps
* `Stack Overflow` - Triggered for any attempt to add to the top of a full stack (execution stack or call stack)
* `Division By Zero` - Triggered in case of division (integer) by zero
* `Error "message"` - Triggered when the err statement is executed
* `Anomaly` - This error must never occur; If so, please report it to the teachers, attaching as much as possible the program that triggered it.


# Progress

*`32 of 75 instructions completed`*

This project is an work in progress and not all `vm` instructions are yet implemented.

The following sections documents all the instructions for `vm` while reporting if the implementation of each instruction is complete (✅) or still missing (❌).


## Integer Operations

Instructions | Status | Description
-------------|--------|------------
ADD          | ✅      | Pop `n` then `m` which must be an integer and stack the result of `m + n`
SUB          | ✅      | Pop `n` then `m` which must be an integer and stack the result of `m - n`
MUL          | ✅      | Pop `n` then `m` which must be an integer and stack the result of `m x n`
DIV          | ✅      | Pop `n` then `m` which must be an integer and stack the result of `m / n`
MOD          | ✅      | Pop `n` then `m` which must be an integer and stack the result of `m mod n`

Instructions | Status | Description
-------------|--------|------------
NOT          | ❌      | Pop `n` which must be an integer and stack the result of `n = 0`
INF          | ✅      | Pop `n` then `m` which must be an integer and stack the result of `m < n`
INFEQ        | ✅      | Pop `n` then `m` which must be an integer and stack the result of `m ≤ n`
SUP          | ✅      | Pop `n` then `m` which must be an integer and stack the result of `m > n`
SUPEQ        | ✅      | Pop `n` then `m` which must be an integer and stack the result of `m ≥ n`


## Floating Point Operations

Instructions | Status | Description
-------------|--------|------------
FADD         | ❌      | Pop `n` then `m` which must be a real number and stack the result of `m + n`
FSUB         | ❌      | Pop `n` then `m` which must be a real number and stack the result of `m - n`
FMUL         | ❌      | Pop `n` then `m` which must be a real number and stack the result of `m x n`
FDIV         | ❌      | Pop `n` then `m` which must be a real number and stack the result of `m / n`

Instructions | Status | Description
-------------|--------|------------
FCOS         | ❌      | Pop `n` which must be a real number and stack the result of `cos(n)`
FSIN         | ❌      | Pop `n` which must be a real number and stack the result of `sin(n)`

Instructions | Status | Description
-------------|--------|------------
FINF         | ❌      | Pop `n` then `m` which must be a real number and stack the result of `m < n`
FINFEQ       | ❌      | Pop `n` then `m` which must be a real number and stack the result of `m ≤ n`
FSUP         | ❌      | Pop `n` then `m` which must be a real number and stack the result of `m > n`
FSUPEQ       | ❌      | Pop `n` then `m` which must be a real number and stack the result of `m ≥ n`


## Address Operations

Instructions | Status | Description
-------------|--------|------------
PADD         | ✅      | Pop `n` which must be an integer then `a` which must be an address from the stack and stack the address `a + n`


## String Operations

Instructions | Status | Description
-------------|--------|------------
CONCAT       | ❌      | Pop `n` then `m` which must be string addresses, stack the address of a string equal to the concatenation of the string address  `n` and string address `m`


## Heap Operations

Instructions | Status | Description
-------------|--------|------------
ALLOC n      | ❌      | Allocate a structured block of size `n`(integer) on the heap and stack the corresponding address
ALLOCN       | ❌      | Pop an integer `n` and allocate a structured block of size `n` on the heap and stack the corresponding address
FREE         | ❌      | Pop an address `a` and release the structured block allocated at address `a`


## Equality

The equality test tests whether two objects on the stack (integers, real or addresses) are equal. An execution error occurs if the two objects are not of the same type. Two strings stored at the same address are equal, so this instruction can be used to test the equality of two strings.

Instructions | Status | Description
-------------|--------|------------
EQUAL        | ✅      | Pop `n` then `m` which must be of the same type and stack the result of `n = m`


## Conversions

Various instructions can be used to convert a string to integer or real and vice versa.

Instructions | Status | Description
-------------|--------|------------
ATOI         | ✅      | Pop the address of a string, and stack its conversion to an integer, fail if the string does not represent an integer.
ATOF         | ❌      | Pop the address of a string and stack its conversion to real number, fail if the string does not represent a real number.
ITOF         | ❌      | Pop an integer and stack its conversion into a real number.
FTOI         | ❌      | Pop a real number and stack the integer representing its integer part (obtained by removing the decimals).
STRI         | ❌      | Pop an integer and stack the address of a string representing that integer
STRF         | ❌      | Pop a real number and stack the address of a string representing this real number


## Stack Operations 

If `x` denotes an address in the stack, then `x[n]` denotes an address with an offset of `n` above `x`.


### Push
Instructions | Status | Description
-------------|--------|------------
PUSHI n      | ✅      | Stack `n`(integer)
PUSHN n      | ✅      | Stack `n`(integer) times the integer value `0`
PUSHF n      | ❌      | Stack `n`(real number)
PUSHS n      | ✅      | Store `n`(string) in the string area and stack the address
PUSHG n      | ✅      | Stack the value in `gp[n]`, where `n` must be an integer
PUSHL n      | ❌      | Stack the value in `fp[n]`, where `n` must be an integer
PUSHSP       | ❌      | Stack the value of the `sp`
PUSHFP       | ❌      | Stack the value of the `fp` register
PUSHGP       | ✅      | Stack the value of the `gp` register
LOAD n       | ❌      | Pop an address `a` and stack the value in the stack or heap in `a[n]`, where `n` must be an integer
LOADN        | ✅      | Pop an integer `n`, an address `a` and stack the value in the stack or the heap in `a[n]`
DUP n        | ❌      | Duplicate and stack the `n`(integer) values at the top of the stack
DUPN         | ❌      | Pop an integer `n`, then duplicate and stack the `n` values at the top of the stack


### Pop

Instructions | Status | Description
-------------|--------|------------
POP n        | ❌      | Pop `n`(integer) values in the stack
POPN         | ❌      | Pop an integer `n` then pop `n` values in the stack 


### Store

Instructions | Status | Description
-------------|--------|------------
Storel n     | ❌      | Take a value `n`(integer) and store it in the stack at `fp[n]`
STOREG n     | ✅      | Take a value `n`(integer) and store it in the stack in `gp[n]`
STORE n      | ❌      | Pop a value `v` and an address `a` , store `v` at the address `a[n]` in the stack or heap, where `n` must be an integer
STOREN       | ✅      | Pop a value `v`, an integer `n` and an address `a` , store `v` at the address `a[n]` in the stack or heap, where `n` must be an integer


### Miscellaneous

Instructions  | Status | Description
--------------|--------|------------
CHECK n p     | ❌      | Verify that the vertex of the stack is an integer `i` such that `n` ≤ `i` ≤ `p` , else fails on an error
SWAP          | ❌      | Pop `n` then `m` and stack `n` then `m`


## Input/Output

Instructions  | Status | Description
--------------|--------|------------
WRITEI        | ✅      | Pop an integer and print it to the standard output
WRITEF        | ❌      | Pop a real number and print it to the standard output
WRITES        | ✅      | Pop the address of a string and print the corresponding string to the standard output
READ          | ✅      | Read a string on the keyboard, terminated by a carriage return, store the string (without the carriage return) and stack the address.


## Graphical Primitives

Instructions     | Status | Description
-----------------|--------|------------
DRAWPOINT        | ❌      | Pop `m` then `n` which must be an integers and draw a coordinate point (`n`,`m`)
DRAWLINE         | ❌      | Pop `q`, `p`, `m` and `n` which must be an integers and draw a segment between (`n`,`m`) and (`p`,`q`)
DRAWCIRCLE       | ❌      | Pop `p`, `m` and `n` which must be integers and draw a circle with center (`n`,`m`) and radius `p`
OPENDRAWINGAREA  | ❌      | Pop `h` then `w` which must be integer and open a new graph window with width `w` and height `h`
CLEARDRAWINGAREA | ❌      | Clear the graphic output and reset the current color to `black`
SETCOLOR         | ❌      | Pop `b`, `g` and `r` which must be integer and change the current color according to the RGB value defined by the three integers between `0` and `65535`
REFRESH          | ❌      | Refreshe the graphics window, i.e. make visible the graphical operations performed since the last refresh


## Control Operations

Instructions  | Status | Description
--------------|--------|------------
JUMP label    | ✅      | Assign the address in the program corresponding to label to the register `pc` which can be an integer or a symbolic value
JZ label      | ✅      | Pop a value, if it is `zero` assign the program address corresponding to the label, if not increment `pc` by 1
PUSHA lable   | ✅      | Stack the program address corresponding to the label


## Procedure

When calling a procedure it is necessary to save the instruction register and local variables that will be restored upon returning.

Instructions  | Status | Description
--------------|--------|------------
CALL          | ✅      | Pop a code address `a`, save `pc` and `fp` in the call stack, set `fp` to the current value of `sp` and set `pc` to `a`
RETURN        | ✅      | Assign to `sp` the current value of `fp`, pop the `fp` and `pc` from the call stack and increment `pc` by 1 to return to the instruction following the procedure call


## Initialization and Termination

In the initial state, the `pc` register points to the first instruction in the program. The call stack and run stack are empty. The registers `gp` and `sp` point to the execution stack while `fp` is not defined. The `fp` register must be initialized by the START statement, which can only be used once. The following instructions are used to stop the machine at the end of the program or in the event of an error.

Instructions  | Status | Description
--------------|--------|------------
START         | ✅      | Assign the value of `sp` to `fp`
NOP           | ✅      | Do nothing
ERR x         | ✅      | Trigger an instruction error with message `x`(string)
STOP          | ✅      | Stop program execution


# Syntax

## Lexical Conventions

`spaces`, `tabs` and `carriage returns` are considered whitespace. Comments begin with `//` and continue until the end of the line. The identifiers for `<ident>` obey the following regular expression: 

	<digit> ::= 0-9
	<alpha> ::= a-z | A-Z
	<ident> ::= (<alpha>|_)(<alpha>|<digit>|_|')*
	
The integer and real number constants are defined by the following regular expressions:
	
	<integer> ::= -?<digit>+
	<float> ::= -?<digit>+(.<digit>*)?((e|E)(+|-)?<digit>+)?
 
With the convention that a constant is a real number only if it is not also an integer constant (in other words, a real constant contains at least a decimal point or an exponent).

Strings are delimited by the character `"` , and can contain the same character only if preceded by the character `\`. In other words, strings obey the following regular expression:
	
	<string> ::= "([^"]|\")*"
	
All identifiers that are instructions (see syntax below) are reserved and are case-insensitive.


## Syntax

Every program follows the following syntax:

    <code> ::= <instr>*
    
    <instr> ::= <ident> :
              | <instr_atom>
              | <instr_int> <integer>
              | pushf <float>
              | (pushs | err) <string>
              | check <integer> , <integer>
              | (jump | jz | pusha) <ident>

    <instr_atom> ::= padd | add | sub | mul | div | mod | not | infeq | inf | supeq
                   | sup | fadd | fsub | fmul | fdiv | fcos | fsin
                   | finfeq | finf | fsupeq | fsup | concat | equal | atoi | atof
                   | itof | ftoi | stri | strf
                   | pushsp | pushfp | pushgp | loadn | storen | swap
                   | writei | writef | writes | read | call | return
                   | drawpoint | drawline | drawcircle
                   | cleardrawingarea | opendrawingarea | setcolor | refresh
                   | start | nop | stop | allocn | free | dupn | popn

    <instr_int> ::= pushi | pushn | pushg | pushl | load
                  | dup | pop | storel | storeg | alloc

