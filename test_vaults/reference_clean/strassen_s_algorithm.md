---
bad_links: 
aliases: []
tags: [algorithms, linearalgebra]
---
# Strassen’s Algorithm

Strassen's Algorithm, proposed by Volker Strassen in 1969, is an efficient algorithm used for matrix multiplication. It is faster than the standard matrix multiplication algorithm and is based on the principle of divide and conquer.

The standard matrix multiplication for two $2 \times 2$ matrices involves 8 multiplications and 4 additions. Strassen's Algorithm reduces the number of multiplications to 7 at the cost of increasing the number of additions. This reduction in multiplications results in a significant speedup for large matrices.

Let's consider two $2 \times 2$ matrices A and B:

$$
A = \begin{bmatrix} a & b \\ c & d \end{bmatrix}, B = \begin{bmatrix} e & f \\ g & h \end{bmatrix}
$$

The standard multiplication would be:

$$
A \times B = \begin{bmatrix} ae+bg & af+bh \\ ce+dg & cf+dh \end{bmatrix}
$$

Strassen's Algorithm, however, calculates seven products (P1 to P7) and uses these to compute the final result:

$$
P1 = a \times (f - h) \\
P2 = (a + b) \times h \\
P3 = (c + d) \times e \\
P4 = d \times (g - e) \\
P5 = (a + d) \times (e + h) \\
P6 = (b - d) \times (g + h) \\
P7 = (a - c) \times (e + f)
$$

The final result is then calculated as:

$$
A \times B = \begin{bmatrix} P5 + P4 - P2 + P6 & P1 + P2 \\ P3 + P4 & P1 + P5 - P3 - P7 \end{bmatrix}
$$

For larger matrices, the algorithm recursively applies itself to smaller and smaller submatrices of the original matrices. This recursive nature leads to a time complexity of $O(n^{log_2 7})$, which is approximately $O(n^{2.81})$, better than the standard $O(n^3)$.

> For more in-depth understanding, you may want to read the original paper by Volker Strassen titled ["Gaussian elimination is not optimal"](https://scholar.google.com/scholar?hl=en&as_sdt=0%2C5&q=Gaussian+elimination+is+not+optimal&btnG=) or this detailed explanation on [GeeksforGeeks](https://www.geeksforgeeks.org/strassens-matrix-multiplication/).