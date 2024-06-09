---
bad_links: 
aliases: []
tags: [linearalgebra]
title: Outer Product
date created: Friday, July 14th 2023, 1:25:50 pm
---
# Outer Product

The outer product, also known as the tensor product, is a concept in linear algebra. It is an operation that takes two vectors and returns a matrix. The outer product of two vectors is calculated by multiplying each element of the first vector by each element of the second vector, resulting in a matrix whose size is determined by the lengths of the original vectors.

For example, if we have two vectors $A = [a_1, a_2]$ and $B = [b_1, b_2]$, their outer product would be a 2x2 matrix:
$$	
\begin{bmatrix} 
a_1*b_1 & a_1*b_2 \\
a_2*b_1 & a_2*b_2 \\
\end{bmatrix}
\quad	
$$
This operation is useful in various fields such as physics for describing quantum states or in computer graphics for transformations. Its important to note that unlike the dot product (which returns a single number), the outer product results in a new geometric object (a matrix).

It also generalizes to the cross-product in 3 dimensions

Here is an example of calculating the outer product of two vectors using LaTeX formatting:

Given two vectors $a = [a_1, a_2]$ and $b = [b_1, b_2]$, the outer product can be calculated as follows:

$$
\begin{gather*} 
a \otimes b = 
\begin{bmatrix}
a_1 \\
a_2
\end{bmatrix}
\begin{bmatrix}
b_1 & b_2
\end{bmatrix} \\
= 
\begin{bmatrix}
a_1*b_1 & a_1*b_2 \\
a_2*b_1 & a_2*b_2
\end{bmatrix}\\
\end{gather*}
$$

For example, if $a = [3, 4]$ and $b = [5, 6]$, the outer product would be:

$$
\begin{gather*} 
3 \otimes 5 =
\begin{bmatrix}
3 \\
4
\end{bmatrix}
\begin{bmatrix}
5 & 6
\end{bmatrix} \\
= 
\begin{bmatrix}
15 & 18 \\
20 & 24
\end{bmatrix}\\
\end{gather*}
$$
