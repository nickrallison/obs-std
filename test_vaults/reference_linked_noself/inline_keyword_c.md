---
aliases:
tags:
  - c
  - cpp
bad_links:
---
# Inline Keyword C++

optimizes the execution of programs by reducing function call overhead. The `inline` keyword allows the function to be expanded in line at the point of call rather than being called through the normal function call mechanism. This can potentially increase the performance of the program by avoiding function call overhead, but it also increases the size of the binary since the function might be copied multiple times.

When a function is declared with the `inline` keyword, the compiler is given a hint that substitution of the function body at the point of call is preferable over the standard function calling mechanism. However, it is important to note that `inline` is merely a suggestion to the compiler, not a command. The compiler may choose to ignore this suggestion for various reasons like the function being too complex or containing loops, dynamic allocation, recursion, etc.

## Syntax
To declare a function as `inline`, place the `inline` keyword before the function definition:

```cpp
inline int add(int a, int b) {
    return a + b;
}
```

## Usage Considerations

1. **Overhead Reduction**: Primarily used for small functions that are called frequently.

2. **Multiple Definitions**: Unlike normal functions, inlined functions can be defined multiple times without violating the One Definition Rule (ODR), as long as all definitions are identical. This is because each definition is substituted at the place where the call is made.

3. **Debugging and Error Handling**: Debugging can be more complex because an inlined function does not exist independently on the call stack.

4. **Recursive Functions**: It is generally not advisable to inline recursive functions because it can lead to an infinite amount of inline expansions.

## Best Practices

- **Small and Frequently Used Functions**: The best use case for `inline` functions is with small, frequently called functions like accessors or simple mathematical functions (e.g., `max()`, `min()`).

- **Avoid Inlining Large Functions**: While it can be tempting to inline larger functions for performance, this should generally be avoided unless profiling dictates it. Inlining large functions can lead to code bloat which may degrade performance due to [[caching.md|cache]] misses.

- **Header Files**: Inline functions should be defined in header files so that their definition is visible in every translation unit they are used. This requires careful management of header file dependencies to avoid making widespread recomplications for small changes.

## Impact on Performance

While the `inline` specification can help in reducing function call overhead, it may not always result in a faster executable. The increased size of the binary can affect instruction [[caching.md|cache]] utilization, potentially reducing overall performance. Therefore, it's generally recommended to rely on the compiler's optimization heuristics and only force inlining as a result of measured and significant performance improvements detected during profiling sessions.