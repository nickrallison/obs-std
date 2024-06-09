---
aliases: []
tags: [coding]
bad_links: [Binary Representation.md]
---
# C Libraries

## Sources
<https://www.youtube.com/watch?v=JbHmin2Wtmc>

## Summary

Creating Libraries in C for Linux: Exploring the Process and Benefits

C Libraries play a crucial role in programming by enabling code reuse and distribution of favorite functions or data structures. While the C standard library, LibC, is commonly used, there are advantages to creating our own libraries. This document explores the process of creating a library in C for Linux.

The first step is to create a header file that includes the necessary function declarations. This example focuses on the "reverse" function, which reverses a string in place and returns a pointer to the reversed string. The function directly modifies the input string without copying it.

To test the library, a separate program is created, which calls the reverse function and prints both the original and reversed strings. A Makefile is included for easy code compilation, and the resulting object files or libraries can be used in other projects or shared with colleagues.

In addition to using .o files for linking, the document discusses creating shared object files (.so) or dynamically linked libraries. A shared object file is loaded at runtime, separate from the final compiled binary. The necessary options and compilation steps to create a shared library are explained.

When using a shared library, it may be necessary to specify its location to the program loader. This can be achieved by adding the library directory to the LD_LIBRARY_PATH environment variable or installing the library in one of the default search directories like /usr/lib. Using a shared library helps reduce overall code size, as demonstrated by comparing object dumps of programs with and without the shared library.

The benefits of using shared libraries include easy bug patching by installing new versions of the library without modifying every program using it, and space-saving as multiple programs can share the same code instead of including individual copies.

In conclusion, creating and using libraries in C enhances programming efficiency and maintainability. Whether utilizing existing libraries like LibC or creating custom ones, understanding the process and benefits of library usage is crucial for programmers.

The document also covers the process of creating a static library in C for Linux. A static library, created using a .a file, packages static code for static linking. The "ar" command is used to create the .a file, replacing any existing files with the same name and generating an index for the compiler. Multiple .o files can be bundled in the static library. A Makefile rule can be added to compile a program with the static library. The static version is larger in size compared to the dynamic version but does not require the library to be present.

Although the process may differ in Mac OS and Windows, the principles remain the same, facilitating code packaging and sharing across platforms.