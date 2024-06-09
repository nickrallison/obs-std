---
aliases: [binary heap]
tags: [algorithms, datastructures]
bad_links: [Binary Representation.md]
---
# Binary Heap Data Structure

A **Binary [[Heap Data Structure.md|Heap]]** is a complete binary tree which is either Min Heap or Max Heap. In a Min Binary Heap, the key at the root must be minimum among all other keys present in the Binary Heap. The same property must be recursively true for all nodes in the Binary Tree. Max Binary Heap is similar to Min Heap, but the key at the root must be maximum among all other keys present in the Binary Heap and the same property must be recursively true for all nodes.

## Properties of Binary Heap:

1. **Shape Property**: A Binary Heap is a Complete Binary Tree. That is, all levels of the tree are fully filled except for the last level. Furthermore, in the last level, the keys are all left-justified.

2. **[[Heap Property.md|Heap Property]]**: If A is a parent node of B then the key of node A is ordered with respect to the key of node B with the same ordering applying across the heap. Either the keys of parent nodes are always greater than or equal to those of the children and the key of the root node is the largest among all other nodes (Max Heap) or keys of parent nodes are less than or equal to those of the children and the key of the root node is the smallest among all other nodes (Min Heap).

## Basic Operations:

1. **getMin or getMax**: It returns the root element of Min Heap or Max Heap. Time Complexity of this operation is $O(1)$.

2. **extractMin or extractMax**: Removes the maximum element from Max Heap or minimum element from Min Heap. Time Complexity of this Operation is $O(\log n)$ as this operation needs to maintain the heap property (by calling heapify()) after removing the root.

3. **insert**: Inserting a new key takes $O(\log n)$ time. We add a new key at the end of the tree. If the new key is greater than its parent, then we swap the key with its parent. We keep comparing the new key with the parent and moving up until the parent’s key is smaller than the inserted key.

4. **delete**: Deleting a key also takes $O(\log n)$ time. We replace the key to be deleted with the minimum infinite by calling decreaseKey(). After decreaseKey(), the minus infinite value must reach the root, so we call extractMin() to remove the key.

## Binary Heap Representation:

Binary Heap is typically represented as an array. The root element will be at `Arr[0]`. For any `i`-th node in the array, if the array is zero-indexed:

- `Arr[(i-1)/2]` returns its parent node.
- `Arr[(2*i)+1]` returns its left child node.
- `Arr[(2*i)+2]` returns its right child node.

## Applications of Binary Heap:

1. Heap Sort: Heap Sort uses Binary Heap to sort an array in $O(n\log n)$ time.

2. Priority Queue: Priority queues can be efficiently implemented using Binary Heap because it supports `insert()`, `delete()` and `extractmax()`, `decreaseKey()` operations in $O(\log n)$ time.

3. Graph Algorithms: The priority queues are especially used in Graph Algorithms like Dijkstra’s Shortest Path and Prim’s Minimum Spanning Tree.

> For more detailed information, you can refer to the following resources:
> - [Binary Heap - GeeksforGeeks](https://www.google.com/search?q=Binary+Heap+GeeksforGeeks)
> - [Binary Heap - Wikipedia](https://www.google.com/search?q=Binary+Heap+Wikipedia)
> - [Heap Data Structure - Programiz](https://www.google.com/search?q=Heap+Data+Structure+Programiz)