---
bad_links: 
aliases: [T-Test]
tags: [probability]
---
# T Testing

T Testing, also known as Student's T Test, is a statistical hypothesis testing method used to determine if there is a significant difference between the means of two groups. It was developed by William Sealy Gosset under the pseudonym "Student".

The T Test is based on the T distribution, a probability distribution that is used to estimate population parameters when the sample size is small and/or when the population variance is unknown. The T distribution is similar to the normal distribution but has heavier tails, which makes it more sensitive to changes in the sample size.

There are three main types of T Tests:

1. **Independent Samples T Test**: This is used when comparing the means of two independent groups. For example, comparing the test scores of students taught by two different teachers.

2. **Paired Samples T Test**: This is used when comparing the means of the same group at two different times. For example, comparing the test scores of students before and after a teaching intervention.

3. **One Sample T Test**: This is used when comparing the mean of a single group against a known mean. For example, comparing the average height of a class of students to the national average height.

The formula for the T statistic in a One Sample T Test is:

$$
t = \frac{\bar{x} - \mu}{s/\sqrt{n}}
$$

Where:
- $\bar{x}$ is the sample mean
- $\mu$ is the population mean
- $s$ is the sample standard deviation
- $n$ is the sample size

The formula for the T statistic in an Independent Samples T Test is slightly more complex due to the need to account for the variances and sample sizes of both groups:

$$
t = \frac{\bar{x}_1 - \bar{x}_2}{\sqrt{s^2_1/n_1 + s^2_2/n_2}}
$$

Where:
- $\bar{x}_1$ and $\bar{x}_2$ are the sample means
- $s^2_1$ and $s^2_2$ are the sample variances
- $n_1$ and $n_2$ are the sample sizes

The T statistic follows a T distribution with degrees of freedom determined by the sample size(s). The critical value of T for a given significance level (commonly 0.05) can be found from a T distribution table or calculated using statistical software. If the absolute value of the T statistic is greater than the critical value, the null hypothesis (that the population means are equal) is rejected.

The T Test assumes that the data are normally distributed and that the variances of the two populations are equal (homoscedasticity). If these assumptions are violated, alternative tests such as the Mann-Whitney U Test or the Welch's T Test may be more appropriate.

> For more information, you may want to read the following resources:
> - [T Test (Student's T-Test): Definition and Examples](https://www.statisticshowto.com/probability-and-statistics/t-test/)
> - [T-Distribution](https://www.statisticshowto.com/probability-and-statistics/t-distribution/)
> - [Assumptions of the T-Test](https://www.statisticssolutions.com/assumptions-of-the-t-test/)