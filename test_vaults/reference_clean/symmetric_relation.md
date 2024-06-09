---
bad_links: 
aliases: [symmetric, Symmetry]
tags: [logic]
---
# Symmetric Relation

A relation $R$ on a set $A$ is said to be symmetric if the following condition holds: for all $a, b \in A$, if $aRb$ then $bRa$. In other words, if a pair $(a, b)$ is in the relation $R$, then the pair $(b, a)$ is also in the relation $R$. 

Mathematically, this can be expressed as:

$$
\forall a, b \in A, (a, b) \in R \Rightarrow (b, a) \in R
$$

An example of a symmetric relation is the "is equal to" relation (=) on the set of integers. If $a = b$, then it is also true that $b = a$.

A relation that is not symmetric is said to be asymmetric. For example, the "is less than" relation (<) on the set of integers is asymmetric because if $a < b$, it is not true that $b < a$.

Symmetric relations are important in many areas of mathematics, including graph theory, where a symmetric relation corresponds to an undirected graph.

A relation can be represented by a matrix, where the entry in the $i$-th row and $j$-th column is 1 if the pair $(i, j)$ is in the relation, and 0 otherwise. A relation is symmetric if and only if its matrix is symmetric, i.e., the matrix is equal to its transpose.

The proof of this is straightforward. Let $R$ be a relation on a set $A = \{a_1, a_2, …, a_n\}$, and let $M$ be the matrix representing $R$. Then $M_{ij} = 1$ if and only if $(a_i, a_j) \in R$. The matrix $M$ is symmetric if and only if $M_{ij} = M_{ji}$ for all $i, j$. But $M_{ij} = M_{ji}$ if and only if $(a_i, a_j) \in R$ if and only if $(a_j, a_i) \in R$, which is the definition of a symmetric relation.

> For more information, you can refer to the following resources:
> - [Symmetric Relation - Wikipedia](https://www.google.com/search?q=Symmetric+Relation+Wikipedia)
> - [Symmetric Relations - Brilliant.org](https://www.google.com/search?q=Symmetric+Relations+Brilliant.org)
> - [Symmetric, Asymmetric, and Antisymmetric Relations - Math Insight](https://www.google.com/search?q=Symmetric%2C+Asymmetric%2C+and+Antisymmetric+Relations+Math+Insight)