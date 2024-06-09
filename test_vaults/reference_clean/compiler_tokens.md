---
bad_links: 
aliases:
  - token
tags:
  - compilers
---
# Compiler Tokens

Compiler tokens, also known as lexical tokens, are the smallest units of a program that a compiler uses during the process of creating an abstract syntax tree (AST). The process of breaking down the source code into these tokens is called lexical analysis or scanning.

There are several types of tokens:

1. **Identifiers**: These are the names used for variables, functions, classes, etc. For example, in the line of code `int myVariable = 5;`, `myVariable` is an identifier.

2. **Keywords**: These are reserved words in a programming language that have special meaning. For example, `int`, `if`, `while`, etc.

3. **Operators**: These are symbols that perform operations on one or more operands. For example, `+, -, *, /, =`, etc.

4. **Literals**: These are constant values in the code. For example, `5` in the line of code `int myVariable = 5;` is a literal.

5. **Punctuators**: These are symbols that are used to organize the code but do not perform operations. For example, `{`, `}`, `(`, `)`, `;`, etc.

The process of lexical analysis can be represented by the following regular expression, where `D` represents a digit and `L` represents a letter:

```
identifier = letter (letter | digit)*
number = digit digit*
```

This means that an identifier is a letter followed by any number of letters or digits, and a number is one or more digits.

The output of the lexical analysis phase is a stream of tokens which is passed to the syntax analysis phase, where it is transformed into an AST. The AST is then used in the semantic analysis phase to check for semantic errors and generate intermediate code.

> For more information, you can refer to the following resources:
> - [Compiler Design - Lexical Analysis](https://www.tutorialspoint.com/compiler_design/compiler_design_lexical_analysis.htm)
> - [Lexical analysis](https://en.wikipedia.org/wiki/Lexical_analysis)
> - [Tokens in C](https://www.geeksforgeeks.org/tokens-in-c/)