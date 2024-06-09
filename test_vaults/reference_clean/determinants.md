---
bad_links: 
aliases: [Determinant]
tags: [linearalgebra]
title: Determinants
date created: Wednesday, July 12th 2023, 4:54:09 pm
---
# Determinants

Determinants are mathematical objects associated with square matrices. They condense the information in a matrix into a single number, which can be used to solve systems of linear equations, find the inverse of a matrix, and calculate the volume of geometric shapes. The determinant of a matrix is calculated differently depending on the size of the matrix. For a 2x2 matrix, it's simply the difference between the products of its diagonals. For larger matrices, more complex methods like Laplace expansion or cofactor expansion are used. Determinants have wide applications in various fields including physics, engineering, economics and computer graphics.

The determinant of a 2x2 matrix:

$$
\begin{gather*} 
\text{Let } A = \begin{bmatrix} a & b \\ c & d \end{bmatrix} \\
\text{Then, } \det(A) = ad - bc
\end{gather*}
$$

The determinant of a 3x3 matrix using cofactor expansion:

$$
\begin{gather*} 
\text{Let } A = \begin{bmatrix} a & b & c \\ d & e & f \\ g & h & i \end{bmatrix} \\
\text{Then, } \det(A) = aei + bfg + cdh - ceg - bdi - afh
\end{gather*}
$$
