---
bad_links: 
tags: [controlsystems, linearalgebra]
aliases: []
---
# Controllable Canonical Form

The controllable canonical form is a specific representation of a linear system in control theory, used to analyze and design control systems, particularly used to design [[Full State Feedback|Full State Feedback]] control systems. It provides a standard model to describe the dynamic behavior of a system and its response to external inputs. This form is particularly useful because if a system can be transformed into this form, it implies that the system is controllable, meaning that by using appropriate inputs, it can be brought to any desired state in a finite time. The controllable canonical form simplifies the design of controllers and the analysis of system behavior.

The controllable canonical form of a [[Linear Time Invarient Systems|linear time-invariant]] system can be represented using the following matrices:

$$
\begin{gather*} 
A_c = \begin{bmatrix}
0 & 1 & 0 & \cdots & 0 \\
0 & 0 & 1 & \cdots & 0 \\
\vdots & \vdots & \vdots & \ddots & \vdots \\
0 & 0 & 0 & \cdots & 1 \\
-a_0 & -a_1 & -a_2& \cdots& -a_{n-1}
\end{bmatrix}
\end{gather*}
$$
$$
\begin{gather*} 
B_c = \begin{bmatrix}
0 \\ 
0 \\ 
\vdots \\ 
1
\end{bmatrix}
\end{gather*}
$$
$$
\begin{gather*} 
C_c = \begin{bmatrix}
0, 0, 0 ..., 0, 1
\end{bmatrix}
\end{gather*}
$$
$$
\begin{gather*} 
D_c = 0
\end{gather*}
$$

Where $a_i$ and $b_i$ are the coefficients of the characteristic polynomial and [[Transfer Function|transfer function]] of the system respectively.

