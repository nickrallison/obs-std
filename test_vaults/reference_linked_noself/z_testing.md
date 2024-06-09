---
bad_links: 
aliases: [Z-Test]
tags: [probability]
---
# Z Testing

Z Testing is a statistical [[Hypothesis Testing|hypothesis testing]] method used to determine whether two population means are different when the [[Variance|variances]] are known and the sample size is large. The Z statistic follows a standard [[Normal Distribution|normal distribution]] (mean = 0, [[Standard Deviation|standard deviation]] = 1) under the null hypothesis.

The Z test statistic is calculated as follows:

$$
Z = \frac{(\bar{X} - \mu)}{\sigma / \sqrt{n}}
$$

Where:
- $\bar{X}$ is the sample mean
- $\mu$ is the population mean
- $\sigma$ is the population [[Standard Deviation|standard deviation]]
- $n$ is the sample size

The numerator, $(\bar{X} - \mu)$, represents the difference between the sample mean and the population mean. The denominator, $\sigma / \sqrt{n}$, is the standard error, which measures the statistical accuracy of an estimate, or the [[Standard Deviation|standard deviation]] of the sampling distribution.

The Z test statistic is then compared to a critical value from the standard [[Normal Distribution|normal distribution]] table, which corresponds to the desired confidence level (often 95% or 99%). If the absolute value of Z is greater than the critical value, we reject the null hypothesis.

The [[Central Limit Theorem|Central Limit Theorem]] plays a crucial role in Z Testing. It states that if you have a population with mean $\mu$ and [[Standard Deviation|standard deviation]] $\sigma$ and take sufficiently large random samples from the population with replacement, then the distribution of the sample means will be approximately normally distributed. This is the foundation of [[Hypothesis Testing|hypothesis testing]] in general.

The assumptions for the Z test are:
1. The data points are independent from each other.
2. The data follows a [[Normal Distribution|normal distribution]]. However, due to the [[Central Limit Theorem|Central Limit Theorem]], this assumption can be relaxed if the sample size is large enough (usually n > 30).
3. The population [[Standard Deviation|standard deviation]] is known. If it's not known, a [[T Testing|t-test]] might be a better choice.

> For more information, you can refer to the following resources:
> - [Z-Test: Definition, Types, Examples](https://www.google.com/search?q=Z-Test%3A+Definition%2C+Types%2C+Examples)
> - [Central Limit Theorem](https://www.google.com/search?q=Central+Limit+Theorem)
> - [Standard Normal Distribution Table](https://www.google.com/search?q=Standard+Normal+Distribution+Table)
> - [T-Test vs Z-Test](https://www.google.com/search?q=T-Test+vs+Z-Test)