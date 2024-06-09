---
bad_links: 
aliases: []
tags: [algorithms]
---
# Heap Property

A heap is a specialized tree-based data structure that satisfies the heap property. This property can be of two types: 

1. **Max-[[Heap Data Structure.md|Heap]]**: For any given node I, the value of I is greater than or equal to the values of its children. This property must be recursively true for all nodes in the Heap Data Structure.md|Binary Heap]].
2. **Min-[[Heap Data Structure.md|Heap]]**: For any given node I, the value of I is less than or equal to the values of its children. This property must be recursively true for all nodes in the Heap Data Structure.md|Binary Heap]].

In a heap, the highest (max-heap) or lowest (min-heap) element is always at the "root" of the tree. Heaps are crucial in several efficient graph algorithms such as Dijkstra's algorithm, and in the sorting algorithm Heapsort.

A heap is not a sorted structure and can be regarded as partially ordered. There is no particular relationship among nodes on any given level, even among the siblings. 

When a heap is a complete binary tree, it has a smallest possible height—a heap with N nodes and for each node a branches always has log<sub>a</sub>N height. As such, it is very memory efficient.

The heap data structure provides efficient implementations of heap operations like extract-min, extract-max, delete-min, delete-max, insert, decrease-key, and merge. 

Here are the time complexities for various operations in a Heap Data Structure.md|Binary Heap]]:

- Heap Data Structure.md|Binary Heap]] Construction: $O(n)$
- Extract-Max in Max Heap or Extract-Min in Min Heap: $O(\log n)$
- Decrease-Key in Max Heap or Increase-Key in Min Heap: $O(\log n)$
- Insert: $O(\log n)$
- Delete: $O(\log n)$

The heap property is maintained after each operation, ensuring the structure remains a heap.

> For more in-depth reading, you may refer to the following resources:
> - [Heap Data Structure](https://www.google.com/search?q=Heap+Data+Structure)
> - [Heap (data structure) - Wikipedia](https://www.google.com/search?q=site:wikipedia.org+Heap+(data+structure))
> - [Binary Heap - GeeksforGeeks](https://www.google.com/search?q=site:geeksforgeeks.org+Binary+Heap)
> - [Introduction to Algorithms by Cormen, Leiserson, Rivest, and Stein](https://www.google.com/search?q=Introduction+to+Algorithms+by+Cormen,+Leiserson,+Rivest,+and+Stein)

## [[Introduction to Algorithms 4e.pdf]] - Pages 186-188 Summary

Chapter 6 of the document primarily delves into the concept of heapsort, starting with a comprehensive discussion of several questions related to max-heaps. The nature of the root node in a subtree, the probable location of the smallest element in a max-heap, and the possible levels at which the kth largest element might dwell in a max-heap are among the topics under investigation. The chapter also introduces the seemingly paradoxical question of whether a sorted array can be considered a min-heap and then challenges it further by presenting an example array, questioning its qualification as a max-heap.

The chapter further progressing, introduces the pivotal procedure known as MAX-HEAPIFY designed to maintain the max-heap property. With an array 'A' with the heap-size attribute and an index 'i' as inputs, the MAX-HEAPIFY presumes that the binary trees with roots at LEFT.i/ and RIGHT.i/ already maintain the max-heaps. This procedure then corrects any possible violation of the max-heap property by moving the smaller element down the max-heap, ensuring that the subtree rooted at index i holds true to the max-heap property.

The chapter comes to a constructive conclusion by providing pseudocode for the MAX-HEAPIFY procedure, giving readers valuable insight into its implementation. The chapter thus curated provides an expansive understanding of heapsort and the mechanism to maintain max-heap property.