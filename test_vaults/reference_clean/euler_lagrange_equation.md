---
bad_links: 
aliases: [lagrangian, lagrangian mechanics]
date created: Monday, June 26th 2023, 3:32:29 pm
tags: [physics, calculus]
title: Euler Lagrange Equation
---
# Euler Lagrange Equation

The Euler-Lagrange equation is a fundamental equation in the calculus of variations, which is used to find the path, curve, surface, etc., that will minimize a given function. It's extensively used in physics, particularly in the principle of least action in classical mechanics and quantum mechanics.

The Euler-Lagrange equation is given by:

$$
\frac{d}{dt} \left( \frac{\partial L}{\partial \dot{q}} \right) - \frac{\partial L}{\partial q} = 0
$$

where $L$ is the Lagrangian of the system, $q$ represents the generalized coordinates, and $\dot{q}$ is the time derivative of these coordinates.

The Lagrangian $L$ is a function that describes the dynamics of a system, and is typically of the form $L = T - V$, where $T$ is the kinetic energy and $V$ is the potential energy of the system.

The derivation of the Euler-Lagrange equation starts from the principle of least action, which states that the path taken by a system between two points in its configuration space is the one that minimizes the action $S$, defined as the integral of the Lagrangian over time:

$$
S = \int_{t_1}^{t_2} L dt
$$

To find the path that minimizes this action, we consider a small variation $\delta q$ around the true path, and require that the first variation of the action vanishes:

$$
\delta S = \int_{t_1}^{t_2} \left( \frac{\partial L}{\partial q} \delta q + \frac{\partial L}{\partial \dot{q}} \delta \dot{q} \right) dt = 0
$$

Integrating the second term by parts, we find:

$$
\delta S = \int_{t_1}^{t_2} \left( \frac{\partial L}{\partial q} - \frac{d}{dt} \frac{\partial L}{\partial \dot{q}} \right) \delta q dt = 0
$$

Since this must hold for arbitrary variations $\delta q$, we arrive at the Euler-Lagrange equation.

The Euler-Lagrange equation is a second-order differential equation that determines the evolution of the system. It is a fundamental result in classical mechanics and is also used in other areas of physics, such as quantum mechanics and field theory.

> For more in-depth understanding, you may want to check out the following resources:
> - [Euler-Lagrange Equation - Wikipedia](https://www.google.com/search?q=Euler-Lagrange+equation+Wikipedia)
> - [Calculus of Variations - Wikipedia](https://www.google.com/search?q=Calculus+of+variations+Wikipedia)
> - [Lagrangian Mechanics - Wikipedia](https://www.google.com/search?q=Lagrangian+mechanics+Wikipedia)
> - [Principle of Least Action - Wikipedia](https://www.google.com/search?q=Principle+of+least+action+Wikipedia)
> - [Euler-Lagrange Equation - Wolfram MathWorld](https://www.google.com/search?q=Euler-Lagrange+equation+Wolfram+MathWorld)

## Sources

[Video](https://www.youtube.com/watch?v=VCHFCXgYdvY)

## Video Summary

The Euler Lagrange equation, a powerful tool in variational calculus, seeks to find a function that minimizes another function. Its origins lie with Newton and Leibniz, founders of calculus in the 17th century. Its development was ignited by Newton's Principia Mathematica in 1686, which established classical mechanics and introduced his three laws of motion. Euler and Lagrange then advanced this technique, the former elaborating on the brachistochrone problem in 1733, and the latter developing a new method for analyzing mechanics problems in 1788, which derived equations of motion. Hamilton later broadened its applicability beyond classical mechanics into areas such as electromagnetism and quantum theory in 1834.

The focus of variational calculus is on path minimization, aiming to find a path or function that minimizes another function. This can include finding the shortest path or the path that minimizes the time or potential energy. The Euler Lagrange equation defines an integral to be minimized and involves solving differential equations to find the optimal solution. The integral can include terms such as incremental path length, velocity, or potential energy, depending on the problem.

The calculus of variations theory is based on the principle of making the path or function a stationary point, where the derivative of the function equals zero. Finding these optimal paths (or extremals) is achieved by setting the variation of the integral with respect to wildcard variable epsilon equal to zero. Allowing that the derivative can move inside the integral in line with Leibniz's rule, the functional f is a function of two variables, y bar and y bar prime, with the derivative taken in respect of each one. 

Equation 11 and 12 follow this principle by stating the derivatives of y bar and y bar prime with respect to epsilon, being eta and eta prime respectively. Substituting these into the primary equation, the integral can be rewritten and set equal to zero. As the wildcard epsilon approaches zero, the exact solution is approached. The equation, also known as the first variation of f, can be transformed further into its strong form by integrating a term and considering boundary conditions. This results in the Euler-Lagrange equation, recognize as being one of the most elegant equations in classical mechanics with applications extending beyond Lagrangian mechanics.
