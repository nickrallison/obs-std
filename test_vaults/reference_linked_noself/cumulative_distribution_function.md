---
bad_links:
aliases:
  - Cumulative Density Function
tags:
  - probability
---
# Cumulative Distribution Function

The Cumulative Distribution Function (CDF) is a fundamental concept in probability theory and statistics. It describes the probability that a real-valued [[Random Variable|random variable]] X with a given probability distribution will be found at a value less than or equal to x.

Formally, the CDF of a real-valued [[Random Variable|random variable]] X is defined as:

$$
F_X(x) = P(X \leq x)
$$

This function is right-continuous, which means for every number x, the value of F(x) is equal to the limit of F(x+h) as h approaches zero from the right.

The CDF is used to characterize the probability distribution of [[Random Variable|random variables]] and has several important properties:

1. **Non-decreasing**: The function F(x) is non-decreasing, meaning that if a ≤ b, then F(a) ≤ F(b). This is because if a ≤ b, then the event {X ≤ a} is a subset of the event {X ≤ b}, and therefore, the probability of the former event cannot be greater than the probability of the latter.

2. **Normalized**: The function F(x) is normalized, meaning that as x approaches negative infinity, F(x) approaches 0, and as x approaches positive infinity, F(x) approaches 1. This is because the total probability of all possible outcomes of a [[Random Variable|random variable]] is 1.

3. **Right-continuous**: The function F(x) is right-continuous, meaning that for any number x, the value of F(x) is equal to the limit of F(x+h) as h approaches zero from the right.

The CDF is related to the [[Probability Density Function|probability density function]] ([[Probability Density Function|PDF]]) and the [[Probability Mass Function|probability mass function]] ([[Probability Mass Function|PMF]]) of a [[Random Variable|random variable]]. For a [[Continuous Distribution|continuous random variable]], the [[Probability Density Function|PDF]] is the derivative of the CDF:

$$
f_X(x) = \frac{d}{dx} F_X(x)
$$

For a [[Random Variable|discrete random variable]], the [[Probability Mass Function|PMF]] is the derivative of the CDF in the sense of differences:

$$
p_X(x) = F_X(x) - F_X(x-1)
$$

The CDF is also related to the quantile function, which is the inverse of the CDF. The quantile function Q(p) gives the value x such that P(X ≤ x) = p for a given probability p.

> For more information, you can refer to the following resources:
> - [Cumulative Distribution Function (Wikipedia)](https://www.google.com/search?q=Cumulative+Distribution+Function+site:wikipedia.org)
> - [Probability Density Function (Wikipedia)](https://www.google.com/search?q=Probability+Density+Function+site:wikipedia.org)
> - [Probability Mass Function (Wikipedia)](https://www.google.com/search?q=Probability+Mass+Function+site:wikipedia.org)
> - [Quantile Function (Wikipedia)](https://www.google.com/search?q=Quantile+Function+site:wikipedia.org)