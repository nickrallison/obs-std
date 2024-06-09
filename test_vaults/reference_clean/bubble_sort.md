---
bad_links: 
aliases: []
tags: [algorithms, coding]
---
# Bubble Sort

Bubble Sort is a simple sorting algorithm used in computer programming for arranging items in a specific order. It works by repeatedly swapping the adjacent elements if they are in the wrong order. This process continues until no more swaps are needed, indicating that the list is sorted. The algorithm gets its name because with each iteration, the largest unsorted element "bubbles up" to its correct position. Despite its simplicity, Bubble Sort is not efficient for large data sets as its average and worst-case time complexity is high $O(n^2)$.

```pseudo
\begin{algorithm}
\caption{Bubble Sort Algorithm}
\begin{algorithmic}
  \Procedure{BubbleSort}{$arr$}
	\State $n \gets $ length of $arr$
	\For{$i \gets 0$ to $n-1$}
	  \For{$j \gets 0$ to $n-i-1$}
	    \If{$arr[j] > arr[j+1]$} 
		  \State Swap $arr[j]$ and $arr[j+1]$
		\EndIf
	  \EndFor
	\EndFor
  \EndProcedure
\end{algorithmic}
\end{algorithm}
```