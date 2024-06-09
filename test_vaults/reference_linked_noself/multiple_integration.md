---
bad_links: 
aliases: [multiple integral, triple integration]
tags: [calculus, physics]
title: Multiple Integration
date created: Wednesday, July 12th 2023, 10:50:55 am
---

# Multiple Integration

Multiple integration is a mathematical process used in calculus to compute the volume under a surface in three-dimensional space. This concept extends the application of single and double integrals, enabling us to solve more complex mathematical problems. In triple integrals, the function being integrated is a function of three variables, and the integration is carried out with respect to these three variables. The process involves integrating over a three-dimensional region, often represented as a volume in space. Applications of Multiple integration can be found in physics, engineering, and other scientific fields where calculations involving 3D objects are necessary.

Here is an example performing Multiple integration in spherical coordinates:

Suppose we want to solve the following triple integral:

$$
\iiint_E r^2 \sin(\phi) \, dV 
$$

where E is the solid bounded above by the hemisphere $x^2 + y^2 + z^2 = 4$ and below by the cone $z = \sqrt{x^2 + y^2}$.

In spherical coordinates, our volume element $dV$ becomes $\rho^2\sin(\phi)\,d\rho\,d\phi\,d\theta$. The [[Limits|limits]] of integration for $\phi$ is from $0$ to $\frac{\pi}{4}$ (due to the constraints from the cone), for $\rho$ is from $0$ to $2$, and for $\theta$ is from $0$ to $2\pi$.

Therefore, our integral becomes:

$$
\begin{gather*} 
\int_{0}^{2}\int_{0}^{(π/4)}\int_{0}^{2π} \rho^4 \sin(\phi)^2 \, d\theta \, d\phi \, d\rho\\
= 2π \int_{0}^{2}\int_{0}^{(π/4)}  ρ^4 sin(φ)^2 dφ dρ\\
= 8π/15
\end{gather*}
$$

This calculation involves using standard formulas for integrating powers of sine and cosine over their period.
