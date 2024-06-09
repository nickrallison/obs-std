---
aliases: [Symmetric Matrices]
tags: [linearalgebra]
bad_links: [Symmetric Relation.md]
---
# Symmetric Matrix

A symmetric matrix is a type of square matrix where corresponding elements about the main diagonal are equal. In other words, a matrix $A$ is symmetric if it is equal to its transpose, denoted as $A = A^T$. 

The main diagonal of a matrix runs from the top left to the bottom right. For a matrix to be symmetric, the element in the i-th row and j-th column must be equal to the element in the j-th row and i-th column for all i and j. Mathematically, this can be represented as $a_{ij} = a_{ji}$ for all i and j.

Here is an example of a symmetric matrix:

$$
A = \begin{bmatrix}
a & b & c \\
b & d & e \\
c & e & f \\
\end{bmatrix}
$$

In this matrix, $a_{12} = a_{21} = b$, $a_{13} = a_{31} = c$, and $a_{23} = a_{32} = e$.

Symmetric matrices have several important properties:

1. The sum of two symmetric matrices is also a symmetric matrix.
2. The difference of two symmetric matrices is also a symmetric matrix.
3. The product of two symmetric matrices is symmetric if and only if the matrices commute (i.e., $AB = BA$).
4. If $A$ is a symmetric matrix and $k$ is a scalar, then $kA$ is also a symmetric matrix.
5. The eigenvalues of a symmetric matrix are always real.
6. Symmetric matrices are always diagonizable, and the eigenvectors corresponding to distinct eigenvalues are orthogonal.

The proof for the last property can be found in many linear algebra textbooks and online resources. Here is a [link](https://www.google.com/search?q=proof+that+symmetric+matrices+are+diagonalizable) to a Google search for the proof.

> For further reading, you might find these resources helpful:
> - [Symmetric Matrix](https://www.google.com/search?q=Symmetric+Matrix)
> - [Eigenvalues and Eigenvectors](https://www.google.com/search?q=eigenvalues+and+eigenvectors)
> - [Diagonalization of Matrices](https://www.google.com/search?q=diagonalization+of+matrices)