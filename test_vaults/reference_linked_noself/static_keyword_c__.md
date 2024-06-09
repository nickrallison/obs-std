---
aliases:
tags:
  - cpp
bad_links:
---
# Static Keyword C++

In C++ when a function is declared as static, that is telling the [[Linker.md|linker]] that it will only ever be used in that translation unit. This restriction produces several effects and uses that can be beneficial for a C++ program:

### Local Function Visibility

Static functions are only visible within the file (translation unit) they are defined in. This is akin to declaring functions "private" to a file, preventing other files from calling them. This is useful for creating helper functions in a file that should not be accessible externally, thereby reducing potential for conflicts in function names across different files and ensuring better encapsulation.

### Maintaining State within a Function

If a variable within a function is declared as `static`, it maintains its state between function calls. This means that the variable is not reinitialized each time the function is called. Instead, it retains the value from the previous call. This can be useful for functions that need to keep track of state or information between invocations, such as generating unique identifiers or counting the number of times a function is called:

```cpp
void countCalls() {
    static int count = 0;  // Initialize only once
    count++;
    std::cout << "Function has been called " << count << " times" << std::endl;
}
```

### Class-Level Static

When applied to class members (variables and functions), the `static` keyword gives a different behavior:

#### Static Members
Static member variables are shared across all instances of a class, meaning they have the same value in every instance. This could be useful for counting the total number of class instances, or storing data that needs to be accessed by all instances globally, rather than on a per-object basis:

```cpp
class Example {
    public:
        static int count;
        Example() {
            count++;
        }
};

int Example::count = 0;  // Initialize static member
```

#### Static Methods
Static methods can be called on the class itself, rather than on instances of the class. They can only access static members of the class. Static methods are often used as utility functions that are related to the class but do not operate on specific instances:

```cpp
class MathUtils {
public:
    static int add(int a, int b) {
        return a + b;
    }
};
```

Use `MathUtils::add(5, 3)` to call this static method.

### Use in Anonymous Namespace

Alternatively, to limit the scope of functions or variables within a translation unit, C++ developers can use anonymous namespaces, which offer a modern alternative to `static` for hiding names:

```cpp
namespace {
    void helperFunction() {
        std::cout << "This function is local to the file." << std::endl;
    }
}
```

This function has internal linkage, similar to `static` functions, but uses a more explicit and scoped approach.

## Conclusion

The `static` keyword serves multiple purposes in C++, from controlling linkage and visibility of functions and variables, to managing state and shared data across instances of classes. It provides crucial tools for developers to manage scope, lifetime, and access, contributing to more robust and maintainable C++ codebases.