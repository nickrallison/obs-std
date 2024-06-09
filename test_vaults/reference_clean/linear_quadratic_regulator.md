---
bad_links: 
aliases: [LQR]
tags: [controlsystems]
title: Linear Quadratic Regulator
date created: Sunday, July 16th 2023, 10:26:12 am
---
# Linear Quadratic Regulator

The Linear Quadratic Regulator (LQR) is a method used in control theory to find an optimal control law for a given system. It is designed to minimize a cost function, which is typically the sum of the state and control effort. The system being controlled must be represented by linear differential equations, and the cost function must be quadratic. The LQR provides a balance between system performance (how well the system follows a desired trajectory) and energy usage (how much control effort is used). Its widely used in engineering fields such as robotics, automation, and aerospace due to its effectiveness and simplicity.

The Linear Quadratic Regulator (LQR) problem can be formulated as follows:

$$
\begin{gather*} 
\min_{u} \quad J = \frac{1}{2} \int_{0}^{T} (x(t)^T Q x(t) + u(t)^T R u(t)) dt \\
\text{subject to: } \dot{x}(t) = Ax(t) + Bu(t)
\end{gather*}
$$

Where:

- $x$ is the state vector
- $u$ is the control vector
- $A$ and $B$ are system matrices
- $Q$ and $R$ are weight matrices for the state and control effort respectively.

Lets consider a simple one-dimensional system where a car is moving along a straight line. The state of the car is represented by its position and velocity. The control input is the force applied to the car.

The system can be represented by the following linear differential equations:

$$
\begin{gather*} 
\dot{x}_1(t) = x_2(t) \\
\dot{x}_2(t) = u(t)
\end{gather*}
$$

where $x_1$ is the position, $x_2$ is the velocity, and $u$ is the force. This can be written in matrix form as:

$$
\begin{gather*} 
\dot{x}(t) = \begin{bmatrix} 0 & 1 \\ 0 & 0 \end{bmatrix} x(t) + \begin{bmatrix} 0 \\ 1 \end{bmatrix} u(t)
\end{gather*}
$$

So, we have $A = \begin{bmatrix} 0 & 1 \\ 0 & 0 \end{bmatrix}$ and $B = \begin{bmatrix} 0 \\ 1 \end{bmatrix}$.

Lets choose $Q = I$ (the identity matrix) and $R = 1$. We want to minimize:

$$
J = \frac{1}{2}\int_{0}^{T}(x^T x + u^2) dt
$$

Using LQR theory, we know that the optimal control law is given by:

$$
u^*(t) = -R^{-1}B^TP(t)x(t)
$$

where P(t) satisfies the Riccati differential equation:

$$
-\dot{P}(t) = A^TP(t)+P(t)A-P(t)BR^{-1}B^TP(t)+Q
$$

with terminal condition $P(T)=Q$. Solving this equation gives us P(t), which we can use to find our optimal control law.

This example shows how LQR can be used to find an optimal control law for a simple system. In practice, solving the Riccati equation may require numerical methods or more complex analytical techniques.
