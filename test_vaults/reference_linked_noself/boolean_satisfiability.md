---
bad_links: 
aliases: [SAT]
tags: [logic, algorithms]
---
# Boolean Satisfiability

The Boolean Satisfiability Problem (often abbreviated as SAT) is a decision problem in computer science. Essentially, it's the problem of determining if there exists an interpretation that satisfies a given Boolean formula. In other words, it asks whether the variables of a given Boolean formula can be consistently replaced by the values TRUE or FALSE in such a way that the formula evaluates to TRUE. If this is the case, the formula is said to be satisfiable. On the other hand, if no such assignment exists, the function expressed by the formula is FALSE for all possible variable assignments and the formula is unsatisfiable.

For example, let's take a simple Boolean expression: (A AND B). This expression is satisfiable because we can find an assignment for A and B (both TRUE) that makes it TRUE.

The SAT problem is significant in theoretical computer science because it was one of the first problems proven to be [[NP-Complete|NP-complete]]. This means that all problems in class [[Nondeterministic Polynomial Time|NP]], which includes many problems in logical and numerical reasoning, are at most as difficult to solve as SAT. There is no known algorithm that efficiently solves each SAT problem, and it is generally believed that no such algorithm exists; yet this belief has not been proven mathematically.

Despite this theoretical difficulty, algorithms for solving SAT in practical situations have been developed and are used in many applications including model checking, planning, software testing and optimization research.

An exponential time solution to the SAT problem can be found using [[Integer Linear Programming|Integer Linear Programming]] (ILP).