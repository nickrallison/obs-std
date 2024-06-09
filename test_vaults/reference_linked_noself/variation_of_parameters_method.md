---
bad_links: 
aliases: []
tags: [calculus]
title: Variation of Parameters Method
date created: Monday, July 17th 2023, 9:11:22 am
---
# Variation of Parameters Method

The Variation of Parameters Method is a technique used in [[Differential Equations|differential equations]] to find particular solutions. It is typically applied to non-homogeneous [[Linear Differential Equations|linear differential equations]], where the non-homogeneous term does not fit the form required for other methods such as undetermined coefficients. The method involves expressing the particular solution as a linear combination of the homogeneous solutions, with coefficients that are functions of the changing variable. These coefficient functions are then determined by solving a system of equations derived from the original [[Differential Equations|differential equation]]. This method is particularly useful when dealing with complex or irregular non-homogeneous terms.

The Variation of Parameters Method cant be represented as a single formula, but the process can be outlined as follows:

$$
\begin{gather*} 
y_p = u_1 y_1 + u_2 y_2 \\
u'_1 y_1 + u'_2 y_2 = 0 \\
u'_1 y'_1 + u'_2 y'_2 = f(x)
\end{gather*}
$$

Where:
- $y_p$ is the particular solution we are trying to find.
- $y_1$ and $y_2$ are the solutions to the homogeneous equation.
- $u_1$ and $u_2$ are functions of the changing variable that we need to determine.
- $f(x)$ is the non-homogeneous term of the [[Differential Equations|differential equation]].
