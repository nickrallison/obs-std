---
aliases: []
tags: [algorithms, coding]
bad_links: [Binary Representation.md]
---
# Binary Search

Binary Search is a [[Divide and Conquer|divide-and-conquer]] algorithm used for searching a sorted array or list. It works by repeatedly dividing the search interval in half. If the value of the search key is less than the item in the middle of the interval, the algorithm continues on the lower half. Otherwise, it continues on the upper half. This process continues until the value is found or the interval is empty.

The binary search algorithm can be written as follows:

```
function binary_search(A, n, T) is
    L := 0
    R := n − 1
    while L ≤ R do
        m := floor((L + R) / 2)
        if A[m] < T then
            L := m + 1
        else if A[m] > T then
            R := m - 1
        else:
            return m
    return unsuccessful
```

In this pseudocode, `A` is the sorted array, `n` is the size of the array, `T` is the target value we're searching for, `L` and `R` are the left and right bounds of the search, and `m` is the midpoint.

The [[Big-O Notation|time complexity]] of Binary Search is $O(\log n)$, where $n$ is the number of elements in the array. This is because with each comparison, the algorithm eliminates half of the elements from consideration. The [[Big-O Notation|space complexity]] is $O(1)$, as it only requires a constant amount of space to store the variables.

The proof of [[Total Correctness|correctness]] for Binary Search is based on the [[Loop Invariant|loop invariant]], which is a condition that is initially true and remains true after each iteration. For Binary Search, the [[Loop Invariant|loop invariant]] is that the target value `T` is in the current search space. Before the loop starts, the search space is the entire array, so the invariant is true. If the search space becomes empty, the loop ends, and we conclude that `T` is not in the array.

> For more information, you can refer to the [Binary Search Wikipedia page](https://www.google.com/search?q=site:wikipedia.org+Binary+Search) or this [Binary Search tutorial](https://www.google.com/search?q=Binary+Search+tutorial).

```pseudo
\begin{algorithm}
\caption{Binary Search}
\begin{algorithmic}
  \Procedure{BinarySearch}{$Array, target$}
	\State $low \gets 0$
	\State $high \gets length(Array) - 1$
	\While{$low \leq high$}
	  \State $mid \gets low + (high - low) / 2$
	  \If{$Array[mid] = target$} 
		  \State return $mid$
		\ElseIf{$Array[mid] < target$}
		  \State $low \gets mid + 1$
		\Else
		  \State $high \gets mid - 1$
		\EndIf
	\EndWhile
	\State return $-1$ \Comment{Target not found}
  \EndProcedure
\end{algorithmic}
\end{algorithm}
```