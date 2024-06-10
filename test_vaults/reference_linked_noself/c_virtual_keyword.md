---
aliases:
  - virtual keyword
tags:
  - coding
  - cpp
bad_links:
---
# C++ Virtual Keyword

The virtual keyword in C++ is a powerful feature that allows for polymorphic behavior among classes. This keyword is primarily used in the context of inheritance, where a base class defines a method as "virtual", permitting derived classes to override it. Understanding the usage and implications of the virtual keyword is fundamental for applying object-oriented programming principles effectively in C++, especially in scenarios where dynamic binding is required.

## Basics of Virtual Functions

A virtual function is a member function in the base class that you expect to redefine in derived classes. When you use the `virtual` keyword in a base class, you are indicating to the C++ compiler that it should dynamically bind function calls involving that function. This dynamic binding allows a program to decide at runtime which function to call, based on the type of the object [[Pointer.md|pointer]] or reference, rather than the type of the [[Pointer.md|pointer]] or reference itself.

### Syntax

To declare a virtual function:

```cpp
class Base {
public:
    virtual void display() {
        std::cout << "Display of Base" << std::endl;
    }
};
```

### Overriding a Virtual Function

A derived class overrides a base class's virtual function by defining a function with the same signature.

```cpp
class Derived : public Base {
public:
    void display() override { // 'override' is optional but good practice
        std::cout << "Display of Derived" << std::endl;
    }
};
```

## Importance of Virtual Functions

### Polymorphism

The core purpose of virtual functions is to achieve polymorphism. They allow for the same function call to execute different code depending on the type of the object invoking the function. This behavior is crucial for designing flexible and scalable systems.

### Dynamic Binding

Without virtual functions, C++ uses static binding, which resolves the function call at compile time based on the type of the reference or [[Pointer.md|pointer]] through which the function is called. By declaring a function as virtual, you ensure that the call is resolved at runtime, enabling dynamic binding.

## Pure Virtual Functions and Abstract Classes

A pure virtual function is a virtual function that does not have an implementation in the base class and is declared by assigning 0 to it.

```cpp
class AbstractBase {
public:
    virtual void pureVirtualFunction() = 0;
};
```

This makes the class an abstract class, meaning it cannot be instantiated on its own. Abstract classes are used to define interfaces, where derived classes are expected to provide implementations for the pure virtual functions.

## Virtual Destructors

When a base class [[Pointer.md|pointer]] or reference is used to delete a derived class object, having a virtual destructor ensures that the derived class's destructor is called first, followed by the base class's destructor. This is crucial for proper resource management and avoiding memory leaks.

```cpp
class Base {
public:
    virtual ~Base() {
        // Cleanup code
    }
};
```

## Conclusion

The virtual keyword in C++ is fundamental for implementing polymorphism, one of the cornerstones of object-oriented programming. It allows classes to define interfaces and enable dynamic behavior at runtime, which is essential for writing flexible, maintainable, and scalable software. Understanding and correctly applying the principles of virtual functions, pure virtual functions, and virtual destructors is crucial for advanced C++ programming.