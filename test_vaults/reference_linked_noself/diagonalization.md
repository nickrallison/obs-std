---
bad_links: 
aliases: [diagonalize, diagonalized, diagonal matrix, diagonalizable]
tags: [linearalgebra]
---
# Diagonalization

Diagonalization is a mathematical process used in linear algebra that involves transforming a square matrix into a more simplified or "diagonal" form. This is achieved by finding a change of basis which turns the matrix into this simpler form. The diagonal matrix is unique and consists of the [[Eigenvalue|eigenvalues]] of the original matrix, while the change of basis is given by the [[Eigenvectors|eigenvectors]]. Diagonalization is useful in simplifying complex calculations and in solving systems of linear equations or [[Differential Equations|differential equations]]. However, not all matrices can be diagonalized.

Given a matrix $A$, if there exists an invertible matrix $P$ such that $P^{-1}AP$ is a diagonal matrix $D$, then $A$ is said to be diagonalizable. The process of diagonalization can be represented as follows:

$$
A = PDP^{-1}
$$

The columns of $P$ are the [[Eigenvectors|eigenvectors]] of $A$, and the entries of the diagonal matrix $D$ are the corresponding [[Eigenvalue|eigenvalues]]. So, if $\lambda_{1}, \lambda_{2}, …, \lambda_{n}$ are the [[Eigenvalue|eigenvalues]] and $\mathbf{v}_1, \mathbf{v}_2, …, \mathbf{v}_n$ are corresponding [[Eigenvectors|eigenvectors]], then 

$$
D = diag(\lambda_1,\lambda_2,...,\lambda_n)
$$

and 

$$
P = [\mathbf{v}_1,\mathbf{v}_2,...,\mathbf{v}_n]
$$

where "diag" denotes a diagonal matrix with its specified entries on the main diagonal.