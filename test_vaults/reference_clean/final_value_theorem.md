---
bad_links: 
aliases: []
tags: [controlsystems, differentialequations]
---
# Final Value Theorem

The Final Value Theorem (FVT) is a fundamental concept in control systems engineering and signal processing. It is used to determine the final value of a system's response in the time domain by examining its Laplace Transform in the frequency domain. 

The theorem is stated as follows:

If $F(s)$ is the Laplace Transform of $f(t)$, and $sF(s)$ has no poles in the right half-plane except possibly at $s=0$, then:

$$
\lim_{t \to \infty} f(t) = \lim_{s \to 0} sF(s)
$$

This theorem is particularly useful in control systems to determine the steady-state response of a system without having to perform the inverse Laplace Transform.

The proof of the Final Value Theorem is based on the properties of the Laplace Transform. Here is a simplified version:

1. Start with the definition of the Laplace Transform:

$$
F(s) = \int_{0}^{\infty} f(t)e^{-st} dt
$$

1. Multiply both sides by $s$:

$$
sF(s) = s\int_{0}^{\infty} f(t)e^{-st} dt
$$

1. Take the limit as $s$ approaches $0$:

$$
\lim_{s \to 0} sF(s) = \lim_{s \to 0} s\int_{0}^{\infty} f(t)e^{-st} dt
$$

1. As $s$ approaches $0$, $e^{-st}$ approaches $1$, so the right-hand side simplifies to:

$$
\lim_{s \to 0} sF(s) = \lim_{t \to \infty} f(t)
$$

This completes the proof.

A tangentially related concept is the Initial Value Theorem, which is used to find the initial value of a function in the time domain by examining its Laplace Transform in the frequency domain. The theorem is stated as follows:

If $F(s)$ is the Laplace Transform of $f(t)$, and $sF(s)$ has no poles in the right half-plane except possibly at $s=0$, then:

$$
\lim_{t \to 0} f(t) = \lim_{s \to \infty} sF(s)
$$

> For further reading, you may want to explore the following resources:
> - [Final Value Theorem - Wikipedia](https://www.google.com/search?q=Final+Value+Theorem+Wikipedia)
> - [Laplace Transforms - Paul's Online Math Notes](https://www.google.com/search?q=Laplace+Transforms+Paul%27s+Online+Math+Notes)
> - [Control Systems - MIT OpenCourseWare](https://www.google.com/search?q=Control+Systems+MIT+OpenCourseWare)