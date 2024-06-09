---
bad_links:
aliases: []
tags: [linux]
---
# SED

SED, or Stream Editor, is a powerful text manipulation utility in Unix and Linux systems. It is used to perform basic text transformations on an input stream (a file or input from a pipeline). SED works by making only one pass over the input(s), and is consequently more efficient than text editors which must first read the entire input into memory, then perform a series of global search-and-replace operations on the in-memory copy of the file.

The basic syntax of a SED command is as follows:

```
sed 'options' [file-to-edit]
```

The 'options' part of the command can contain one or more 'functions', which are the actual instructions that SED will execute on the input text. These functions can be anything from simple text replacement to more complex operations like line deletion or insertion.

Here are a few examples of common SED functions:

- `s/regexp/replacement/flags`: This is the substitute command. The 'regexp' is a [[Regular Expression.md|regular expression]] that matches the text to be replaced. The 'replacement' is the text that will replace the matched text. The 'flags' are optional parameters that modify the behavior of the substitution.

- `d`: This is the delete command. It deletes the current pattern space, reads in the next line, puts the new line into the pattern space, and starts the script again from the first command.

- `p`: This is the print command. It prints the current pattern space.

SED commands can be combined into a script, and the script can be executed with the `-f` option. For example, the command `sed -f scriptfile inputfile` would execute the SED commands in 'scriptfile' on the text in 'inputfile'.

SED operates in a cycle. It reads an input line into a buffer, called the pattern space, executes all the commands on the pattern space, and then outputs the pattern space to the screen. Then it reads the next line and repeats the process. This is why SED is called a stream editor.

> For more in-depth information, you can refer to the [GNU SED Manual](https://www.gnu.org/software/sed/manual/sed.html).

As for relevant formulas, derivations, or proofs, these are not typically associated with SED, as it is a practical tool for text manipulation rather than a theoretical concept. However, the efficiency of SED can be understood in terms of Big O notation, which is used in computer science to describe the performance or complexity of an algorithm. SED's one-pass approach gives it a linear [[Big-O Notation|time complexity]], or O(n), where n is the size of the input. This means that the time it takes to process the input grows linearly with the size of the input.

> For more information on Big O notation and [[Big-O Notation|time complexity]], you can refer to this [Google search](https://www.google.com/search?q=Big+O+notation+and+time+complexity).