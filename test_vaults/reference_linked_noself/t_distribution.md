---
aliases: []
tags: [probability]
title: T Distribution
date created: Saturday, July 15th 2023, 4:02:59 pm
bad_links: [Symmetric Relation.md]
---
# T Distribution

The T Distribution, also known as the Student's T Distribution, is a type of probability distribution that is [[Symmetric Relation|symmetric]] and bell-shaped, similar to the [[Normal Distribution|normal distribution]], but has heavier tails. It was developed by William Sealy Gosset under the pseudonym "Student".

The T Distribution is primarily used when the sample size is small (typically less than 30), and the population [[Standard Deviation|standard deviation]] is unknown. It is a foundational aspect of inferential statistics, particularly in [[Hypothesis Testing|hypothesis testing]] and in constructing [[Confidence Interval|confidence intervals]].

The [[Probability Density Function|probability density function]] ([[Probability Density Function|pdf]]) of the T Distribution is given by:

$$
f(t) = \frac{\Gamma(\frac{v+1}{2})}{\sqrt{v\pi}\Gamma(\frac{v}{2})}\left(1+\frac{t^2}{v}\right)^{-\frac{v+1}{2}}
$$

where:
- $t$ is the [[Random Variable|random variable]],
- $v$ is the degrees of freedom (which is related to the sample size, $n$, by $v = n - 1$),
- $\Gamma$ is the [[gamma_encoding_and_decoding.md|gamma]] function.

The T Distribution approaches the standard [[Normal Distribution|normal distribution]] as the degrees of freedom increase. This is due to the Law of Large Numbers, which states that as the size of a sample increases, the estimate of the sample mean will be closer to the population mean.

The T Distribution is used in the [[T Testing|t-test]], a statistical test that compares the means of two samples. The t-statistic is calculated as:

$$
t = \frac{\bar{x} - \mu}{s/\sqrt{n}}
$$

where:
- $\bar{x}$ is the sample mean,
- $\mu$ is the population mean,
> [T Distribution - Wikipedia](https://www.google.com/search?q=T+Distribution+site:wikipedia.org)
- $s$ is the sample [[Standard Deviation|standard deviation]],
- $n$ is the sample size.

The t-statistic follows a T Distribution with $n - 1$ degrees of freedom if the null hypothesis is true.

> [T Distribution - Stat Trek](https://www.google.com/search?q=T+Distribution+site:stattrek.com)  
> [T Distribution - MathWorld](https://www.google.com/search?q=T+Distribution+site:mathworld.wolfram.com)

For further reading, you might find these resources helpful:
- [Understanding the T Distribution](https://www.google.com/search?q=Understanding+the+T+Distribution)
- [T Distribution - Khan Academy](https://www.google.com/search?q=T+Distribution+site:khanacademy.org)
- [T Distribution - Towards Data Science](https://www.google.com/search?q=T+Distribution+site:towardsdatascience.com)