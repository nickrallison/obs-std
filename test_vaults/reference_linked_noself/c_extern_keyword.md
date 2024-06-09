---
aliases:
  - extern
tags:
  - coding
bad_links:
---
# C Extern Keyword

The `extern` keyword in C programming language is used to declare a global variable or function in another file. Essentially, it is used to give a reference of a global variable that is visible to ALL the programming files. When you use 'extern', the variable cannot be initialized however, it points the variable name at a storage location that has been previously defined.

When you have multiple files in a C project and you need to access a variable or function defined in one file from another file, `extern` is used to declare the variables or functions in the external file. This is especially useful in large projects structured around multiple files, allowing for better separation of concerns and cleaner code organization.

## Common Usage

### Declaring Global Variables

In projects where multiple files need to access the same variable, `extern` is used. For instance, you can define a global variable in one file and declare it in another file using `extern` to be able to use it.

File1.c:

```c
int counter = 0; // Definition
```

File2.c:

```c
extern int counter; // Declaration without initialization
```

### Function Declarations

Functions that are defined in one file can be declared in another file using `extern` keyword to allow them to be called from the file where they were declared.

File1.c:

```c
void printHello() {
    printf("Hello, World!\n");
}
```

File2.c:

```c
extern void printHello();
```

## Advantages of Using Extern

1. **Modularity:** Allows for better modularity and separation of concerns by letting you define variables and functions in separate files.
2. **Reusability:** Promotes code reusability by allowing multiple files to access common variables or functions.
3. **Organized Codebase:** Helps in keeping the codebase organized and manageable, especially in large projects.

## Key Points to Remember

- The `extern` keyword can only be used to declare a variable or function and not to define it. You cannot initialize a variable at the time of declaring it with `extern`.
- If an `extern` variable is not initialized elsewhere, linking will fail due to an unresolved external symbol.
- Using `extern` is a way to make the connection between the files when the project is being linked.
- It's a good practice to organize external variables and function declarations in header files, and include these headers in the .c files where they are used.

## Conclusion

The `extern` keyword plays a crucial role in managing the scope and visibility of variables and functions across different files in a C program. It facilitates clean and structured code organization, allowing for modular programming and easier maintenance of large codebases.