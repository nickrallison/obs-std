---
bad_links: 
aliases: [normalized exponential function]
title: Softmax Function
date created: Tuesday, July 11th 2023, 1:40:26 pm
tags: [algorithms, computerscience, machinelearning]
---
# Softmax Function

The Softmax function, also known as normalized exponential function, is a function that takes as input a vector of K real numbers, and normalizes it into a probability distribution consisting of K probabilities. That is, prior to applying softmax, some vector components could be negative, or greater than one; and might not sum to 1; but after applying softmax, each component will be in the interval (0,1), and the components will add up to 1, so that they can be interpreted as probabilities. Furthermore, the larger input components will correspond to larger probabilities.

The softmax function is often used in the final layer of a neural network-based classifier. Such networks are commonly trained under a log loss (or cross-entropy) regime, giving a non-linear variant of multinomial logistic regression.

The softmax function $\sigma : \mathbb{R}^K \to \mathbb{R}^K$ is defined by the formula:

$$
\sigma(\mathbf{z})_j = \frac{e^{z_j}}{\sum_{k=1}^K e^{z_k}} \quad \text{for } j = 1, \ldots, K \text{ and } \mathbf{z} = (z_1, \ldots, z_K) \in \mathbb{R}^K
$$

Where:
- $\mathbf{z}$ is the input vector (real numbers),
- $j$ is the element of the $\mathbf{z}$,
- $K$ is the total number of classes in the classification problem,
- $\sigma(\mathbf{z})_j$ is the output of the softmax function.

The denominator $\sum_{k=1}^K e^{z_k}$ acts as a normalizer to ensure that the sum of the probabilities is 1.

The softmax function is differentiable, and its derivative (the Jacobian matrix of softmax function) can be computed as follows:

$$
\frac{\partial \sigma(\mathbf{z})_i}{\partial z_j} = \sigma(\mathbf{z})_i (\delta_{ij} - \sigma(\mathbf{z})_j)
$$

Where $\delta_{ij}$ is the Kronecker delta.

The softmax function is used in various multiclass classification methods, such as multinomial logistic regression (also known as softmax regression), multiclass linear discriminant analysis, naive Bayes classifiers, and artificial neural networks.

Specifically, in neural networks, the softmax function is used as an activation function in a network layer for multiclass classification problems. The softmax function outputs a vector that represents the probability distributions of a list of potential outcomes. It's a way of normalizing the output of a network to a probability distribution over predicted output classes.

> For more in-depth understanding, you can refer to the following resources:
> - [Softmax Function - Wikipedia](https://www.google.com/search?q=Softmax+Function+-+Wikipedia)
> - [Understanding softmax and the softmax derivative](https://www.google.com/search?q=Understanding+softmax+and+the+softmax+derivative)
> - [The Softmax function and its derivative](https://www.google.com/search?q=The+Softmax+function+and+its+derivative)