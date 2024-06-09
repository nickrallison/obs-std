---
bad_links: 
aliases: []
tags: [calculus, vectorcalculus]
title: Curl
date created: Sunday, July 16th 2023, 9:56:19 am
---
# Curl

Curl is a concept in vector calculus that measures the rotation or circulation of a [[Vector Field|vector field]]. It's often used in physics to describe the rotation of 3D phenomena such as fluid flow or electromagnetic fields. The curl at a point in the field is represented by a vector whose direction is the axis of rotation and whose magnitude is the rate of rotation. The curl operation on a [[Vector Field|vector field]] produces another [[Vector Field|vector field]] known as the curl of the original field.

$$
\begin{gather*}
\nabla \times \mathbf{F} = \left( \frac{\partial F_z}{\partial y} - \frac{\partial F_y}{\partial z} \right) \mathbf{i} + \left( \frac{\partial F_x}{\partial z} - \frac{\partial F_z}{\partial x} \right) \mathbf{j} +  \left( \frac{\partial F_y}{\partial x} - \frac{\partial F_x}{\partial y} \right)  \mathbf{k}
\end{gather*}
$$

Let's consider a [[Vector Field|vector field]] F = (2xz, -y^2, z^3). We want to find the curl of this [[Vector Field|vector field]].

$$
\begin{gather*}
\nabla \times \mathbf{F} \\ \left( \frac{\partial F_z}{\partial y} - \frac{\partial F_y}{\partial z} \right) \mathbf{i} + \left( \frac{\partial F_x}{\partial z} - \frac{\partial F_z}{\partial x} \right) \mathbf{j} +  \left( \frac{\partial F_y}{\partial x} - \frac{\partial F_x}{\partial y} \right)  \mathbf{k}\\
\\ (0 - (-2y))i + ((2x) - 0)j + (0 - 0)k\\
\\ 2yi + 2xj\\
\text{So, the curl of the [[Vector Field.md|vector field F]]fieldFis (2y, 2x, 0).}
\end{gather*}
$$
