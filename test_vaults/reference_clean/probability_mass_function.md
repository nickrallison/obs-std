---
bad_links: 
aliases: [PMF]
tags: [probability]
---
# Probability Mass Function

The Probability Mass Function (PMF) is a function that gives the probability that a discrete random variable is exactly equal to some value. The probability mass function is often the primary means of defining a discrete probability distribution, and such functions exist for either scalar or multivariate random variables whose domain is discrete.

A PMF is denoted by $P(X=x)$, where $X$ is the random variable and $x$ is a particular outcome. The PMF satisfies two properties:

1. Non-negativity: For every possible outcome $x$, $P(X=x) \geq 0$.
2. Normalization: The sum of the probabilities for all possible outcomes equals 1, i.e., $\sum_x P(X=x) = 1$.

The PMF for a random variable $X$ taking on a discrete set of possible values is given by:

$$
P(X=x) = \begin{cases} 
p(x) & \text{if } x \in \text{values of } X, \\
0 & \text{otherwise}.
\end{cases}
$$

where $p(x)$ is the probability of $X$ taking the value $x$.

A related concept is the Cumulative Distribution Function (CDF), which gives the probability that a random variable is less than or equal to a certain value. It is defined as:

$$
F(x) = P(X \leq x) = \sum_{t \leq x} P(X=t)
$$

The CDF is always non-decreasing and right-continuous, which makes it a distribution function.

> For more context and reading, you can refer to the following resources:
> - [Probability Mass Function - Wikipedia](https://www.google.com/search?q=Probability+Mass+Function+site:wikipedia.org)
> - [Cumulative Distribution Function - Wikipedia](https://www.google.com/search?q=Cumulative+Distribution+Function+site:wikipedia.org)
> - [Probability Mass Function - Wolfram MathWorld](https://www.google.com/search?q=Probability+Mass+Function+site:wolfram.com)
> - [Cumulative Distribution Function - Wolfram MathWorld](https://www.google.com/search?q=Cumulative+Distribution+Function+site:wolfram.com)