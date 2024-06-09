---
bad_links: 
aliases: [Multiplying square matrices]
tags: [linearalgebra]
---
# Matrix Multiplication

Matrix multiplication is a binary operation that takes a pair of matrices, and produces another matrix. Numbers such as the real or complex numbers can be multiplied according to elementary arithmetic. On the other hand, matrices are arrays of numbers, so there is more than one *plausible* definition of multiplication of matrices. 

The definition used in most areas of mathematics is that provided by linear algebra, which is based on composition of linear maps that are represented by matrices. If $A$ is an $n × m$ matrix and $B$ is an $m × p$ matrix, then their matrix product $AB$ is an $n × p$ matrix, in which the $m$ entries across the rows of $A$ are multiplied with the $m$ entries down the columns of $B$ and summed to produce an entry of $AB$.

The formula for matrix multiplication is:

$$
(AB)_{ij} = \sum_{k=1}^{m} a_{ik}b_{kj}
$$

Where $a_{ik}$ is the element in the $i$th row and $k$th column of $A$, $b_{kj}$ is the element in the $k$th row and $j$th column of $B$, and $(AB)_{ij}$ is the element in the $i$th row and $j$th column of the product matrix $AB$.

Matrix multiplication has a number of properties that distinguish it from ordinary number multiplication. For example, matrix multiplication is not commutative: swapping the order of the factors $A$ and $B$ to $B$ and $A$ can result in a different product matrix, or the product might not even be defined if $A$ and $B$ are not square matrices. However, matrix multiplication is associative, meaning that the product does not change if the way in which the factors are grouped changes: $(AB)C = A(BC)$.

Matrix multiplication is also distributive, meaning that it respects addition in the following sense: $A(B + C) = AB + AC$ and $(B + C)A = BA + CA$.

The identity matrix $I$ (a square matrix with ones on the diagonal and zeros elsewhere) serves as a multiplicative identity: $AI = IA = A$ for any matrix $A$.

For a more in-depth understanding, you might want to look into the following topics:

- [Matrix (mathematics)](https://www.google.com/search?q=Matrix+(mathematics))
- [Linear algebra](https://www.google.com/search?q=Linear+algebra)
- [Associative property](https://www.google.com/search?q=Associative+property)
- [Distributive property](https://www.google.com/search?q=Distributive+property)
- [Identity matrix](https://www.google.com/search?q=Identity+matrix)

> "Matrix multiplication is a fundamental operation in many areas of mathematics, and understanding its properties can provide deep insights into the structure of mathematical systems."