# pl-vm [![Build Status](https://travis-ci.com/p32blo/pl-vm.svg?token=ygk3UqtDM6juWT6yDp3H&branch=master)](https://travis-ci.com/p32blo/pl-vm)

An implementation of a stack-based Virtual Machine with debug support

# Documentation

The full architecture specification can be seen at the following links:

* [Portuguese](http://www.di.ubi.pt/~desousa/Compil/docpt.pdf)
* [French](http://www.di.ubi.pt/~desousa/Compil/doc.html)

# About

This is a Rust Language port of a VM created as a learning resources for Compiler Design class. In the assignment the goal is to use a compiler generator to translate a simple imperative language into instruction of this VM. This VM helps validating the translation by running the generated output.

# Usage

To just launch the VM and run the instruction in a file use:

	$ pl-vm  <file>

To initiate the VM in the integrated interactive debug use:

	$ pl.vm -d <file>

In order to see all available debug commands and their description you can use the `help` command:

	(debug) help
	COMMANDS:
		r, run              Continue the execution
		s, step [NUMBER]    Step by NUMBER instructions. NUMBER defaults to 1
		n, next [NUMBER]    Show NUMBER instructions. NUMBER defaults to 1
		reg, registers      Print the current value for the registers
		st, stack           Print the current state of the stack
		c, code             Print the code that is beeing run
		l, labels           Print all labels found in the code
		h, help             Print this message
		q, quit             Exit from the debugger

