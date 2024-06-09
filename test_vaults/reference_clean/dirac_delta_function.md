---
bad_links: 
aliases: []
tags: [physics]
title: Dirac Delta Function
date created: Monday, July 24th 2023, 5:30:48 pm
---
# Dirac Delta Function

The Dirac Delta Function, named after physicist Paul Dirac, is a mathematical construct used in physics and engineering. It's not a function in the traditional sense, but rather a distribution or measure. It's often represented as an infinitely high, infinitely narrow spike at zero, with total area under the curve equal to one. This function is used to model idealized point sources or sinks in various fields such as signal processing, control systems, electromagnetics and quantum mechanics. The Dirac Delta Function has the unique property that it is zero everywhere except at zero and its integral over the whole real line is equal to one.

$$
\begin{gather*} 
\delta(x) = 
\begin{cases} 
+\infty, & \text{if } x = 0 \\
0, & \text{if } x \neq 0 
\end{cases} \\
\int_{-\infty}^{\infty} \delta(x) dx = 1
\end{gather*}
$$

One use of the Dirac Delta Function is in filters and control theory. The impulse response of a filter has many special properties and is defined as being:

$$

\delta(t) \rightarrow H \rightarrow h(t)
$$

The impulse response is a common way to define 