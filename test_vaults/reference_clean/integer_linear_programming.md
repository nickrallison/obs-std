---
aliases: []
tags: [algorithms]
bad_links: [Continuity.md]
---
# Integer Linear Programming

**Expert**: Operations Research Analyst  
**Objective**: To provide a comprehensive explanation of Integer Linear Programming (ILP), including related concepts, formulas, derivations, and proofs where applicable.  
**Assumptions**: You have a basic understanding of linear programming and mathematical optimization.

Integer Linear Programming (ILP) is a mathematical optimization or feasibility program in which some or all of the variables are restricted to be integers. In many settings, the term refers to integer linear programming (ILP), in which the objective function and the constraints (other than the integer constraints) are linear.

ILP is a special case of linear programming where the solution space is discrete rather than continuous. While a linear programming problem can be solved relatively easily, an integer linear programming problem is much more complex. In fact, ILP is NP-hard, meaning that there is no known algorithm that can solve all instances of the problem quickly (in polynomial time).

The general form of an ILP is as follows:

$$
\begin{align*}
\text{minimize} \quad & c^T x \\
\text{subject to} \quad & Ax \leq b \\
& x \geq 0 \\
& x \in \mathbb{Z}^n
\end{align*}
$$

where:
- $c$ is a vector of coefficients for the objective function,
- $x$ is the vector of variables we want to solve for,
- $A$ is a matrix of coefficients for the inequality constraints,
- $b$ is a vector representing the right-hand side of the inequality constraints,
- $\mathbb{Z}^n$ denotes the set of n-dimensional integer vectors.

One of the most common methods for solving ILPs is the branch-and-bound method. This method involves partitioning the solution space into smaller subsets, solving a relaxation of the problem on each subset (ignoring the integer constraints), and using the solutions to these relaxations to find the best possible integer solution.

The branch-and-bound method works as follows:

1. Solve the relaxation of the problem (ignoring the integer constraints). If the optimal solution is integer, then it is also the optimal solution to the ILP. If not, go to step 2.
2. "Branch" by partitioning the solution space into smaller subsets. This is typically done by choosing a variable that is non-integer in the optimal solution to the relaxation, and creating two new problems: one where the chosen variable is rounded down to the nearest integer, and one where it is rounded up.
3. "Bound" by solving the relaxation on each subset. If the optimal solution to a relaxation is worse than the best integer solution found so far, then that subset can be discarded. If the optimal solution is integer and better than the current best, then it becomes the new best. If the optimal solution is non-integer, then repeat from step 2 on that subset.

This process continues until all subsets have been discarded or an integer solution has been found for them, at which point the best integer solution found is the optimal solution to the ILP.

> For more in-depth reading, you may refer to the following resources:
> - [Integer Programming](https://www.google.com/search?q=Integer+Programming)
> - [Linear Programming](https://www.google.com/search?q=Linear+Programming)
> - [Branch and Bound Method](https://www.google.com/search?q=Branch+and+Bound+Method)
> - [NP-Hardness](https://www.google.com/search?q=NP-Hardness)