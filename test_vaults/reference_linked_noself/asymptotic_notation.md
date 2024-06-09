---
bad_links: 
aliases: []
tags: [algorithms]
---
# Asymptotic Notation

Asymptotic notation is a set of mathematical notations used to describe the limiting behavior of a function when the argument tends towards a particular value or infinity, most often in terms of simpler functions. It's a way of comparing function growth rates. In computer science, it's used to describe the complexity of algorithms.

There are three main types of asymptotic notation:

1. **[[Big-O Notation|Big O Notation]] ($O$)**: This notation describes an upper bound on the [[Big-O Notation|time complexity]] of an algorithm. It provides an asymptotic upper bound for the growth rate of runtime of an algorithm. For a given function $g(n)$, we denote by $O(g(n))$ the set of functions:

$$
O(g(n)) = \{f(n): \text{there exist positive constants } c \text{ and } n_0 \text{ such that } 0 \leq f(n) \leq c \cdot g(n) \text{ for all } n \geq n_0\}
$$

1. **Omega Notation ($\Omega$)**: This notation provides an asymptotic lower bound for the growth rate of runtime of an algorithm. For a given function $g(n)$, we denote by $\Omega(g(n))$ the set of functions:

$$
\Omega(g(n)) = \{f(n): \text{there exist positive constants } c \text{ and } n_0 \text{ such that } 0 \leq c \cdot g(n) \leq f(n) \text{ for all } n \geq n_0\}
$$

1. **Theta Notation ($\Theta$)**: This notation bounds a function from above and below, so it defines exact asymptotic behavior. A function $f(n)$ has $\Theta(g(n))$ if $f(n)$ is $O(g(n))$ and $f(n)$ is also $\Omega(g(n))$. For a given function $g(n)$, we denote by $\Theta(g(n))$ the set of functions:

$$
\Theta(g(n)) = \{f(n): \text{there exist positive constants } c_1, c_2 \text{ and } n_0 \text{ such that } 0 \leq c_1 \cdot g(n) \leq f(n) \leq c_2 \cdot g(n) \text{ for all } n \geq n_0\}
$$

These notations are used to analyze and compare algorithms based on their running time and space requirements, which are critical resources in computation.

> For more in-depth reading, you might find the following resources helpful:
> - [Asymptotic Notation](https://www.google.com/search?q=Asymptotic+Notation)
> - [Big O Notation](https://www.google.com/search?q=Big+O+Notation)
> - [Omega Notation](https://www.google.com/search?q=Omega+Notation)
> - [Theta Notation](https://www.google.com/search?q=Theta+Notation)
> - [Analysis of Algorithms](https://www.google.com/search?q=Analysis+of+Algorithms)

## Sources
[[Introduction to Algorithms 4e.pdf]] - Pages 75-97
## Summary

The evaluation and analysis of algorithms often involve the use of comprehensive mathematical notations and functions. These range from asymptotic notation to polynomials, exponentials, logarithms, floor and ceilings among others. One such algorithm, the [[Insertion Sort|INSERTION-SORT]], is analyzed with a worst-case running time determined to be O(n^2). Based on this analysis, it is inferred that a minimum of n=3 values must traverse at least n=3 positions, leading to a [[Big-O Notation|time complexity]] of n^2. Such an upper bound holds true in all scenarios as there exists an input that prompts [[Insertion Sort|INSERTION-SORT]] to take approximately n^2 time.

The document further augments this concept by modifying the lower bound argument for [[Insertion Sort|insertion sort]], making it applicable to input sizes that are not confined to a multiple of 3. Analyses are also extended to the running time of the [[Selection Sort|selection sort]] algorithm using similar reasoning. Asymptotic notation, including [[Big-O Notation|O-notation]], $\Omega$-notation, and $\Theta$-notation play a significant role in this analytical process. These notations offer an estimated upper and lower bounds on a function, along with bounding a function within constant factors. 

A notable part of this discussion includes the correct use of asymptotic notation to characterize an algorithm's running time. It elaborates on the importance of using the precise notation without exaggerations and presents examples of using such notation to describe running times for algorithms such as [[Insertion Sort|insertion sort]] and [[Merge Sort|merge sort]]. The primary focus of this analysis lies in interpreting the asymptotic notation in equations and inequalities with an emphasis on its rightful application. 

The latter parts of the document provide an introduction to common mathematical functions and their relationships. The mathematical concepts covered include floors and ceilings, modular arithmetic, polynomials, exponentials, logarithms, factorials, functional iteration, the iterated logarithm function, and Fibonacci numbers. Each of these elements is elaborated upon with theories, equations, and properties. Also, essential properties like [[Transitive Relation|transitive]], [[Reflexive Relation|reflexive]], [[Symmetric Relation|symmetry]], and transpose [[Symmetric Relation|symmetry]] properties of asymptotic notation are enlightened.

Closing sections of the document revolve around diverse exercises concerning asymptotic behavior and notation. It discusses topics from proving the monotonicity of functions, properties of polynomials, [[Ordering|ordering]] by asymptotic growth rates, manipulating asymptotic notation, to variations on [[Big-O Notation|big O]] and $\Theta$ notation. Furthermore, iterated functions come under scrutiny as the document delves into bounds on iterated applications of specific functions. All in all, the discussion offers an extensive understanding of asymptotic analysis and notation, paving the way for a structured approach to [[Analyzing Algorithms|analyzing algorithms']] running time.