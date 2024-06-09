---
bad_links: 
aliases: []
tags: [linearalgebra]
---
# Eigendecomposition

Eigendecomposition is a mathematical process used in linear algebra that involves decomposing a square matrix into a set of eigenvalues and eigenvectors. It is a crucial concept in many areas of science and engineering, including physics, computer science, and data analysis. The eigenvalues represent the magnitude of the transformation described by the matrix, while the eigenvectors represent the directions along which this transformation occurs. This decomposition provides valuable insights into the properties and behaviors of the system represented by the matrix.

Given a square matrix $\mathbf{A}$, if it has $n$ linearly independent eigenvectors, then $\mathbf{A}$ can be factorized as follows:

$$
\mathbf{A} = \mathbf{PDP^{-1}}
$$

where $\mathbf{P}$ is a matrix whose columns are the eigenvectors of $\mathbf{A}$ and $\mathbf{D}$ is a diagonal matrix whose entries are the corresponding eigenvalues. 

The $i$th column of $\mathbf{P}$ corresponds to the $i$th diagonal element of $\mathbf{D}$. 

So if we denote the $i$th eigenvector as $\overrightarrow{\mathbf{x_i}}$ and the corresponding eigenvalue as $\lambda_i$, then we have:

$$
\begin{align*}
\overrightarrow{\mathbf{x_1}} & , \overrightarrow{\mathbf{x_2}}, ..., \overrightarrow{\mathbf{x_n}} \\
\lambda_1 & , \lambda_2, ..., \lambda_n 
\end{align*}
$$
