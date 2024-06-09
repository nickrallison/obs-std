---
bad_links: 
aliases: []
tags: [algorithms, linearalgebra]
---
# Matrix Inversion

Matrix inversion is a mathematical process applied to matrices, which are rectangular arrays of numbers. The inverse of a matrix A is another matrix, denoted as A^-1, such that when A is multiplied by A^-1, the result is the identity matrix. The identity matrix is a special type of square matrix with 1's on the diagonal and 0's elsewhere. Not all matrices have an inverse; those that do are called invertible or nonsingular, while those that don't are called noninvertible or singular. Matrix inversion is used in various fields including physics, computer science, and engineering for solving systems of linear equations, among other applications.

## 2x2 Inversion

$$
\begin{gather*} 
A = \begin{bmatrix} a & b \\ c & d \end{bmatrix}
\end{gather*}
$$

Its inverse, if it exists, is given by:

$$
\begin{gather*} 
A^{-1} = \frac{1}{ad - bc}\begin{bmatrix} d & -b \\ -c & a \end{bmatrix}
\end{gather*}
$$

Here, $ad-bc$ is the determinant of the matrix A. If $ad-bc = 0$, then the matrix does not have an inverse.  
Note that $a$, $b$, $c$ and $d$ are numbers in the field over which the matrix is defined.

## Matrix Inversion Algorithm

The Matrix Inversion Algorithm is a step-by-step process used to find the inverse of a matrix, if it exists. The algorithm varies depending on the size of the matrix. For a 2x2 matrix, the process is relatively simple and can be done by hand, as shown in the content above. 

For larger matrices, however, the process becomes more complex and typically requires computational software. One common method for finding the inverse of a larger matrix is the identity matrix of the same size, then performing row operations until the original matrix has been transformed into an identity matrix. At this point, the augmented portion of the matrix will be the inverse of the original matrix.

Another method is through the use of minors, cofactors and adjugates. This method involves finding the determinant of smaller matrices within the original matrix (minors), applying a checkerboard of positive and negative signs (cofactors), transposing the resulting matrix (adjugate), and dividing each element by the determinant of the original matrix.

It's important to note that not all matrices have an inverse. If a matrix does not have an inverse, it is called a singular or noninvertible matrix. If a matrix does have an inverse, it is called a nonsingular or invertible matrix. The existence of an inverse depends on the determinant of the matrix. If the determinant is zero, then the matrix does not have an inverse.

### Algorithmic Time Complexity

The Big-O notation of the above matrix inverses are O(n^3). 

This is because matrix inversion is typically achieved through Gaussian elimination or similar methods, which involve a series of row operations. The time complexity of these operations is proportional to the cube of the number of rows (or columns, since the matrix is square). Hence, the Big-O notation for matrix inversion is O(n^3). 

However, it's important to note that there are certain algorithms that can perform matrix inversion in less than O(n^3) time for specific types of matrices. For example, the Strassen Algorithm can perform matrix inversion in approximately O(n^2.81) time. But for a general square matrix, the time complexity is O(n^3).