---
bad_links: 
aliases: []
tags:
  - robotics
title: Quaternions
date created: Saturday, July 15th 2023, 7:49:31 pm
---
# Quaternions

Quaternions are a number system that extends the complex numbers. They were first described by Irish mathematician Sir William Rowan Hamilton in 1843 and are used in both pure and applied mathematics. Quaternions find uses in describing the rotation of three-dimensional objects in physics and computer graphics. A quaternion is composed of one real part and three imaginary parts, typically denoted as $w + xi + yj + zk$, where $w$, $x$, $y$, $z$ are real numbers and $i$, $j$, $k$ are the units of the quaternion.

Problem:  
Given two quaternions $q = w_1 + x_1i + y_1j + z_1k$ and $p = w_2 + x_2i + y_2j + z_2k$, rotate quaternion $q$ by quaternion $p$.

Solution:  
The rotation of a quaternion $q$ by another quaternion $p$ can be performed using the operation of quaternion multiplication. The result is given by the formula:

$$
r = pqp^{-1}
$$

where $p^{-1}$ is the inverse of quaternion $p$. The inverse of a quaternion is given by:

$$
p^{-1} = \frac{w - xi - yj - zk}{w^2 + x^2 + y^2 + z^2}
$$

Substituting this into the formula for rotation gives:

$$
r = pqp^{-1} = (w_2 + x_2i + y_2j + z_2k)(w_1 + x_1i + y_1j + z_1k)\left(\frac{w - x_{2}i - y_{2}j - z_{2}k}{w_{2}^{2}+x_{2}^{2}+y_{2}^{2}+z_{2}^{2}}\right)
$$

This can be simplified to give the final result. Note that quaternion multiplication is not [[Commutativity|commutative]], so the order in which the quaternions are multiplied matters.
