---
aliases:
tags:
  - compilers
bad_links:
---
# Compiler vs. Linker

The compiler’s job is to take the source code and translate it into object code (machine code). It does this for each file separately and doesn’t have knowledge about the entire program or other source files. Therefore, it doesn’t know whether a `main` function exists in another file. Its main concern is that the syntax is correct and that all references within a single file can be resolved.

The linker takes over after the compiler has done its job. It links all the object files together to create the final executable. It’s during this stage that the linker resolves all the external references made in the object files.

**Note:**
The `main` function is usually the entry point of the program, if it’s missing, the linker won’t be able to find the starting point of the program, and hence, it throws an error.

## Differences in Functionality

### Compilation Process
The compilation phase, managed by the compiler, involves several steps:
1. **Lexical Analysis**: The source code is broken into tokens.
2. **Syntax Analysis**: The tokens are parsed into a parse tree to verify that they follow the appropriate grammatical rules.
3. **Semantic Analysis**: This step checks for semantic consistency like type checking and variable declarations.
4. **Code Optimization**: Optimizes the intermediate code for better performance.
5. **Code Generation**: Generates intermediate or machine code specific to a target architecture, resulting in object files.

### Linking Process
The linker's process involves:
1. **Symbol Resolution**: The linker identifies external symbols in the object files and matches with their definitions.
2. **Relocation**: Adjusts code and data in object files to their assigned memory locations.
3. **Library Linking**: Includes linking static libraries and, sometimes, dynamic libraries as per the references made.

## Key Differences
- **Scope of operation**: The compiler operates on a file-by-file basis, whereas the linker operates on the entire program.
- **Output**: The compiler outputs object files while the linker outputs an executable file by combining all object files.
- **Error Handling**: Compilation errors are usually syntax or semantic errors in the code. Linker errors are mainly due to missing symbols or functions, which are not found during the linking process.

## Conclusion
Understanding the roles and differences between a compiler and a linker is crucial for developers, especially in complex projects involving multiple source files and libraries. The compiler ensures the code meets the programming language specifications, while the linker ensures that separately compiled pieces can work together as a single cohesive program. Together, these tools streamline code development, error detection, and execution in software development.