---
bad_links: 
aliases: []
tags: [algorithms]
title: Perfect Hashing Algorithm
date created: Monday, July 24th 2023, 5:46:20 pm
---
# Perfect Hashing Algorithm

Perfect hashing is a technique used in computer science for the direct access of a record in a database, or an element in an array, using a computed key value. It involves two levels of hashing: the first level selects a slot based on a [[Hash Function|hash function]], and the second level uses another [[Hash Function|hash function]] to select a position within that slot. The main advantage of perfect hashing is that it eliminates the possibility of collisions (where two different keys produce the same hash), thus allowing constant [[Big-O Notation|time complexity]] for search operations

```pseudo
\begin{algorithm}
\caption{Perfect Hashing}
\begin{algorithmic}
  \Procedure{PerfectHashing}{$A$}
	\State $n \gets $ length of $A$
	\State $p \gets $ a prime number greater than $n$
	\State Choose a [[Hash Function.md|hash function]] $h$ from a universal family of [[Hash Function.md|hash functions]]
	\For{$i \gets 0$ \To $p - 1$}
	  \State Initialize an empty list at index $i$ in the hash table
	\EndFor
	\For{$j \gets 0$ \To $n - 1$}
	  \State Insert element $A[j]$ into the list at index $h(A[j])$ in the hash table
	\EndFor
	\For{$k \gets 0$ \To $p - 1$}
	  \If{the list at index $k$ in the hash table has more than one element}
	    \State Construct a secondary hash table for the elements in this list using another [[Hash Function.md|hash function]] from the universal family, with size equal to the square of the number of elements in this list.
	  \EndIf
	\EndFor
  \EndProcedure
  \end{algorithmic}
\end{algorithm}
```
