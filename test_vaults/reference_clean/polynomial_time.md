---
bad_links: 
aliases: []
tags: [theoreticalcompsci]
---
# Polynomial Time

Polynomial time is a term used in computational complexity theory, a branch of computer science, to describe the computational complexity of an algorithm. Specifically, an algorithm is said to run in polynomial time if its running time is upper bounded by a polynomial expression in the size of the input for the algorithm.

The class of problems for which a polynomial time algorithm exists is known as P (Polynomial time). This class is central to the P vs NP problem, one of the most important open questions in computer science.

The running time of an algorithm is usually expressed using Big O notation, which describes the upper bound of the complexity in the worst-case scenario. An algorithm is said to be polynomial time if its complexity is $O(n^k)$, where $n$ is the size of the input and $k$ is a constant. For example, an algorithm that runs in $O(n^2)$ or $O(n^3)$ time is polynomial time, but an algorithm that runs in $O(2^n)$ time is not, as $2^n$ is an exponential function, not a polynomial.

It's important to note that not all polynomial time algorithms are practical. If the degree of the polynomial is large, then the algorithm can still be very slow. However, in practice, many important problems can be solved by algorithms that run in polynomial time.

> For more in-depth reading, you might find the following resources useful:
> - [Polynomial Time (Wikipedia)](https://www.google.com/search?q=Polynomial+Time+site:wikipedia.org)
> - [P vs NP Problem (Wikipedia)](https://www.google.com/search?q=P+vs+NP+Problem+site:wikipedia.org)
> - [Computational Complexity Theory (Stanford Encyclopedia of Philosophy)](https://www.google.com/search?q=Computational+Complexity+Theory+site:plato.stanford.edu)
> - [Introduction to the Theory of Computation (Michael Sipser)](https://www.google.com/search?q=Introduction+to+the+Theory+of+Computation+Michael+Sipser)