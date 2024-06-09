---
aliases: [heapsort]
tags: [algorithms]
bad_links: [Heap Memory.md, Heap Data Structure]
---
# Heap Sort

Heap Sort is a comparison-based sorting algorithm that uses a heap data structure. It is part of the selection sort family, but unlike simple selection sort, it does not do a linear-time scan of the unsorted region in each step. Instead, it maintains the unsorted region in a heap data structure to more quickly find the largest element in each step.

The Heap Sort algorithm can be broken down into two main parts:

1. **Heapify**: This is the process of building a heap from an array. The heap can be either a max heap (where the parent node is greater than or equal to its child nodes) or a min heap (where the parent node is less than or equal to its child nodes). For simplicity, we'll consider a max heap in this explanation. The heapify process is applied from the bottom up, starting from the first non-leaf node (which is at index `n/2 - 1`, where `n` is the number of elements in the array, assuming 0-based indexing). The heapify process ensures that the heap property (parent node >= child nodes for a max heap) is maintained for all nodes.

2. **Sorting**: Once the heap is built, the sorting process begins. The root of the heap (which is the maximum element in a max heap) is swapped with the last element of the heap. The size of the heap is reduced by one (the last element is now considered sorted). The heapify process is then reapplied to the root node to restore the heap property. This process is repeated until the heap size reduces to 1, resulting in a sorted array.

The time complexity of Heap Sort is $O(n \log n)$ for all cases (best, average, and worst), where `n` is the number of elements in the array. This is because the heapify process takes $O(\log n)$ time (as it works on the height of the tree, which is $\log n$), and we perform this process `n` times.

Heap Sort is an in-place sorting algorithm, as it only requires a constant amount of additional space. However, it is not a stable sort, meaning that equal elements may not retain their original order in the sorted array.

Here is a pseudocode representation of Heap Sort:

```
function heapSort(array)
    n = length(array)

    // Build [[Heap Data Structure.md|heap]]
    for i from n/2 - 1 to 0
        heapify(array, n, i)

    // Extract elements from [[Heap Data Structure.md|heap]]
    for i from n - 1 to 0
        // Swap
        swap array[0] with array[i]

        // Heapify root element
        heapify(array, i, 0)
```

Where `heapify` is defined as:

```
function heapify(array, n, i)
    largest = i
    left = 2*i + 1
    right = 2*i + 2

    // If left child is larger than root
    if left < n and array[left] > array[largest]
        largest = left

    // If right child is larger than largest so far
    if right < n and array[right] > array[largest]
        largest = right

    // If largest is not root
    if largest != i
        swap array[i] with array[largest]

        // Recursively heapify the affected sub-tree
        heapify(array, n, largest)
```

For more in-depth understanding, you may want to explore the following resources:

> - [Heap Sort - GeeksforGeeks](https://www.google.com/search?q=Heap+Sort+GeeksforGeeks)
> - [Heap Sort - Wikipedia](https://www.google.com/search?q=Heap+Sort+Wikipedia)
> - [Heapify - Wikipedia](https://www.google.com/search?q=Heapify+Wikipedia)
> - [Binary Heap - Wikipedia](https://www.google.com/search?q=Binary+Heap+Wikipedia)
> - [Heap Sort - Khan Academy](https://www.google.com/search?q=Heap+Sort+Khan+Academy)

## [[Introduction to Algorithms 4e.pdf]] - Pages 183-203 Summary

The chapter provides an in-depth exploration of the heapsort algorithm. The heapsort algorithm uniquely combines the benefits of both merge sort and insertion sort, marking itself as a highly efficient in-place sorting algorithm. Furthermore, the chapter introduces the concept of a heap, which is a key data structure for information management and efficient operation of a priority queue. Heaps here indicate the data structure, not the storage class. This concept and data structure are discussed further across various chapters.

Additionally, the chapter explores the heap data structure in depth. It demonstrates a Heap Data Structure.md|binary heap data structure]] as a nearly complete binary tree where each node reflects an element in an array. A heap is defined by its "heap-size" attribute, indicating the number of valid components in the heap. The structure is discussed in two categories: max-heaps and min-heaps. Various operations such as MAX-HEAPIFY and BUILD-MAX-HEAP, which have running times relative to the tree's height, are intricately detailed out. Moreover, the procedure of nodes swapping, running time of the heapsort algorithm as well as the applications of these procedures in other data structures and algorithms are carefully explored.

Further procedures of heapsort, including HEAPSORT algorithm which helps sort an array in O(n log n) time, are discussed. Key functions of the algorithm are described including building a max-heap using the BUILD-MAX-HEAP procedure, swapping the first element with the last, and calling MAX-HEAPIFY to maintain the max-heap property. Various sub-procedures in the heap, like MAX-HEAP-MAXIMUM and MAX-HEAP-EXTRACT-MAX, are extensively discussed with their running time and practical applications, including their use in scheduling jobs and event-driven simulation.

The subsequent section of the chapter examines Young tableaus, which are organized grids with sorted rows and columns. An algorithm to conduct EXTRACT-MIN on a nonempty tableau with time complexity O(m+n) is shared. Other topics include an insert operation on a nonfull tableau, an n x n Young tableau sorting n^2 numbers without the support of another sort subroutine, and an illustration of O(m+n) time complexity algorithm to determine if a specific number exists in a provided tableau.

Lastly, the chapter includes notes highlighting the origins of heapsort algorithm, the use of min-heaps in implementing min-priority queues, and references to advanced data structures like Fibonacci Heaps and Van Emde Boas Trees. The chapter concludes with a reference to research papers exploring efficient implementations for various operations. Therefore, the chapter effectively covers the heapsort algorithm procedures, its applications and, critically, the data structure that underpins it.
