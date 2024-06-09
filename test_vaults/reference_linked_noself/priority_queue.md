---
aliases: []
tags: [datastructures, algorithms]
bad_links: [Heap Memory.md]
---
# Priority Queue

A **Priority Queue** is an abstract data type in computer science that is similar to a queue, and every element has some priority value associated with it. The priority of the elements in a priority queue determines the deletion order. 

In a priority queue, an element with the highest priority is dequeued before an element with low priority. If two elements have the same priority, they are dequeued according to their order in the queue.

Priority queues are used in various algorithms like [[Dijkstra's Algorithm|Dijkstra's algorithm]], [[Prim's Algorithm|Prim's algorithm]], and [[Huffman Coding|Huffman coding]]. They are also used in CPU scheduling, load balancing, and interrupt handling.

Priority queues can be implemented using an array, a linked list, a [[Heap Data Structure|heap data structure]], or a [[Binary Search|binary search]] tree. Among these data structures, [[Heap Data Structure|heaps]] are commonly used because they provide the best performance characteristics. 

A **[[Heap Data Structure|Heap]]** is a special tree-based data structure that satisfies the [[Heap Property|heap property]]. In a max [[Heap Data Structure|heap]], for any given node I, the value of I is greater than or equal to the values of its children. In a min [[Heap Data Structure|heap]], for any given node I, the value of I is less than or equal to the values of its children.

The [[Big-O Notation|time complexity]] for enqueue (insertion) and dequeue (deletion) operations in a priority queue implemented using a [[Heap Data Structure|heap]] is $O(\log n)$, where $n$ is the number of elements in the priority queue.

Here is a simple implementation of a priority queue using a max [[Heap Data Structure|heap]]:

```python
import heapq

class PriorityQueue:
    def __init__(self):
        self._queue = []
        self._index = 0

    def push(self, item, priority):
        heapq.heappush(self._queue, (-priority, self._index, item))
        self._index += 1

    def pop(self):
        return heapq.heappop(self._queue)[-1]
```

In this Python code, the PriorityQueue class uses the heapq module to create a [[Heap Data Structure|Heap]] Data Structure.md|binary heap]]. The `push` method adds an item to the [[Heap Data Structure|heap]] with a given priority, and the `pop` method removes and returns the highest priority item. The `_index` variable ensures that two items with the same priority are returned in the order they were added.

> For more information, you can refer to the following resources:
> - [Priority Queue in Python](https://www.geeksforgeeks.org/priority-queue-in-python/)
> - [Heap Data Structure](https://www.geeksforgeeks.org/heap-data-structure/)
> - [Dijkstra's Algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)
> - [Prim's Algorithm](https://en.wikipedia.org/wiki/Prim%27s_algorithm)
> - [Huffman Coding](https://en.wikipedia.org/wiki/Huffman_coding)