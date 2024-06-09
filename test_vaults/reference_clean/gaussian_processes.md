---
bad_links: 
aliases: []
tags: [probability, machinelearning]
title: Gaussian Processes
date created: Saturday, July 15th 2023, 6:47:29 pm
---
# Gaussian Processes

Gaussian Processes (GP) are a type of statistical model used in machine learning and statistics for tasks such as regression, classification, and optimization. They are based on the concept of Gaussian distribution and are particularly useful for dealing with uncertainty. GP models consider every point in the input space as a random variable, with any finite number of these variables having a joint Gaussian distribution. This allows them to provide a prediction not only for the expected output but also for the uncertainty around that prediction. They are non-parametric, meaning they can adapt their complexity to the data, and they offer flexibility in modeling relationships between inputs and outputs through the use of different kernel functions.

The concept of Gaussian Processes doesnt translate directly into a single formula, but it can be represented in several parts. Here are some key components:

1. A Gaussian Process is defined as:

$$
f(x) \sim GP(m(x), k(x, x'))
$$

where $m(x)$ is the mean function and $k(x, x)$ is the covariance function or kernel function.

1. The joint distribution of any finite number of these variables follows a multivariate Gaussian distribution:

$$
\begin{bmatrix} f(x_1) \\ f(x_2) \\ \vdots \\ f(x_n) \end{bmatrix} 
\sim \mathcal{N} 
\left( 
\begin{bmatrix} m(x_1) \\ m(x_2) \\ \vdots \\ m(x_n) \end{bmatrix}, 
\begin{bmatrix} k(x_1, x_1) & k(x_1, x_2) & \cdots & k(x_1, x_n) \\
k(x_2, x_1) & k(x_2, x_2) & \cdots & k(x_2, x_n) \\
\vdots  & \vdots  & \ddots & \vdots  \\
k(x_n, x_1) & k(x_n, x_2) & \cdots & k(x_n, x_n)
\end{bmatrix}
\right)
$$

1. The prediction for a new point $x^*$ given observed data $(X,y)$ is given by:

$$
f^* | X,y,x^* \sim  N(\mu^*,\sigma^{*2})
$$

where

$$
\mu^* = K(X,x^*)[K(X,X)+\sigma^{2}I]^{-1}y
$$

and

$$
\sigma^{*2}=K(X,X)-K(X,x^*)[K(X,X)+\sigma^{2}I]^{-1}K(X,x^*)
$$

Here $K(A,B)$ represents the kernel function matrix with elements $k(a,b)$ for all $a$ in $A$ and $b$ in $B$. $\sigma^{2}$ represents noise variance and I is the Identity (Group Theory)|identity matrix.
