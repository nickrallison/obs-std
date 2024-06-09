---
bad_links: 
aliases: []
tags: [linux]
---
# AWK

AWK is a powerful text-processing language named after its creators: Alfred Aho, Peter Weinberger, and Brian Kernighan. It's primarily used in Unix-like operating systems for data manipulation and report generation. AWK treats each line as a separate record and each word as a field, making it particularly useful for structured data.

The basic syntax of an AWK command is:

```bash
awk 'pattern {action}' file
```

Here, 'pattern' is a condition that you specify. If a line in the 'file' matches this pattern, AWK performs the 'action' on that line. If no pattern is specified, the action is performed on every line.

A simple example of an AWK command is:

```bash
awk '{print $1}' file
```

This command prints the first field (or word) of every line in 'file'. The '$1' is a variable that represents the first field. Similarly, '$2' would represent the second field, and so on.

AWK is a Turing-complete language, meaning it can simulate a universal Turing machine. This is a theoretical device that manipulates symbols on a strip of tape according to a table of rules. Despite its simplicity, a Turing machine can be adapted to simulate the logic of any computer algorithm, and is used in proofs of computability.

> For more information, you can refer to the [AWK manual](https://www.gnu.org/software/gawk/manual/gawk.html) or the book ["The AWK Programming Language"](https://www.amazon.com/AWK-Programming-Language-Alfred-Aho/dp/020107981X) by its creators. For a deeper understanding of Turing machines and computability, you can refer to the book ["Introduction to the Theory of Computation"](https://www.amazon.com/Introduction-Theory-Computation-Michael-Sipser/dp/113318779X) by Michael Sipser.