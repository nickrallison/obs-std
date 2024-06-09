---
aliases: [Heap]
tags: [algorithms, datastructures]
bad_links: [Heap Memory.md]
---
# Heap Data Structure

A **Heap** is a specialized tree-based data structure that satisfies the [[Heap Property|heap property]]. It is a complete [[Binary Tree|binary tree]], which means all levels of the tree are fully filled except possibly for the last level, which is filled from left to right.

There are two types of heaps:

1. **Max-Heap**: In a Max-Heap, for any given node I, the value of I is greater than or equal to the values of its children. This property must be true across the entire tree. In other words, the parent node has a value greater than or equal to its children. The key of the root node is the maximum key in the max heap.

2. **Min-Heap**: In a Min-Heap, for any given node I, the value of I is less than or equal to the values of its children. This property must be true across the entire tree. In other words, the parent node has a value less than or equal to its children. The key of the root node is the minimum key in the min heap.

The heap data structure is used in various algorithms, including the [[Heap Sort|Heap Sort]] algorithm and in implementing efficient [[Priority Queue|priority queues]].

The primary operations performed on heaps include:

1. **Insertion**: Insertion is performed by adding the element at the bottom-most rightmost spot and then pushing it up until it's in the correct position. This operation has a [[Big-O Notation|time complexity]] of $O(\log n)$.

2. **Deletion**: Deletion is performed by removing the root and placing the bottom-most rightmost element at the root. Then, the new root is pushed down until it's in the correct position. This operation also has a [[Big-O Notation|time complexity]] of $O(\log n)$.

3. **Peek**: This operation returns the root of the heap. Since the root is at the top of the heap, this operation can be performed in constant time, $O(1)$.

The heap data structure can be implemented using an array. The parent-child relationship can be defined using array indices. If the parent node is at index $i$, then:

- The left child can be found at index $2i + 1$
- The right child can be found at index $2i + 2$

And the parent of any node at index $i$ can be found at index $\lfloor \frac{i-1}{2} \rfloor$.

> For more information, you can refer to the following resources:
> - [Heap Data Structure](https://www.google.com/search?q=Heap+Data+Structure)
> - [Heap Sort Algorithm](https://www.google.com/search?q=Heap+Sort+Algorithm)
> - [Priority Queue](https://www.google.com/search?q=Priority+Queue)
> - [Binary Tree](https://www.google.com/search?q=Binary+Tree)
> - [Complete Binary Tree](https://www.google.com/search?q=Complete+Binary+Tree)