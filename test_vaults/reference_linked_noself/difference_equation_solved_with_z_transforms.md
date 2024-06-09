---
bad_links: 
aliases: []
tags: [signalprocessing]
title: Difference Equation Solved With Z-Transforms
date created: Monday, July 10th 2023, 7:44:40 pm
---
# Difference Equation Solved With [[Z-Transform|Z-Transform]]

Given the difference equation:
$$
\begin{gather*}
y[n] - 0.5y[n-1] = x[n]
\end{gather*}
$$

Taking [[Z-Transform|Z-transform]] on both sides, we get:
$$
\begin{gather*}
Y(z) - 0.5z^{-1}Y(z) = X(z)
\end{gather*}
$$

Solving for Y(z), we get the system function H(z):
$$
\begin{gather*}
H(z) = \frac{Y(z)}{X(z)} = \frac{1}{1-0.5z^{-1}} = \frac{z}{z-0.5}
\end{gather*}
$$

To find the inverse [[Z-Transform|z-transform]], we rewrite H(z) as follows:
$$
\begin{gather*}
H(z) = \frac{z}{z} + \frac{-0.5}{z-0.5} = 1 - \frac{-0.5}{1 - z/2}
\end{gather*}
$$

Using the geometric series formula for inverse [[Z-Transform|Z-transforms]] (where |z| > a):
$$
\begin{gather*}
Z^{-1}\left(\frac{-a}{1-z/a}\right) = (-a)(a^n)
\end{gather*}
$$

We find the [[Impulse Response|impulse response]] $h[n]$:
$$
\begin{gather*}
h[n] = Z^{-1}(H(Z))= δ[n] + ((-0.5)^n )u[n]
\end{gather*}
$$

Where δ is the unit impulse function and u is the unit step function.  
So, the solution to this difference equation in time domain is $δ[n] + ((-0.5)^n )u[n]$.

Remember that this analysis assumes that all sequences are right-sided and that initial conditions are zero.
