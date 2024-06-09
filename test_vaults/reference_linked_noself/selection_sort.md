---
bad_links: 
aliases: []
tags: [algorithms]
---
# Selection Sort

Selection sort is a simple comparison-based sorting algorithm. It works by dividing the input into a sorted and an unsorted region. The algorithm repeatedly selects the smallest (or largest, depending on the [[Ordering|ordering]]) element from the unsorted region and moves it to the end of the sorted region. This process continues until the unsorted region is empty and the sorted region has expanded to encompass the entire input. Despite its simplicity, selection sort is inefficient on large lists, and generally performs worse than other sorting algorithms like [[Insertion Sort|insertion sort]] or quick sort.

```pseudo
\begin{algorithm}
\caption{Selection Sort}
\begin{algorithmic}
  \Procedure{SelectionSort}{$Array$}
	\For{$i \gets 0$ to $length(Array) - 1$}
	  \State $minIndex \gets i$
	  \For{$j \gets i + 1$ to $length(Array)$}
	    \If{$Array[j] < Array[minIndex]$} 
		  \State $minIndex \gets j$
		\EndIf
	  \EndFor
	  \If{$minIndex \neq i$}
	    \State Swap $Array[i]$ and $Array[minIndex]$
	  \EndIf
	\EndFor
  \EndProcedure
\end{algorithmic}
\end{algorithm}
```