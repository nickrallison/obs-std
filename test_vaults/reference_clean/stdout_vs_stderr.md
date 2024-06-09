---
bad_links: 
aliases: []
tags: [linux, operatingsystems]
---
# STDOUT Vs STDERR

STDOUT (Standard Output) and STDERR (Standard Error) are two of the three standard streams (the other being STDIN, Standard Input) used in computer programming and operating systems. These streams are preconnected input and output communication channels between a program and its environment.

STDOUT is the default file descriptor where a process can write output. In most systems, STDOUT is displayed on the computer screen. However, it can be redirected to a file, another program, or even a printer. For example, in a Unix-like operating system, you can redirect the STDOUT of a command to a file using the '>' operator:

```bash
command > file.txt
```

STDERR is another output stream typically used by programs to output error messages or diagnostics. It is distinct from STDOUT and is not affected by STDOUT redirections. This separation allows for better handling and debugging of error messages. Like STDOUT, STDERR can also be redirected. In Unix-like systems, the '2>' operator is used to redirect STDERR:

```bash
command 2> error.txt
```

The numbers '1' and '2' are file descriptors (FDs). They are integer values that uniquely identify an opened file. By convention, FD 0 is STDIN, FD 1 is STDOUT, and FD 2 is STDERR.

The concept of these streams comes from the Unix philosophy of "everything is a file", which abstracts all input and output operations into reads and writes from or to file descriptors. This abstraction simplifies many operations and allows for powerful combinations of commands.

> For more in-depth reading, you might find the following resources helpful:
> - [File Descriptors](https://www.google.com/search?q=File+Descriptors)
> - [Redirection (computing)](https://www.google.com/search?q=Redirection+(computing))
> - [Unix Philosophy](https://www.google.com/search?q=Unix+Philosophy)