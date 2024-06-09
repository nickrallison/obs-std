---
bad_links: 
aliases: []
tags: [algorithms, coding]
---
# Merge Sort

Merge sort is a type of sorting algorithm that uses the divide and conquer strategy to sort lists or arrays of data. It works by dividing the unsorted list into n sublists, each containing one element (which is considered sorted), then repeatedly merging these sublists to produce new sorted sublists until there is only one sublist remaining. This final sublist is the sorted list. The efficiency of merge sort makes it useful for sorting large data sets, and it has a predictable time complexity of O(n log n).

```pseudo
\begin{algorithm}
\caption{Merge Sort}
\begin{algorithmic}
  \Procedure{MergeSort}{$Array, leftIndex, rightIndex$}
	\If{$leftIndex < rightIndex$} 
	  \State $middleIndex \gets (leftIndex + rightIndex)/2$
	  \State \Call{MergeSort}{$Array, leftIndex, middleIndex$}
	  \State \Call{MergeSort}{$Array, middleIndex+1, rightIndex$}
	  \State \Call{Merge}{$Array, leftIndex, middleIndex, rightIndex$}
	\EndIf
  \EndProcedure
  
  \Procedure{Merge}{$Array, leftIndex, middleIndex, rightIndex$}
    \State Initialize two temporary arrays $Left[]$ and $Right[]$
    \For{$i = leftindex; i <= middleindex; i++$}
      \State $Left[i] = Array[i]$
    \EndFor
    \For{$i = middleindex+1; i <= rightindex; i++$}
      \State $Right[i] = Array[i]$
    \EndFor
    \State Initialize three indices $i$, $j$, and $k$ to 0
    while($i < size(Left)$ AND $j < size(Right)$)
      if($Left[i] <= Right[j]$)
        Array[k] = Left[i]
        i++
      else
        Array[k] = Right[j]
        j++
      k++
    while($i < size(Left)$)
      Array[k] = Left[i]
      i++
      k++
    while($j < size(Right)$)
      Array[k] = Right[j]
      j++
      k++
  \EndProcedure
  
  \end{algorithmic}
\end{algorithm}
```