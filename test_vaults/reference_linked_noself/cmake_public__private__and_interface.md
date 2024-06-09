---
aliases: 
tags:
  - coding
  - cmake
bad_links:
---
# CMake Public, Private, and Interface

CMake is a cross-platform, open-source build system that manages the build process in an operating system and compiler-independent manner. When CMake processes a project, it generates native makefiles and workspaces that can be used in the compiler environment of your choice. A crucial part of modern CMake practices involves the use of target properties to manage and propagate usage requirements, especially include directories, preprocessor definitions, and compilation options. These properties can be categorized into three different scopes: `PUBLIC`, `PRIVATE`, and `INTERFACE`. Understanding these scopes is key to writing robust and maintainable CMake scripts, especially in projects that involve multiple targets or are intended to be used as dependencies in other projects.

## `PUBLIC`

The `PUBLIC` keyword is used when specifying requirements that are both needed by the target itself and will also be used by anyone linking to the target. Put simply, using `PUBLIC` means that the requirement is added to both the target and anyone linking against the target.

When you specify something as `PUBLIC`, both the target and the targets linking to it inherit the properties or requirements. This is commonly used for headers that are part of the public interface of a library, as well as any compile definitions or options that are necessary both for compiling the library itself and for compiling code that uses the library.

Example:

```cmake
add_library(my_library STATIC src.cpp)
target_include_directories(my_library PUBLIC include/)
```

In this example, `include/` directory is added as a public include directory for `my_library`. This means that not only will `my_library` be able to include headers from `include/`, but also any target that links against `my_library` will automatically get `include/` added to its include directories.

## `PRIVATE`

The `PRIVATE` keyword is used for requirements that are only applicable to the target, and not to anyone linking to it. `PRIVATE` is used for details that are internal to the target and should not be propagated to other targets that link against it.

For instance, if a target source file requires a specific include directory, that is not needed by anyone linking against the target (perhaps because it's an internal implementation detail), then this directory should be added as a `PRIVATE` requirement.

Example:

```cmake
target_compile_definitions(my_library PRIVATE -DMY_LIBRARY_INTERNAL)
```

Here, the preprocessor definition `MY_LIBRARY_INTERNAL` is used only while compiling `my_library`. It is not visible or required by other targets that link against `my_library`.

## `INTERFACE`

The `INTERFACE` keyword is used for requirements that are not used by the target itself, but are required by targets that link against it. Essentially, `INTERFACE` is for properties that are meant purely for consumers of the target.

An `INTERFACE` requirement is useful for header-only libraries or when a target needs to expose compile options, compile definitions, or include directories to targets that depend on it without using those properties itself.

Example:

```cmake
add_library(interface_lib INTERFACE)
target_compile_definitions(interface_lib INTERFACE -DINTERFACE_LIB_USE)
```

In this case, `interface_lib` does not compile any sources itself (it's an interface library). However, any target linking to `interface_lib` will inherit the `INTERFACE_LIB_USE` definition.

In summary, `PUBLIC`, `PRIVATE`, and `INTERFACE` keywords in CMake allow fine-grained control over how usage requirements are propagated to different targets. This facilitates better encapsulation and cleaner, more maintainable builds especially in projects that involve multiple targets or those that act as dependencies for other projects.