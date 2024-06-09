---
bad_links: 
aliases: []
tags: [calculus, vectorcalculus]
title: Divergence
date created: Sunday, July 16th 2023, 9:52:45 am
---
# Divergence

Divergence in calculus is a vector operation that operates on a [[Vector Field|vector field]], which results in a scalar field giving the quantity of the [[Vector Field|vector field's]] source at each point. More technically, divergence represents the volume density of the outward flux of a [[Vector Field|vector field]] from an infinitesimal volume around a given point. It is used in various fields such as fluid dynamics, electromagnetism, and heat conduction. The divergence of a [[Vector Field|vector field]] is denoted as div F or ∇ • F where F is the [[Vector Field|vector field]].

$$
\begin{gather*} 
\text{div} \mathbf{F} \text{ or } \nabla \cdot \mathbf{F}
\end{gather*}
$$

Sure, let's consider a [[Vector Field|vector field]] F = (2x, 3y, z^2). We want to find the divergence of this [[Vector Field|vector field]].

The formula for the divergence of a [[Vector Field|vector field]] F = (P, Q, R) in three dimensions is:

$$
\begin{gather*} 
\nabla \cdot \mathbf{F} = \frac{\partial P}{\partial x} + \frac{\partial Q}{\partial y} + \frac{\partial R}{\partial z}
\end{gather*}
$$

Substituting P = 2x, Q = 3y and R = z^2 into the formula we get:

$$
\begin{gather*} 
\nabla \cdot \mathbf{F} = \frac{\partial (2x)}{\partial x} + \frac{\partial (3y)}{\partial y} + \frac{\partial (z^2)}{\partial z}\\
= 2 + 3 + 2z\\
= 5 + 2z
\end{gather*}
$$

So the divergence of the [[Vector Field|vector field]] F = (2x, 3y, z^2) is ∇ • F = 5 + 2z.
