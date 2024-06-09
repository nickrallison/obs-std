---
bad_links: 
aliases: [Jacobian determinant, jacobian, Jacobian matrix, Jacobian matrices]
date created: Monday, July 10th 2023, 8:05:35 am
tags: [calculus]
title: Jacobians
---

# Jacobians

The Jacobian matrix, often simply called the Jacobian, is a matrix of all first-order partial derivatives of a vector-valued function. It plays a crucial role in many areas of mathematics, including calculus, differential equations, and dynamical systems.

The Jacobian matrix is named after the mathematician Carl Gustav Jacob Jacobi. It's a fundamental concept in vector calculus due to its role in the chain rule for the differentiation of composite functions.

Let's consider a vector-valued function $f: \mathbb{R}^n \rightarrow \mathbb{R}^m$ that maps an input vector $x \in \mathbb{R}^n$ to an output vector $f(x) \in \mathbb{R}^m$. The function $f$ is given by $f(x) = (f_1(x), f_2(x), …, f_m(x))$, where each $f_i: \mathbb{R}^n \rightarrow \mathbb{R}$ is a scalar-valued function. The Jacobian matrix $J_f(x)$ of $f$ at a point $x$ is an $m \times n$ matrix defined by:

$$
J_f(x) = \begin{bmatrix}
\frac{\partial f_1}{\partial x_1} & \cdots & \frac{\partial f_1}{\partial x_n} \\
\vdots & \ddots & \vdots \\
\frac{\partial f_m}{\partial x_1} & \cdots & \frac{\partial f_m}{\partial x_n}
\end{bmatrix}
$$

Each entry of this matrix is a partial derivative of an output component with respect to an input component. The Jacobian matrix thus encapsulates how small changes in the input can affect the output.

The determinant of a Jacobian matrix, known as the Jacobian determinant, is particularly important in change of variables for multiple integrals. If $f: \mathbb{R}^n \rightarrow \mathbb{R}^n$ is a bijective function and its Jacobian determinant at a point $x$ is non-zero, then $f$ is locally invertible near $x$. This is a consequence of the inverse function theorem.

For a deeper understanding of Jacobians, you might want to look into the following topics:

- The chain rule in multivariable calculus, which is the primary context where Jacobians appear.
- The inverse and implicit function theorems, which provide conditions under which a function has a local inverse or satisfies an implicit equation, respectively.
- The change of variables formula in multiple integration, which involves the Jacobian determinant.
- Applications of Jacobians in differential equations and dynamical systems.

> [Chain Rule](https://www.google.com/search?q=chain+rule+in+multivariable+calculus)  
> [Inverse Function Theorem](https://www.google.com/search?q=inverse+function+theorem)  
> [Implicit Function Theorem](https://www.google.com/search?q=implicit+function+theorem)  
> [Change of Variables in Multiple Integration](https://www.google.com/search?q=change+of+variables+in+multiple+integration)  
> [Applications of Jacobians](https://www.google.com/search?q=applications+of+jacobians)