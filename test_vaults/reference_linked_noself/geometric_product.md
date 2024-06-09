---
bad_links: 
aliases: []
tags: [linearalgebra, physics]
title: Geometric Product
date created: Friday, July 14th 2023, 2:58:05 pm
---
# Geometric Product

The geometric product is a fundamental concept in the mathematical field of geometric algebra. It combines aspects of both the [[Inner Product|inner product]] and the [[Outer Product|outer product]], which are operations used in geometric algebra. The geometric product of two vectors results in a new entity that contains information about both the magnitude and direction of the original vectors. This operation is useful for describing rotations and reflections in space, among other applications. The geometric product is [[Associativity|associative]] and [[Distributivity|distributive]], but not [[Commutativity|commutative]], meaning that the order in which vectors are multiplied can affect the result.

$$
\begin{gather*} 
ab = a \cdot b + a \wedge b
\end{gather*}
$$

Lets consider two vectors in 3-dimensional space, a = (1,2,3) and b = (4,5,6).

Firstly, we calculate the [[Dot Product|dot product]] ([[Inner Product|inner product]]) of a and b:

$$
\begin{gather*} 
a \cdot b = 1*4 + 2*5 + 3*6 = 32
\end{gather*}
$$

Then we calculate the [[Outer Product|outer product]] (wedge product) of a and b:

$$
\begin{gather*} 
a \wedge b = \begin{vmatrix}
i & j & k \\
1 & 2 & 3 \\
4 & 5 & 6 \\
\end{vmatrix} = -3i + 6j -3k
\end{gather*}
$$

Finally we add these together to get the geometric product:

$$
\begin{gather*} 
ab = a \cdot b + a \wedge b \\
ab = 32 -3i + 6j -3k
\end{gather*}
$$

So the geometric product of vectors a and b is given by ab = 32 -3i + 6j -3k.
