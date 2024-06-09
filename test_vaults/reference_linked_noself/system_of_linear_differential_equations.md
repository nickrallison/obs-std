---
bad_links: 
aliases: []
tags: [differentialequations]
---
# System of [[Linear Differential Equations|Linear Differential Equations]]

A system of [[Linear Differential Equations|linear differential equations]] is a set of two or more related [[Differential Equations|differential equations]] involving the same set of variables. These systems often arise in the mathematical modeling of physical phenomena, such as the motion of multiple interacting bodies or the change in populations of different species that interact with each other.

The general form of a system of $n$ first-order [[Linear Differential Equations|linear differential equations]] is:

$$
\begin{align*}
\frac{dx_1}{dt} &= a_{11}x_1 + a_{12}x_2 + \ldots + a_{1n}x_n + f_1(t) \\
\frac{dx_2}{dt} &= a_{21}x_1 + a_{22}x_2 + \ldots + a_{2n}x_n + f_2(t) \\
&\vdots \\
\frac{dx_n}{dt} &= a_{n1}x_1 + a_{n2}x_2 + \ldots + a_{nn}x_n + f_n(t)
\end{align*}
$$

where $x_1, x_2, \ldots, x_n$ are the dependent variables, $t$ is the independent variable, $a_{ij}$ are constants, and $f_i(t)$ are given functions of $t$.

This system can be written in matrix form as:

$$
\frac{d\mathbf{x}}{dt} = A\mathbf{x} + \mathbf{f}(t)
$$

where $\mathbf{x} = (x_1, x_2, \ldots, x_n)^T$ is the vector of dependent variables, $A$ is the matrix of coefficients $a_{ij}$, and $\mathbf{f}(t) = (f_1(t), f_2(t), \ldots, f_n(t))^T$ is the vector of given functions.

The solution to this system involves finding the functions $x_1(t), x_2(t), \ldots, x_n(t)$ that satisfy all the equations in the system. The methods for solving such systems depend on the properties of the matrix $A$ and the vector $\mathbf{f}(t)$.

If $\mathbf{f}(t) = \mathbf{0}$, the system is homogeneous, and its solutions can be found by finding the [[Eigenvalue|eigenvalues]] and [[Eigenvectors|eigenvectors]] of the matrix $A$. If $\mathbf{f}(t) \neq \mathbf{0}$, the system is nonhomogeneous, and its solutions can be found by methods such as variation of parameters or undetermined coefficients.

> For more detailed information, you may want to refer to the following resources:
> - [System of Linear Differential Equations](https://www.google.com/search?q=System+of+Linear+Differential+Equations)
> - [Eigenvalues and Eigenvectors](https://www.google.com/search?q=Eigenvalues+and+Eigenvectors)
> - [Variation of Parameters](https://www.google.com/search?q=Variation+of+Parameters)
> - [Undetermined Coefficients](https://www.google.com/search?q=Undetermined+Coefficients)