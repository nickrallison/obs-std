---
bad_links: 
aliases: []
tags: [controlsystems]
---
# Steady-State Error

Steady-state error (SSE) is a key concept in control systems engineering. It is the difference between the desired output (reference input) and the actual output of a system when the system has reached its steady state. The steady state is the condition of a system after it has had sufficient time to react to all inputs and the outputs have become unchanging over time.

The steady-state error is a measure of the accuracy of a control system. A smaller steady-state error indicates a more accurate system. It's important to note that the steady-state error is not necessarily zero, even for stable systems. The steady-state error depends on the type of input (step, ramp, or parabolic) and the type of system (type 0, type 1, or type 2).

The steady-state error can be calculated using the [[Final Value Theorem|Final Value Theorem]], which states that the final value of the output equals the limit as time approaches infinity of the [[Laplace Transform|Laplace transform]] of the output. The formula for the [[Final Value Theorem|Final Value Theorem]] is:

$$
\lim_{t \to \infty} f(t) = \lim_{s \to 0} sF(s)
$$

where $f(t)$ is the time-domain function and $F(s)$ is its [[Laplace Transform|Laplace transform]].

For a unity feedback control system with a [[Transfer Function|transfer function]] $G(s)$, the error $E(s)$ is given by:

$$
E(s) = R(s) - Y(s) = R(s) - G(s)E(s)
$$

where $R(s)$ is the [[Laplace Transform|Laplace transform]] of the reference input and $Y(s)$ is the [[Laplace Transform|Laplace transform]] of the output. Solving for $E(s)$ gives:

$$
E(s) = \frac{R(s)}{1 + G(s)}
$$

The steady-state error $e_{ss}$ for a step input $R(s) = 1/s$ is then:

$$
e_{ss} = \lim_{s \to 0} sE(s) = \lim_{s \to 0} \frac{s}{1 + G(s)}
$$

For a ramp input $R(s) = 1/s^2$ and a parabolic input $R(s) = 1/s^3$, the formulas are similar.

The steady-state error also depends on the type of the system. A type 0 system has a finite steady-state error for a step input, but infinite error for a ramp or parabolic input. A type 1 system has zero error for a step input, finite error for a ramp input, and infinite error for a parabolic input. A type 2 system has zero error for step and ramp inputs, and finite error for a parabolic input.

> For more information, you may want to read the following resources:
> - [Steady-State Error](https://www.google.com/search?q=Steady-State+Error)
> - [Final Value Theorem](https://www.google.com/search?q=Final+Value+Theorem)
> - [Laplace Transform](https://www.google.com/search?q=Laplace+Transform)
> - [Control Systems](https://www.google.com/search?q=Control+Systems)