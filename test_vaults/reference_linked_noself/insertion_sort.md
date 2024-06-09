---
bad_links:
aliases: []
tags: [algorithms]
---
# Insertion Sort

## Pseudocode
```pseudo
\begin{algorithm}
\caption{Insertion Sort}
\begin{algorithmic}
  \Procedure{InsertionSort}{$array$}
	\For{$i = 1$ to $length(array)$}
	  \State $key \gets array[i]$
	  \State $j \gets i - 1$
	  \While{$j \geq 0$ and $array[j] > key$}
	    \State $array[j + 1] \gets array[j]$
	    \State $j \gets j - 1$
	  \EndWhile
	  \State $array[j + 1] \gets key$
	\EndFor
  \EndProcedure
\end{algorithmic}
\end{algorithm}
```

## Explanation

Insertion sort is a simple sorting algorithm that works by repeatedly inserting elements from an unsorted portion of the array into their correct position in a sorted portion of the array. Here's how it works:

1. Start with the second element in the array (index 1) and compare it with the first element (index 0).
2. If the second element is smaller, swap it with the first element to put them in the correct order.
3. Move to the third element (index 2) and compare it with the elements before it (index 1 and 0). Insert it in the correct position by shifting the larger elements to the right.
4. Repeat this process for all the remaining elements in the array, comparing each element with the elements before it and inserting it in the correct position.
5. At the end of this process, the array will be sorted in ascending order.

Here's a step-by-step example to illustrate the insertion sort algorithm:

Initial array: \[5, 2, 4, 6, 1, 3\]

1. Compare 2 with 5. Since 2 is smaller, swap them: \[2, 5, 4, 6, 1, 3\]
2. Compare 4 with 5. Since 4 is smaller, swap them: \[2, 4, 5, 6, 1, 3\]
3. Compare 6 with 5. Since 6 is larger, no swap is needed: \[2, 4, 5, 6, 1, 3\]
4. Compare 1 with 6, 5, 4, and 2. Shift them to the right to make space for 1: \[1, 2, 4, 5, 6, 3\]
5. Compare 3 with 6, 5, 4, and 2. Shift them to the right to make space for 3: \[1, 2, 3, 4, 5, 6\]

The final sorted array is \[1, 2, 3, 4, 5, 6\].

Insertion sort has a [[Big-O Notation|time complexity]] of O(n^2) in the worst case, where n is the number of elements in the array. It is an efficient algorithm for small input sizes or partially sorted arrays.