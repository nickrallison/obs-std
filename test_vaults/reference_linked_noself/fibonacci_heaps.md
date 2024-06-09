---
bad_links: 
aliases: []
tags: [datastructures]
---
# Fibonacci [[Heap Data Structure|Heaps]]

Fibonacci [[Heap Data Structure|Heaps]] are a type of data structure for [[Priority Queue|priority queue]] operations, consisting of a collection of heap-ordered trees. They are named after the Fibonacci sequence, which is used in their running time analysis. Fibonacci [[Heap Data Structure|Heaps]] were developed by Michael L. Fredman and Robert E. Tarjan in 1984.

The Fibonacci [[Heap Data Structure|Heap data structure]] has a faster amortized running time for many operations than binary or binomial [[Heap Data Structure|heaps]]. The key operations in Fibonacci [[Heap Data Structure|Heaps]] are `insert`, `find minimum`, `extract minimum`, `decrease key`, and `delete`. 

The structure of a Fibonacci [[Heap Data Structure|Heap]] is a collection of trees satisfying the minimum [[Heap Property|heap property]] (each node's key is greater than or equal to the key of its parent), where each tree is a [[Rooted Tree|rooted tree]]. The roots of these trees are connected using a circular, doubly linked list. Each node contains a [[Pointer|pointer]] to one of its children, and the children are linked together in a circular, doubly linked list. 

The trees in a Fibonacci [[Heap Data Structure|Heap]] are allowed to be of any shape, which is a property that makes Fibonacci [[Heap Data Structure|Heaps]] more efficient than other [[Heap Data Structure|heap]] types for certain operations. 

The Fibonacci [[Heap Data Structure|Heap]] has a lazy approach to consolidating trees. When nodes are removed from the [[Heap Data Structure|heap]], the trees are not immediately restructured. Instead, the restructuring is postponed until the next time the nodes are needed. This lazy approach is what allows Fibonacci [[Heap Data Structure|Heaps]] to have better amortized running times for certain operations.

Here are the time complexities for the key operations in a Fibonacci [[Heap Data Structure|Heap]]:

- `Insert`: $O(1)$
- `Find Minimum`: $O(1)$
- `Extract Minimum`: $O(\log n)$ amortized
- `Decrease Key`: $O(1)$ amortized
- `Delete`: $O(\log n)$ amortized

The Fibonacci [[Heap Data Structure|Heap]] is named after the Fibonacci sequence because of the way that the sequence is used in the analysis of the `Extract Minimum` and `Delete` operations. The maximum degree $D(n)$ of any node in an $n$-node Fibonacci [[Heap Data Structure|Heap]] is $O(\log n)$, and it is shown that $D(n)$ is actually very close to the integer part of $A \log_{\phi} n$, where $A$ is a constant and $\phi$ is the golden ratio.

The Fibonacci [[Heap Data Structure|Heap]] has many applications in graph algorithms, including [[Dijkstra's Algorithm|Dijkstra's algorithm]] and [[Prim's Algorithm|Prim's algorithm]] for [[Minimum Spanning Tree|minimum spanning trees]].

> For more in-depth information, you can refer to the original paper by Fredman and Tarjan: ["Fibonacci heaps and their uses in improved network optimization algorithms"](https://scholar.google.com/scholar?hl=en&as_sdt=0%2C5&q=Fibonacci+heaps+and+their+uses+in+improved+network+optimization+algorithms&btnG=)  
> You can also refer to the [Fibonacci Heap entry](https://en.wikipedia.org/wiki/Fibonacci_heap) on Wikipedia for a more detailed explanation and diagrams.