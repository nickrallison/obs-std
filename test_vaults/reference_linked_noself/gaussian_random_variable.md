---
aliases: []
tags: [probability]
title: Gaussian Random Variable
date created: Saturday, July 15th 2023, 3:55:09 pm
bad_links: [Symmetric Relation.md]
---
# Gaussian [[Random Variable|Random Variable]]

A Gaussian [[Random Variable|random variable]], also known as a normal [[Random Variable|random variable]], is a type of [[Continuous Distribution|continuous random variable]] that follows a [[Gaussian Distribution|Gaussian distribution]], or [[Normal Distribution|normal distribution]]. The [[Gaussian Distribution|Gaussian distribution]] is one of the most important probability distributions in statistics due to the [[Central Limit Theorem|Central Limit Theorem]], which states that the sum of many independent and identically distributed (i.i.d.) [[Random Variable|random variables]] tends to a [[Gaussian Distribution|Gaussian distribution]], regardless of the shape of the original distribution.

The [[Probability Density Function|probability density function]] ([[Probability Density Function|PDF]]) of a Gaussian [[Random Variable|random variable]] $X$ is given by:

$$
f(x) = \frac{1}{\sqrt{2\pi\sigma^2}} e^{ -\frac{(x-\mu)^2}{2\sigma^2} }
$$

where $\mu$ is the mean or expectation of the distribution, $\sigma$ is the [[Standard Deviation|standard deviation]], and $\sigma^2$ is the [[Variance|variance]]. The factor $\frac{1}{\sqrt{2\pi\sigma^2}}$ ensures that the total probability integrates to 1.

The [[Gaussian Distribution|Gaussian distribution]] is [[Symmetric Relation|symmetric]] about its mean, and its shape is determined by the mean and [[Variance|variance]]. The mean, [[Median|median]], and mode of a [[Gaussian Distribution|Gaussian distribution]] are all equal. The distribution is bell-shaped, and about 68% of values drawn from a [[Gaussian Distribution|Gaussian distribution]] are within one [[Standard Deviation|standard deviation]] $\sigma$ away from the mean; about 95% are within two [[Standard Deviation|standard deviations]]; and about 99.7% lie within three [[Standard Deviation|standard deviations]]. This is known as the 68-95-99.7 rule, or the empirical rule.

The standard [[Normal Distribution|normal distribution]] is a special case of the [[Gaussian Distribution|Gaussian distribution]] where $\mu = 0$ and $\sigma = 1$. Any Gaussian [[Random Variable|random variable]] can be transformed to a standard normal [[Random Variable|random variable]] by the formula:

$$
Z = \frac{X - \mu}{\sigma}
$$

where $Z$ is a standard normal [[Random Variable|random variable]].

The [[Cumulative Distribution Function|cumulative distribution function]] ([[Cumulative Distribution Function|CDF]]) of a Gaussian [[Random Variable|random variable]] is given by the integral of its [[Probability Density Function|PDF]] from $-\infty$ to $x$, but it does not have a simple closed form and is often computed numerically. However, it can be expressed in terms of the error function, a special function that is defined as:

$$
\text{erf}(x) = \frac{2}{\sqrt{\pi}} \int_0^x e^{-t^2} dt
$$

Then the [[Cumulative Distribution Function|CDF]] of a Gaussian [[Random Variable|random variable]] $X$ is:

$$
F(x) = \frac{1}{2}[1 + \text{erf}(\frac{x - \mu}{\sigma\sqrt{2}})]
$$

> For more information, you may want to read the following:
> - [Gaussian Distribution](https://www.google.com/search?q=Gaussian+Distribution)
> - [Central Limit Theorem](https://www.google.com/search?q=Central+Limit+Theorem)
> - [Error Function](https://www.google.com/search?q=Error+Function)
> - [Empirical Rule](https://www.google.com/search?q=Empirical+Rule)