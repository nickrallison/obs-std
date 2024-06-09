---
bad_links: 
aliases: [universal hash]
tags: [algorithms]
title: Universal Hashing
date created: Monday, July 24th 2023, 6:11:00 pm
---
# Universal Hashing

Universal hashing refers to a method used in computer science to select a [[Hash Function|hash function]] randomly from a set of such functions. This technique is designed to reduce the chance of collisions (where different inputs produce the same output), thereby improving the performance of hash tables. It is particularly useful in situations where the distribution of keys is not known in advance or may change over time. Universal hashing ensures that for any given set of keys, each key has an equal probability of being hashed into any slot, independent of where other keys have hashed.

```pseudo
\begin{algorithm}
\caption{Universal Hashing}
\begin{algorithmic}
  \Procedure{UniversalHash}{$x, A, B, P, M$}
    \State $hashValue \gets 0$
    \For{$i \gets 0$ \To $sizeof(x) - 1$}
      \State $hashValue \gets (hashValue * A + x[i]) mod P$
    \EndFor
    \State $hashValue \gets (hashValue + B) mod P$
    \State $hashValue \gets (hashValue) mod M$
    \Return $hashValue$
  \EndProcedure
\end{algorithmic}
\end{algorithm}
```
In this pseudocode, `x` is the input to be hashed. `A`, `B`, `P`, and `M` are parameters of the [[Hash Function|hash function]]. `A` and `B` are randomly chosen integers less than `P`. `P` is a prime number larger than the largest number in the universe of keys. `M` is the size of the hash table. The function calculates a hash value for an input by iterating over each element in the input, multiplying the current hash value by `A`, adding the current element, and taking modulo `P`. After all elements have been processed, it adds `B`, takes modulo `P` again, and finally takes modulo `M` to ensure that the result fits into the hash table.
