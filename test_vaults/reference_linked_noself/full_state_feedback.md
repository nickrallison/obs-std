---
bad_links: 
aliases: []
tags: [controlsystems, linearalgebra]
title: Full State Feedback
date created: Saturday, July 15th 2023, 4:31:14 pm
---
# Full State Feedback

Full state feedback is a method used in control systems to regulate system behavior. It involves measuring all the [[State Space|states of a system]] and feeding them back into the controller to adjust future system inputs. This method allows for precise control of the system, as it takes into account all possible variables that could affect the system's performance. The feedback information is used to correct any errors or deviations from desired performance, making it an essential tool in maintaining [[LTI System Stability|stability]] and accuracy in various systems such as robotics, aircraft, and industrial processes.

A Full State Feedback system will be stable when all the poles of the system are placed in the left half of the s-plane. This means that the system's [[Eigenvalue|eigenvalues]] should have negative real parts. In other words, the system is stable if all its state variables are controllable and observable, and if the feedback gains are appropriately chosen to place the system's poles in the left half of the s-plane.

The poles of the full state system can be found analytically via the [[Eigenvalue|eigenvalues]] of the state update matrix $A$ when represented in [[Controllable Canonical Form|Controllable Canonical Form]]

$$
u(t) = -Kx(t)
$$

Where:
- $u(t)$ is the input to the system,
- $x(t)$ is the state vector representing all states of the system,
- $K$ is the feedback gain matrix.

The negative sign indicates that the input is adjusted in opposite direction of the error. This equation represents how each state of the system contributes to adjusting future inputs.

Full State feedback controllers may be challenging to design, one option is to design them with a [[Linear Quadratic Regulator|Linear Quadratic Regulator]], which applies a cost function to the state function and to the actuation function and the result is a control that minimizes the cost function.