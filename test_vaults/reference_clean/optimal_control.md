---
bad_links: 
aliases: [optimal control law]
tags: [controlsystems]
---
# Optimal Control

Optimal control theory is a branch of mathematical optimization that deals with finding a control for a dynamical system over a period of time such that an objective function is optimized. It has numerous applications in both science and engineering.

The general form of an optimal control problem is:

$$
\begin{align*}
&\min_{u(.)}\quad J = \phi(x(t_f),t_f) + \int_{t_0}^{t_f} L(x(t),u(t),t) dt \\
&\text{subject to} \quad \dot{x}(t) = f(x(t),u(t),t), \quad x(t_0) = x_0
\end{align*}
$$

where $J$ is the cost function to be minimized, $u(t)$ is the control function to be determined, $x(t)$ is the state of the system, $f$ is the system dynamics function, $L$ is the running cost function, $\phi$ is the terminal cost function, and $t_0$ and $t_f$ are the initial and final times, respectively.

The solution to this problem involves the use of the Hamiltonian function, which is defined as:

$$
H(x,u,\lambda,t) = L(x,u,t) + \lambda^T f(x,u,t)
$$

where $\lambda$ is the costate variable. The optimal control $u^*$ and the optimal trajectory $x^*$ are found by solving the following system of differential equations, known as the Hamiltonian system:

$$
\begin{align*}
&\dot{x}^* = \frac{\partial H}{\partial \lambda} = f(x^*,u^*,t) \\
&-\dot{\lambda}^* = \frac{\partial H}{\partial x} = \frac{\partial L}{\partial x} + \lambda^T \frac{\partial f}{\partial x} \\
&0 = \frac{\partial H}{\partial u} = \frac{\partial L}{\partial u} + \lambda^T \frac{\partial f}{\partial u}
\end{align*}
$$

These are known as the costate equations and the optimality condition, respectively. The boundary conditions for the costate equations are obtained from the condition that the Hamiltonian is minimized at the final time, i.e., $\frac{\partial \phi}{\partial x}(x(t_f),t_f) = \lambda(t_f)$.

The solution to the optimal control problem is obtained by solving this two-point boundary value problem, which is typically done numerically.

> For more in-depth understanding, you may refer to the following resources:
> - [Optimal Control Theory: An Introduction by Donald E. Kirk](https://www.google.com/search?q=Optimal+Control+Theory%3A+An+Introduction+by+Donald+E.+Kirk)
> - [Principles of Optimal Control Theory by Richard Bellman](https://www.google.com/search?q=Principles+of+Optimal+Control+Theory+by+Richard+Bellman)
> - [Optimal Control and Estimation by Robert F. Stengel](https://www.google.com/search?q=Optimal+Control+and+Estimation+by+Robert+F.+Stengel)