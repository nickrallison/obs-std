---
bad_links: 
aliases: []
tags: [probability]
---
# Hypothesis Testing

Hypothesis testing is a statistical method that is used in making statistical decisions using experimental data. It is basically an assumption that we make about the population parameter.

Key steps in hypothesis testing:

1. **State the hypotheses**. This involves stating the null and alternative, or research, hypothesis. The null hypothesis states that a population parameter (such as the mean, the standard deviation, and so on) is equal to a hypothesized value. The alternative hypothesis states that a population parameter is smaller, greater, or different than the hypothesized value.

2. **Formulate an analysis plan**. For this analysis, the significance level is defined. The significance level is the probability of rejecting the null hypothesis if it is true. The significance level is often denoted by $\alpha$.

3. **Analyze sample data**. Using the analysis plan, the test statistic is calculated. Often, the formula for a test statistic will depend on the sample size, the hypothesis being tested, and the standard deviation.

4. **Interpret the results**. If the test statistic is extreme enough (based on the $\alpha$ level), reject the null hypothesis in favor of the alternative hypothesis.

The test statistic will follow a distribution based on the nature of the data and the hypothesis being tested. For example, if we are testing a hypothesis about a mean and we know the population standard deviation, we would use a Z-test and the test statistic would follow a standard normal distribution. If we do not know the population standard deviation, we would use a T-test and the test statistic would follow a t-distribution.

The formula for a Z-test is:

$$
Z = \frac{\bar{X} - \mu_0}{\sigma / \sqrt{n}}
$$

Where:
- $\bar{X}$ is the sample mean
- $\mu_0$ is the hypothesized population mean
- $\sigma$ is the population standard deviation
- $n$ is the sample size

The formula for a T-test is similar, but we use the sample standard deviation ($s$) in place of the population standard deviation:

$$
T = \frac{\bar{X} - \mu_0}{s / \sqrt{n}}
$$

The decision to reject the null hypothesis is made by comparing the test statistic to a critical value, which is determined by the significance level ($\alpha$) and the nature of the test (one-tailed or two-tailed).

> For more information, you may want to read about [Hypothesis Testing](https://www.google.com/search?q=Hypothesis+Testing) and [Z-Test vs T-Test](https://www.google.com/search?q=Z-Test+vs+T-Test).