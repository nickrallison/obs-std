---
bad_links: 
aliases: []
tags: [algorithms]
---
# Quicksort

## Pseudocode

```pseudo
\begin{algorithm}
\caption{Quicksort Algorithm}
\begin{algorithmic}
  \Procedure{QuickSort}{$arr, low, high$}
	\If{$low < high$}
	  \State $pi \gets$ \Call{Partition}{$arr, low, high$}
	  \State \Call{QuickSort}{$arr, low, pi - 1$}
	  \State \Call{QuickSort}{$arr, pi + 1, high$}
	\EndIf
  \EndProcedure
  
  \Procedure{Partition}{$arr, low, high$}
    \State $pivot \gets arr[high]$
    \State $i \gets (low - 1)$
    \For{$j \gets low$ to $high - 1$}
      \If{$arr[j] \leq pivot$}
        \State $i \gets i + 1$
        \State Swap $arr[i]$ and $arr[j]$
      \EndIf
    \EndFor
    \State Swap $arr[i + 1]$ and $arr[high]$
    \State \Return $i + 1$
  \EndProcedure
  
  \end{algorithmic}
\end{algorithm}
```

## Explanation

Quicksort is a [[Divide and Conquer|divide-and-conquer]] algorithm for sorting arrays. It was developed by British computer scientist Tony Hoare in 1959 and published in 1961. The algorithm works by selecting a 'pivot' element from the array and partitioning the other elements into two sub-arrays, according to whether they are less than or greater than the pivot. The sub-arrays are then recursively sorted.

Here's a high-level description of the algorithm:

1. **Choose a pivot**: Select an element from the array to serve as a 'pivot'. The choice of pivot can greatly affect the algorithm's performance. Common strategies include choosing the first element, the last element, the [[Median|median]], or a random element.

2. **Partition**: Rearrange the elements, so all elements less than the pivot come before, and all elements greater than the pivot come after it. This step is called partitioning. After this step, the pivot is in its final position.

3. **Recursively sort the sub-arrays**: The sub-arrays of elements less than and greater than the pivot are recursively sorted.

The base case for the recursion is an array of size zero or one, which is always sorted.

The partition operation is critical to the algorithm's performance because it is performed in linear time (O(n)) and does not require additional space (in-place). Here's a simple implementation of the partition operation:

```python
def partition(arr, low, high):
    pivot = arr[high]  # pivot
    i = low - 1  # index of smaller element
    for j in range(low, high):
        if arr[j] < pivot:
            i = i + 1
            arr[i], arr[j] = arr[j], arr[i]
    arr[i + 1], arr[high] = arr[high], arr[i + 1]
    return i + 1
```

The [[Big-O Notation|time complexity]] of Quicksort is $O(n^2)$ in the worst case, when the array is already sorted or reverse sorted, and the pivot is the smallest or largest element. However, this situation is rare, and the [[Expected Value|average]] [[Big-O Notation|time complexity]] is $O(n \log n)$, which makes Quicksort one of the fastest sorting algorithms in practice.

The [[Big-O Notation|space complexity]] of Quicksort is $O(\log n)$ due to the recursive stack. This makes Quicksort more space-efficient than other sorting algorithms like [[Merge Sort|Merge Sort]].

The formal proof of Quicksort's [[Total Correctness|correctness]] relies on the [[Loop Invariant|loop invariant]], which states that at the start of each iteration of the main loop, the elements less than the pivot come before the elements greater than the pivot. This invariant is maintained during each iteration, and therefore, the array is sorted when the loop terminates.

> For more in-depth reading, you can refer to the following resources:
> - [Quicksort - Wikipedia](https://www.google.com/search?q=Quicksort+Wikipedia)
> - [Quicksort - GeeksforGeeks](https://www.google.com/search?q=Quicksort+GeeksforGeeks)
> - [Analysis of Quicksort - Khan Academy](https://www.google.com/search?q=Analysis+of+Quicksort+Khan+Academy)
> - [Quicksort - MIT OpenCourseWare](https://www.google.com/search?q=Quicksort+MIT+OpenCourseWare)