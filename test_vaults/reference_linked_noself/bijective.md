---
bad_links: 
aliases: [bijection]
tags: [proofs]
title: Bijective
date created: Monday, July 24th 2023, 5:16:13 pm
---
# Bijective

A bijective function, also known as a bijection or one-to-one correspondence, is a function that is both [[Injective|injective]] (or one-to-one) and [[Surjective|surjective]] (or onto). This means that every element in the domain corresponds to exactly one element in the codomain, and every element in the codomain has exactly one corresponding element in the domain. In other words, a bijective function is a function that pairs each input with a unique output and covers all possible outputs. This concept is fundamental in areas of mathematics such as set theory, algebra, and calculus.

[[Injective|Injective]] (or one-to-one):

$$
\forall x_1, x_2 \in X, f(x_1) = f(x_2) \Rightarrow x_1 = x_2
$$

[[Surjective|Surjective]] (or onto):

$$
\forall y \in Y, \exists x \in X : f(x) = y
$$

Therefore, a function $f: X \rightarrow Y$ is bijective if it satisfies both conditions:

$$
\begin{gather*} 
\forall x_1, x_2 \in X, f(x_1) = f(x_2) \Rightarrow x_1 = x_2 \\
\forall y \in Y, \exists x \in X : f(x) = y 
\end{gather*}
$$
