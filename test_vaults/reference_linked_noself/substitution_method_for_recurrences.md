---
bad_links: 
aliases: [substitution method, Substitution Method for solving Recurrences]
tags: [algorithms]
---
# Substitution Method for Recurrences

The substitution method is a technique for solving [[Difference Equations|recurrence relations]], which are equations that describe a sequence based on its previous terms. This method is often used in computer science to analyze the [[Big-O Notation|time complexity]] of [[Divide and Conquer|recursive algorithms]].

The substitution method involves two steps:

1. **Guess the form of the solution**: The first step is to guess the form of the solution. This is often done based on experience or intuition. For example, if we have a [[Difference Equations|recurrence relation]] like $T(n) = T(n/2) + n$, we might guess that the solution is $T(n) = n \log n$.

2. **Use mathematical [[Induction Proofs|induction]] to find the constants and prove that the solution works**: Once we have a guess, we can use mathematical [[Induction Proofs|induction]] to prove that our guess is correct and to find any constants.

Let's illustrate this with an example. Consider the [[Difference Equations|recurrence relation]] $T(n) = T(n/2) + n$ with the base case $T(1) = 1$. We guess that the solution is of the form $T(n) = n \log n$.

To prove this by [[Induction Proofs|induction]], we need to show that our guess holds for all $n$.

**Base case**: For $n=1$, $T(1) = 1 \log 1 = 0$, which does not match our base case. However, we can add a constant to our guess to make it work. Let's guess $T(n) = n \log n - n + 1$. Now, for $n=1$, $T(1) = 1 \log 1 - 1 + 1 = 0 + 1 = 1$, which matches our base case.

**Inductive step**: Assume that our guess is correct for all values less than $n$. We need to show that it is also correct for $n$.

$T(n) = T(n/2) + n$

Substituting our guess into the recurrence gives:

$T(n) = (n/2) \log (n/2) - n/2 + 1 + n$

Simplifying this gives:

$T(n) = n \log n - n + 1$

which matches our guess, completing the proof.

The substitution method can be a powerful tool for solving recurrences, but it does rely on being able to make a good initial guess. In some cases, other methods like the [[Master Theorem|Master Theorem]] might be easier to apply.

> For more information, you can refer to the following resources:
> - ["Recurrence Relation"](https://www.google.com/search?q=Recurrence+Relation)
> - ["Mathematical Induction"](https://www.google.com/search?q=Mathematical+Induction)
> - ["Master Theorem"](https://www.google.com/search?q=Master+Theorem)