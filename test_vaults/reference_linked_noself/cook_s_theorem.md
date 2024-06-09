---
bad_links: 
aliases: [Cook-Levin Theorem]
tags: [theoreticalcompsci]
---
# Cook's Theorem

Cook's Theorem, also known as the Cook-Levin Theorem, is a fundamental concept in computational theory. It was proposed by Stephen Cook in 1971 and independently by Leonid Levin in 1973. The theorem is about the complexity class [[Nondeterministic Polynomial Time|NP]] ([[Nondeterministic Polynomial Time|nondeterministic polynomial time]]) and introduces the concept of [[NP-Complete|NP-completeness]].

The theorem states that there exists a problem, which if we can solve in [[Polynomial Time|polynomial time]], we can solve all problems in [[Nondeterministic Polynomial Time|NP]] in [[Polynomial Time|polynomial time]]. This problem is known as the [[Boolean Satisfiability|Boolean satisfiability]] problem ([[Boolean Satisfiability|SAT]]), and it is the first problem proven to be [[NP-Complete|NP-complete]].

The [[Boolean Satisfiability|Boolean satisfiability]] problem is defined as follows: Given a Boolean expression formed using a set of Boolean variables, AND, OR, NOT operations, and parentheses, is there a way to assign TRUE or FALSE values to the variables such that the entire expression evaluates to TRUE?

The formal statement of Cook's Theorem is:

> For every problem $L$ in [[Nondeterministic Polynomial Time|NP]], there exists a [[Polynomial Time|polynomial-time]] many-one reduction from $L$ to [[Boolean Satisfiability|SAT]].

A [[Polynomial Time|polynomial-time]] many-one reduction from a problem $L1$ to a problem $L2$ is a function $f$ that can be computed in [[Polynomial Time|polynomial time]], such that for every instance $x$ of $L1$, $x$ is in $L1$ if and only if $f(x)$ is in $L2$.

The proof of Cook's Theorem is quite involved and requires a deep understanding of [[Turing Machine|Turing machines]] and [[Formal Languages|formal languages]]. The proof constructs a specific Boolean formula that simulates the computation of a [[Alan Turing|Turing]] Machine.md|nondeterministic [[Alan Turing|Turing]] machine]], and shows that this formula is satisfiable if and only if the [[Turing Machine|Turing machine]] accepts its input.

Tangentially related items include other [[NP-Complete|NP-complete]] problems, the concept of [[P vs NP|P vs NP]], and the implications of these concepts on practical problems in computer science and other fields.

For further reading, you may want to look into the following resources:

> - ["Cook's Theorem: A Proof Sketch"](https://www.google.com/search?q=Cook%27s+Theorem%3A+A+Proof+Sketch)
> - ["The P vs NP Problem"](https://www.google.com/search?q=The+P+vs+NP+Problem)
> - ["List of NP-complete problems"](https://www.google.com/search?q=List+of+NP-complete+problems)
> - ["Implications of P vs NP"](https://www.google.com/search?q=Implications+of+P+vs+NP)