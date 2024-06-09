---
bad_links:
aliases: []
tags:
  - logic
---
# Symmetric [[Binary Relation|Relation]]

A [[Binary Relation|relation]] $R$ on a set $A$ is said to be symmetric if the following condition holds: for all $a, b \in A$, if $aRb$ then $bRa$. In other words, if a pair $(a, b)$ is in the [[Binary Relation|relation]] $R$, then the pair $(b, a)$ is also in the [[Binary Relation|relation]] $R$.

Mathematically, this can be expressed as:

$$
\forall a, b \in A, (a, b) \in R \Rightarrow (b, a) \in R
$$

An example of a symmetric [[Binary Relation|relation]] is the "is equal to" [[Binary Relation|relation]] (=) on the set of integers. If $a = b$, then it is also true that $b = a$.

A [[Binary Relation|relation]] that is not symmetric is said to be asymmetric. For example, the "is less than" [[Binary Relation|relation]] (<) on the set of integers is asymmetric because if $a < b$, it is not true that $b < a$.

Symmetric [[Binary Relation|relations]] are important in many areas of mathematics, including graph theory, where a symmetric [[Binary Relation|relation]] corresponds to an [[Undirected Graph|undirected graph]].

A [[Binary Relation|relation]] can be represented by a matrix, where the entry in the $i$-th row and $j$-th column is 1 if the pair $(i, j)$ is in the [[Binary Relation|relation]], and 0 otherwise. A [[Binary Relation|relation]] is symmetric if and only if its matrix is symmetric, i.e., the matrix is equal to its transpose.

The proof of this is straightforward. Let $R$ be a [[Binary Relation|relation]] on a set $A = \{a_1, a_2, …, a_n\}$, and let $M$ be the matrix representing $R$. Then $M_{ij} = 1$ if and only if $(a_i, a_j) \in R$. The matrix $M$ is symmetric if and only if $M_{ij} = M_{ji}$ for all $i, j$. But $M_{ij} = M_{ji}$ if and only if $(a_i, a_j) \in R$ if and only if $(a_j, a_i) \in R$, which is the definition of a symmetric [[Binary Relation|relation]].

> For more information, you can refer to the following resources:
> - [Symmetric Relation - Wikipedia](https://www.google.com/search?q=Symmetric+Relation+Wikipedia)
> - [Symmetric Relations - Brilliant.org](https://www.google.com/search?q=Symmetric+Relations+Brilliant.org)
> - [Symmetric, Asymmetric, and Antisymmetric Relations - Math Insight](https://www.google.com/search?q=Symmetric%2C+Asymmetric%2C+and+Antisymmetric+Relations+Math+Insight)