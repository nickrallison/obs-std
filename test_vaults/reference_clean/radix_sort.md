---
bad_links: 
aliases: []
tags: [algorithms]
---
# Radix Sort

Radix sort is a non-comparative integer sorting algorithm that sorts data with integer keys by grouping keys by individual digits which share the same significant position and value. This algorithm works by processing the digits of each number one at a time from least to most significant. This categorizing process repeats for each digit place, resulting in numbers being sorted in complete order. Radix sort has linear time complexity, making it efficient for larger lists with larger number of digits.

```pseudo
\begin{algorithm}
\caption{Radix Sort}
\begin{algorithmic}
  \Procedure{RadixSort}{$Array$}
	\State Determine the maximum number to define the number of digits
	\For{each digit place $i$ from least significant to most significant}
	    \State Create buckets for each possible digit (0-9)
	    \For {each number in $Array$}
	        \State Add the number to its corresponding bucket based on its digit at place $i$
	    \EndFor
	    \State Collect numbers from the buckets, overwriting the original array
	\EndFor
  \EndProcedure
  \end{algorithmic}
\end{algorithm}
```