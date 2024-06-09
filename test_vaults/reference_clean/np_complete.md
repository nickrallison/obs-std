---
bad_links: 
aliases: [NP-completeness]
tags: [theoreticalcompsci, algorithms]
---
# NP-Complete

The concept of NP-Completeness is a central topic in computational complexity theory, a branch of computer science that studies the resources (like time and space) required to solve computational problems.

## P and NP Classes

Before we delve into NP-Completeness, let's first understand the classes P and NP. 

- **P ([[Polynomial Time.md|Polynomial Time]])**: This class contains all decision problems (problems with a yes/no answer) that can be solved in polynomial time by a deterministic Turing machine. In other words, if a problem is in P, there exists an algorithm that can solve the problem in a time that is polynomial in the size of the input.

- **[[Nondeterministic Polynomial Time.md|NP]] ([[Nondeterministic Polynomial Time.md|Nondeterministic Polynomial Time]])**: This class contains all decision problems for which a given solution can be verified in polynomial time by a deterministic Turing machine. It's important to note that while all problems in P are also in NP (since if we can solve a problem in polynomial time, we can certainly verify a solution in polynomial time), the reverse is not known to be true and is, in fact, the famous P vs NP problem.

## NP-Completeness

An NP-Complete problem is a problem that is both in NP and is as hard as the hardest problems in NP. More formally, a problem is NP-Complete if:

1. It is in NP.
2. Every problem in NP is reducible to it in polynomial time.

The concept of reduction is crucial here. A problem A is said to be reducible to problem B if we can use a solution to B to solve A in polynomial time. This concept is used to show that a problem is NP-Complete.

The first problem shown to be NP-Complete was the Boolean satisfiability problem (SAT), proved by Stephen Cook in 1971. This proof is known as Cook's Theorem. After that, many other problems were shown to be NP-Complete using polynomial time reductions from other NP-Complete problems.

## Cook's Theorem

Cook's Theorem is a seminal result in computational complexity theory. It states that the Boolean satisfiability problem (SAT) is NP-Complete. The proof is quite involved and requires a deep understanding of Turing machines and formal languages. Here is a high-level overview:

1. Show that SAT is in NP. This is straightforward because given a satisfying assignment of variables, we can easily check in polynomial time if it makes the formula true.

2. Show that every problem in NP reduces to SAT in polynomial time. This is the crux of the proof. It involves simulating a Turing Machine.md|nondeterministic Turing machine]] that decides an arbitrary problem in NP and constructing a Boolean formula that is satisfiable if and only if the Turing machine accepts its input.

The proof of Cook's Theorem is beyond the scope of this response, but you can find a detailed explanation in the book "Computational Complexity: A Modern Approach" by Arora and Barak.

## Implications of NP-Completeness

If a problem is NP-Complete, then it is unlikely that there exists a polynomial time algorithm to solve it, unless P=NP, which is considered unlikely by most computer scientists. Therefore, for large instances of NP-Complete problems, we often resort to approximation algorithms or heuristics that can find good (but not necessarily optimal) solutions in polynomial time.

> For further reading, I recommend the following resources:
- [Computational Complexity: A Modern Approach](https://www.google.com/search?q=Computational+Complexity%3A+A+Modern+Approach)
- [Introduction to the Theory of Computation](https://www.google.com/search?q=Introduction+to+the+Theory+of+Computation)
- [NP-Completeness and Reduction](https://www.google.com/search?q=NP-Completeness+and+Reduction)
- [P vs NP Problem](https://www.google.com/search?q=P+vs+NP+Problem)
- [Cook's Theorem](https://www.google.com/search?q=Cook%27s+Theorem)