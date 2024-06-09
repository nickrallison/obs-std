---
bad_links: 
aliases: []
tags: [probability]
title: Singular Value Decomposition
date created: Saturday, July 15th 2023, 7:01:18 pm
---
# Singular Value Decomposition

Singular Value Decomposition (SVD) is a method used in linear algebra that decomposes a matrix into three other matrices. It is particularly useful in many practical applications such as signal processing, image compression, and machine learning. The SVD of a matrix provides insight into the structure of the original matrix, revealing information about its rank, range and null space. It transforms complex or large-scale data into a simpler format that can be easily interpreted and analyzed.

The Singular Value Decomposition of a matrix $A$ can be written as:

$$
\begin{gather*} 
A = U \Sigma V^T 
\end{gather*}
$$

Where:
- $A$ is the original matrix.
- $U$ is an $m \times m$ unitary matrix.
- $\Sigma$ is an $m \times n$ [[Diagonalization|diagonal matrix]] with non-negative real numbers on the diagonal.
- $V^T$ (the transpose of V) is an $n \times n$ unitary matrix.

Lets consider a 2x2 matrix A as follows:

$$
\begin{gather*} 
A = \begin{bmatrix} 4 & 11 \\ 2 & 7 \end{bmatrix}
\end{gather*}
$$

The Singular Value Decomposition of this matrix would be calculated as follows:

First, we calculate the [[Eigenvalue|eigenvalues]] and [[Eigenvectors|eigenvectors]] of $AA^T$ and $A^TA$. The [[Eigenvalue|eigenvalues]] of both will be the same, and they will give us the square of the singular values. The [[Eigenvectors|eigenvectors]] will give us the columns of U and V.

In this case, we have:

$$
\begin{gather*} 
AA^T = \begin{bmatrix} 4 & 11 \\ 2 & 7 \end{bmatrix}\begin{bmatrix} 4 & 2 \\ 11 & 7 \end{bmatrix} = \begin{bmatrix}137 &85 \\85&53\end{bmatrix}\\
A^TA = \begin{bmatrix}4&2\\11&7\end{bmatrix}\begin{bmatrix}4&11\\2&7\end{bmatrix}= \begin{bmatrix}20&50\\50&170\end{bmatrix}
\end{gather*}
$$

The [[Eigenvalue|eigenvalues]] of these matrices are approximately {5.46498,184.535}. Taking square roots gives us singular values in Σ: {√5.46498, √184.535}, or approximately {2.34035,13.5893}.

The corresponding normalized [[Eigenvectors|eigenvectors]] for $AA^T$ (which form U) are approximately {[-0.817415, -0.576048], [0.576048,-0.817415]}.

The corresponding normalized [[Eigenvectors|eigenvectors]] for $A^TA$ (which form V) are approximately {[0.404554,-0.914514], [0.914514,0.404554]}.

So our SVD decomposition is:

$$
\begin{gather*}
U &= [-0.817415, -0.576048; \\
    &   0.576048,-0.817415] \\
\Sigma &= [13.5893 ,   0; \\
    &       0 ,   -2.34035] \\
V^T &= [   -0.404554 ,   -0.914514; \\
    &      -0 .914514 ,    -0 .404554]
\end{gather*}
$$

So,

$$
\begin{gather*}
A = U\Sigma V^T 
= [-0 .817415 ,   - .576048;\\ 
     .576048 ,   -.817415]\\
[13 .5893 ,     .00000;\\
     .00000 ,   -2 .34035]\\
[ -.404554 , -.914514;\\
-.914514 ,- .404554]
\end{gather*}
$$
