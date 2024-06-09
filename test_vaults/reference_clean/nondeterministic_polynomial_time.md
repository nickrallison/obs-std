---
bad_links: 
aliases: [NP]
tags: [theoreticalcompsci]
---
# Nondeterministic Polynomial Time

Nondeterministic Polynomial Time (NP) is a complexity class in computational complexity theory. It contains decision problems for which a given solution can be verified as correct or incorrect in polynomial time by a deterministic Turing machine. In other words, if you're given a "yes" answer to a problem, you can check the validity of this answer in polynomial time.

The formal definition of NP is the set of decision problems for which a solution can be verified in polynomial time by a deterministic Turing machine, given an appropriate certificate (a hint or solution sketch). Mathematically, a decision problem is in NP if there exists a verifier $V$ and a polynomial $p$ such that for any instance $I$ of the problem and certificate $C$:

1. If $I$ is a 'yes' instance, then there exists a certificate $C$ of length polynomial in the size of $I$ such that $V(I, C)$ accepts.
2. If $I$ is a 'no' instance, then for all certificates $C$ of length polynomial in the size of $I$, $V(I, C)$ rejects.

The concept of NP is closely related to the P vs NP problem, one of the most important open problems in computer science. The class P contains decision problems that a deterministic Turing machine can solve in polynomial time. If P equals NP, it would mean that every problem for which a solution can be verified in polynomial time can also be solved in polynomial time. However, it is currently unknown whether P equals NP or not.

A problem is NP-complete if it is in NP and every problem in NP can be reduced to it in polynomial time. The concept of NP-completeness, introduced by Stephen Cook and Leonid Levin, is a central topic in computational complexity theory. The first problem proven to be NP-complete is the Boolean satisfiability problem (SAT).

> For more context and reading, you can refer to the following resources:
> - [Introduction to the Theory of Computation by Michael Sipser](https://www.google.com/search?q=Introduction+to+the+Theory+of+Computation+by+Michael+Sipser)
> - [Computational Complexity: A Modern Approach by Sanjeev Arora and Boaz Barak](https://www.google.com/search?q=Computational+Complexity%3A+A+Modern+Approach+by+Sanjeev+Arora+and+Boaz+Barak)
> - [The P vs NP Problem (Clay Mathematics Institute)](https://www.google.com/search?q=The+P+vs+NP+Problem+Clay+Mathematics+Institute)
> - [The Complexity Zoo](https://www.google.com/search?q=The+Complexity+Zoo)