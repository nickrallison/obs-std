---
bad_links: 
aliases: [NP-completeness]
tags: [theoreticalcompsci, algorithms]
---
# NP-Complete

The concept of NP-Completeness is a central topic in computational complexity theory, a branch of computer science that studies the resources (like time and space) required to solve computational problems.

## P and [[Nondeterministic Polynomial Time|NP]] Classes

Before we delve into NP-Completeness, let's first understand the classes P and [[Nondeterministic Polynomial Time|NP]]. 

- **P ([[Polynomial Time|Polynomial Time]])**: This class contains all decision problems (problems with a yes/no answer) that can be solved in [[Polynomial Time|polynomial time]] by a deterministic [[Turing Machine|Turing machine]]. In other words, if a problem is in P, there exists an algorithm that can solve the problem in a time that is polynomial in the size of the input.

- **[[Nondeterministic Polynomial Time|NP]] ([[Nondeterministic Polynomial Time|Nondeterministic Polynomial Time]])**: This class contains all decision problems for which a given solution can be verified in [[Polynomial Time|polynomial time]] by a deterministic [[Turing Machine|Turing machine]]. It's important to note that while all problems in P are also in [[Nondeterministic Polynomial Time|NP]] (since if we can solve a problem in [[Polynomial Time|polynomial time]], we can certainly verify a solution in [[Polynomial Time|polynomial time]]), the reverse is not known to be true and is, in fact, the famous [[P vs NP|P vs NP]] problem.

## NP-Completeness

An NP-Complete problem is a problem that is both in [[Nondeterministic Polynomial Time|NP]] and is as hard as the hardest problems in [[Nondeterministic Polynomial Time|NP]]. More formally, a problem is NP-Complete if:

1. It is in [[Nondeterministic Polynomial Time|NP]].
2. Every problem in [[Nondeterministic Polynomial Time|NP]] is reducible to it in [[Polynomial Time|polynomial time]].

The concept of reduction is crucial here. A problem A is said to be reducible to problem B if we can use a solution to B to solve A in [[Polynomial Time|polynomial time]]. This concept is used to show that a problem is NP-Complete.

The first problem shown to be NP-Complete was the [[Boolean Satisfiability|Boolean satisfiability]] problem ([[Boolean Satisfiability|SAT]]), proved by Stephen Cook in 1971. This proof is known as [[Cook's Theorem|Cook's Theorem]]. After that, many other problems were shown to be NP-Complete using [[Polynomial Time|polynomial time]] reductions from other NP-Complete problems.

## [[Cook's Theorem|Cook's Theorem]]

[[Cook's Theorem|Cook's Theorem]] is a seminal result in computational complexity theory. It states that the [[Boolean Satisfiability|Boolean satisfiability]] problem ([[Boolean Satisfiability|SAT]]) is NP-Complete. The proof is quite involved and requires a deep understanding of [[Turing Machine|Turing machines]] and [[Formal Languages|formal languages]]. Here is a high-level overview:

1. Show that [[Boolean Satisfiability|SAT]] is in [[Nondeterministic Polynomial Time|NP]]. This is straightforward because given a satisfying assignment of variables, we can easily check in [[Polynomial Time|polynomial time]] if it makes the formula true.

2. Show that every problem in [[Nondeterministic Polynomial Time|NP]] reduces to [[Boolean Satisfiability|SAT]] in [[Polynomial Time|polynomial time]]. This is the crux of the proof. It involves simulating a [[Alan Turing|Turing]] Machine.md|nondeterministic [[Alan Turing|Turing]] machine]] that decides an arbitrary problem in [[Nondeterministic Polynomial Time|NP]] and constructing a Boolean formula that is satisfiable if and only if the [[Turing Machine|Turing machine]] accepts its input.

The proof of [[Cook's Theorem|Cook's Theorem]] is beyond the scope of this response, but you can find a detailed explanation in the book "Computational Complexity: A Modern Approach" by Arora and Barak.

## Implications of NP-Completeness

If a problem is NP-Complete, then it is unlikely that there exists a [[Polynomial Time|polynomial time]] algorithm to solve it, unless [[P vs NP|P=NP]], which is considered unlikely by most computer scientists. Therefore, for large instances of NP-Complete problems, we often resort to approximation algorithms or heuristics that can find good (but not necessarily optimal) solutions in [[Polynomial Time|polynomial time]].

> For further reading, I recommend the following resources:
- [Computational Complexity: A Modern Approach](https://www.google.com/search?q=Computational+Complexity%3A+A+Modern+Approach)
- [Introduction to the Theory of Computation](https://www.google.com/search?q=Introduction+to+the+Theory+of+Computation)
- [NP-Completeness and Reduction](https://www.google.com/search?q=NP-Completeness+and+Reduction)
- [P vs NP Problem](https://www.google.com/search?q=P+vs+NP+Problem)
- [Cook's Theorem](https://www.google.com/search?q=Cook%27s+Theorem)