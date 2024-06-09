---
aliases: []
tags: [probability]
title: Gaussian Random Variable
date created: Saturday, July 15th 2023, 3:55:09 pm
bad_links: [Symmetric Relation.md]
---
# Gaussian Random Variable

A Gaussian random variable, also known as a normal random variable, is a type of continuous random variable that follows a Gaussian distribution, or normal distribution. The Gaussian distribution is one of the most important probability distributions in statistics due to the Central Limit Theorem, which states that the sum of many independent and identically distributed (i.i.d.) random variables tends to a Gaussian distribution, regardless of the shape of the original distribution.

The probability density function (PDF) of a Gaussian random variable $X$ is given by:

$$
f(x) = \frac{1}{\sqrt{2\pi\sigma^2}} e^{ -\frac{(x-\mu)^2}{2\sigma^2} }
$$

where $\mu$ is the mean or expectation of the distribution, $\sigma$ is the standard deviation, and $\sigma^2$ is the variance. The factor $\frac{1}{\sqrt{2\pi\sigma^2}}$ ensures that the total probability integrates to 1.

The Gaussian distribution is symmetric about its mean, and its shape is determined by the mean and variance. The mean, median, and mode of a Gaussian distribution are all equal. The distribution is bell-shaped, and about 68% of values drawn from a Gaussian distribution are within one standard deviation $\sigma$ away from the mean; about 95% are within two standard deviations; and about 99.7% lie within three standard deviations. This is known as the 68-95-99.7 rule, or the empirical rule.

The standard normal distribution is a special case of the Gaussian distribution where $\mu = 0$ and $\sigma = 1$. Any Gaussian random variable can be transformed to a standard normal random variable by the formula:

$$
Z = \frac{X - \mu}{\sigma}
$$

where $Z$ is a standard normal random variable.

The cumulative distribution function (CDF) of a Gaussian random variable is given by the integral of its PDF from $-\infty$ to $x$, but it does not have a simple closed form and is often computed numerically. However, it can be expressed in terms of the error function, a special function that is defined as:

$$
\text{erf}(x) = \frac{2}{\sqrt{\pi}} \int_0^x e^{-t^2} dt
$$

Then the CDF of a Gaussian random variable $X$ is:

$$
F(x) = \frac{1}{2}[1 + \text{erf}(\frac{x - \mu}{\sigma\sqrt{2}})]
$$

> For more information, you may want to read the following:
> - [Gaussian Distribution](https://www.google.com/search?q=Gaussian+Distribution)
> - [Central Limit Theorem](https://www.google.com/search?q=Central+Limit+Theorem)
> - [Error Function](https://www.google.com/search?q=Error+Function)
> - [Empirical Rule](https://www.google.com/search?q=Empirical+Rule)