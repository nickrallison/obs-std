---
bad_links: 
aliases: [LP]
tags: [algorithms]
---
# Linear Programming

Linear Programming (LP) is a mathematical method used to find the best (optimal) way to use limited resources to achieve a given objective. It's a technique used in operations research for solving optimization problems where both the objective function and the constraints are linear.

The general form of a linear programming problem is:

$$
\begin{align*}
\text{Maximize } & Z = c_1x_1 + c_2x_2 + \ldots + c_nx_n \\
\text{subject to } & a_{11}x_1 + a_{12}x_2 + \ldots + a_{1n}x_n \leq b_1 \\
& a_{21}x_1 + a_{22}x_2 + \ldots + a_{2n}x_n \leq b_2 \\
& \vdots \\
& a_{m1}x_1 + a_{m2}x_2 + \ldots + a_{mn}x_n \leq b_m \\
& x_1, x_2, \ldots, x_n \geq 0
\end{align*}
$$

Where:
- $Z$ is the objective function which is to be maximized or minimized.
- $x_1, x_2, \ldots, x_n$ are the decision variables.
- $c_1, c_2, \ldots, c_n$ are the coefficients of the objective function.
- $a_{ij}$ are the coefficients of the constraints.
- $b_1, b_2, \ldots, b_m$ are the right-hand side values of the constraints.

The feasible region of a linear programming problem is the set of all points that satisfy all the constraints. The optimal solution of a linear programming problem, if it exists, is always at a corner point (extreme point) of the feasible region.

The most common method for solving linear programming problems is the Simplex Method, developed by George Dantzig in 1947. The Simplex Method is an iterative procedure that starts at a feasible solution and moves towards the optimal solution by improving the objective function at each step.

For more in-depth understanding, you may want to explore the following topics related to Linear Programming:

- Duality in Linear Programming: This is a concept that every linear programming problem has an associated linear programming problem and the optimal solutions to these problems give the same optimal value. Here is a [Google search](https://www.google.com/search?q=Duality+in+Linear+Programming) for more information.
- Sensitivity Analysis in Linear Programming: This is a technique used to determine how different values of an independent variable will impact a particular dependent variable under a given set of assumptions. Here is a [Google search](https://www.google.com/search?q=Sensitivity+Analysis+in+Linear+Programming) for more information.
- Integer Linear Programming: This is a mathematical optimization or feasibility program in which some or all of the variables are restricted to be integers. Here is a [Google search](https://www.google.com/search?q=Integer+Linear+Programming) for more information.

> "Linear Programming: Introduction" from [MIT OpenCourseWare](https://ocw.mit.edu/courses/sloan-school-of-management/15-053-optimization-methods-in-management-science-spring-2013/lecture-notes/MIT15_053S13_lec02.pdf)  
> "Linear Programming" from [Wikipedia](https://en.wikipedia.org/wiki/Linear_programming)  
> "Linear Programming" from [MathWorld](http://mathworld.wolfram.com/LinearProgramming.html)