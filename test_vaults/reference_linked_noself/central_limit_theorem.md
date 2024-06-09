---
bad_links: 
aliases: []
tags: [probability]
title: Central Limit Theorem
date created: Saturday, July 15th 2023, 7:15:17 pm
---
# Central Limit Theorem

The Central Limit Theorem (CLT) is a fundamental concept in statistics. It states that if you have a population with any shape of distribution - be it normal, skewed, uniform or even binomial - and you take sufficiently large random samples from a from the population with replacement, then the distribution of the sample means will approximate a [[Normal Distribution|normal distribution]]. This will be true regardless of the shape of the population distribution.

The theorem also asserts that as the size of your samples increases, the closer the sample means will get to a [[Normal Distribution|normal distribution]]. This is crucial in many areas of statistics as it allows us to make inferences about populations from samples. The CLT is the foundation for various statistical procedures including [[Confidence Interval|confidence intervals]] and [[Hypothesis Testing|hypothesis testing]].

A good example of the Central Limit Theorem is the [[Sum of Random Variables|sum of random variables]]. When 2 [[Random Variable|random variables]] are summed together, the result is a [[Convolution|convolution]] of the two. As this process is repeated, the normalized result approaches the [[Normal Distribution|Normal Distribution]].

It can be represented in general terms as follows:

$$
\begin{gather*}
\text{If } X_1, X_2, ..., X_n \text{ are n independent random variables with the same distribution and finite mean } \mu \text{ and [[Variance.md|variance]] } \sigma^2, \\
\text{then as } n \rightarrow \infty, \text{ the distribution of their mean } \overline{X} = \frac{1}{n}(X_1 + X_2 + ... + X_n) \\
\text{approaches a [[Normal Distribution.md|normal distribution with]]distributionwithmean } \mu \text{ and [[Variance.md|variance]] } \frac{\sigma^2}{n}.
\end{gather*}
$$
