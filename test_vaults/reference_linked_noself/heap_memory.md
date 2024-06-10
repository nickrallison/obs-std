---
bad_links: 
aliases: []
tags: [coding, operatingsystems]
---
# [[Heap Data Structure|Heap]] Memory

[[Heap Data Structure|Heap]] memory, also known as dynamic memory, is a region of a computer's memory that is used for [[Dynamic Memory Allocation|dynamic memory allocation]]. It is called a "[[heap_data_structure.md|heap]]" because it is a pile of memory space available to programmers to allocated and de-allocate. If a program uses more memory space, it can request more from the [[Heap Data Structure|heap]]; if it uses less, it can return memory back to the [[Heap Data Structure|heap]].

[[Heap Data Structure|Heap]] memory is managed via two key operations: allocation and deallocation. Allocation involves reserving a block of memory on the [[Heap Data Structure|heap]] for use by the program, while deallocation involves returning a previously allocated block back to the [[Heap Data Structure|heap]]. These operations are typically performed using functions such as `malloc()`, `calloc()`, `realloc()`, and `free()` in C and C++.

[[Heap Data Structure|Heap]] memory is different from [[Stack Memory|stack memory]], which is used for [[Static Memory Allocation|Static Memory Allocation]]. While [[Stack Memory|stack memory]] is automatically managed by the computer, [[Heap Data Structure|heap]] memory is manually managed by the programmer. This gives more flexibility but also introduces the possibility of errors such as [[Memory Leak|memory leaks]] (forgetting to deallocate memory) and dangling [[Pointer|pointers]] ([[Pointer|pointers]] pointing to deallocated memory).

[[Heap Data Structure|Heap]] memory is also non-contiguous, meaning that blocks of memory can be scattered throughout the [[Heap Data Structure|heap]]. This contrasts with [[Stack Memory|stack memory]], which is contiguous and organized as a stack of memory blocks.

[[Heap Data Structure|Heap]] memory is crucial for many data structures and algorithms. For example, data structures such as trees and graphs are often implemented using dynamically allocated nodes on the [[Heap Data Structure|heap]]. Similarly, algorithms that require variable-size or large amounts of memory often use [[Heap Data Structure|heap]] memory.

In terms of formulas, [[Heap Data Structure|heap]] memory doesn't have specific formulas associated with it. However, the [[Big-O Notation|time complexity]] of [[Heap Data Structure|heap]] operations can be important in algorithm analysis. For example, in a [[Heap Data Structure|Heap]] Data Structure.md|binary [[Heap Data Structure|heap]] data structure]], the [[Big-O Notation|time complexity]] of insertion and deletion operations is O(log n), where n is the number of elements in the [[Heap Data Structure|heap]].

> For more information, you can refer to the following resources:
> - [Heap (data structure) - Wikipedia](https://www.google.com/search?q=Heap+(data+structure)+-+Wikipedia)
> - [Dynamic memory allocation - Wikipedia](https://www.google.com/search?q=Dynamic+memory+allocation+-+Wikipedia)
> - [Memory Management in C and C++](https://www.google.com/search?q=Memory+Management+in+C+and+C%2B%2B)
> - [Binary Heap - GeeksforGeeks](https://www.google.com/search?q=Binary+Heap+-+GeeksforGeeks)