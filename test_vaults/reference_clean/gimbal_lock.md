---
bad_links: 
aliases: []
tags: [robotics, computergraphics]
title: Gimbal Lock
date created: Saturday, July 15th 2023, 7:40:07 pm
---
# Gimbal Lock

Gimbal lock is a term used in the field of 3D computer graphics, robotics and aviation. It refers to the loss of one degree of freedom in a three-dimensional, three-gimbal mechanism that occurs when the axes of two of the three gimbals are driven into a parallel configuration, "locking" the system into rotation in a degenerate two-dimensional space. This can limit the functionality and movement, creating problems in control systems. It's named after an issue that can occur with a physical gimbal, a pivoted support that allows rotation of an object around a single axis.

$$
\begin{gather*} 
\theta = \text{rotation about the z-axis} \\
\phi = \text{rotation about the y-axis} \\
\psi = \text{rotation about the x-axis}
\end{gather*}
$$

When $\theta$ and $\phi$ (or any two) become aligned, we experience Gimbal Lock.
