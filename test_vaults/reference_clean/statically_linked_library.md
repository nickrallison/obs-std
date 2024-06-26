---
bad_links:
aliases: [static link, statically link, statically linked]
tags: [operatingsystems]
---

# Statically Linked Library

A statically linked library is a collection of object files that can be linked into an executable at compile time. This is in contrast to dynamically linked libraries, which are linked at runtime.

When a program is linked statically, all the code that the program needs to execute is included in the binary at compile time. This includes the code from the statically linked libraries that the program uses. The advantage of this is that the program is self-contained and can be run on any system without needing to install additional libraries. However, the downside is that the executable file size can be quite large, as it includes all the necessary code.

The process of static linking involves the linker (a part of the compiler toolchain) taking the object files generated by the compiler and combining them into a single executable file. The linker resolves all symbols (function and variable names) and replaces them with their actual memory addresses.

In the context of C programming, for example, if you have a program that uses functions from the math library (like `sqrt` or `sin`), you would include the math library in your compile command to link it statically:

```bash
gcc -static myprogram.c -lm -o myprogram
```

Here, `-static` tells the compiler to link statically, `-lm` includes the math library, and `-o myprogram` specifies the output file name.

Tangentially related items include:

- **[[Dynamically Linked Library.md|Dynamic Linking]]**: This is the counterpart to static linking. In dynamic linking, libraries are not included in the executable at compile time. Instead, they are linked at runtime. This reduces the size of the executable and allows multiple programs to share the same library code in memory, but it requires that the libraries be present on the system where the program is run.

- **[[Linker.md|Linker]]**: This is the tool that performs the linking process. It takes object files and libraries as input and produces an executable as output. The linker resolves symbols and fixes up relocations, among other things.

- **[[Object Files.md|Object Files]]**: These are the output of the compile step. They contain code in a binary format that the linker can understand. Object files are typically in a format like ELF (on Linux) or COFF (on Windows).

- **Symbols**: These are names of functions or variables in your code. The linker replaces symbols with their actual memory addresses during the linking process.

- **Relocations**: These are places in the code that need to be filled in with actual memory addresses by the linker. For example, a call to a function in a different object file is a relocation.

> For more information, you can refer to the following resources:
> - ["Static Libraries vs Dynamic Libraries"](https://www.google.com/search?q=Static+Libraries+vs+Dynamic+Libraries)
> - ["Linkers and Loaders"](https://www.google.com/search?q=Linkers+and+Loaders)
> - ["Understanding ELF using readelf and objdump"](https://www.google.com/search?q=Understanding+ELF+using+readelf+and+objdump)