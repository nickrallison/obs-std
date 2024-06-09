---
bad_links: 
aliases: []
tags: [probability]
---
# Variance

Variance is a statistical measurement that describes the spread of numbers in a data set. More specifically, it quantifies the degree to which each number in the list is from the mean ([[Expected Value|average]]) of the list, and from each other. 

The formula for variance ($\sigma^2$) in a population of size N is:

$$
\sigma^2 = \frac{1}{N}\sum_{i=1}^{N}(x_i - \mu)^2
$$

Where:
- $x_i$ represents each value from the data set
- $\mu$ is the mean of the data set
- $N$ is the number of data points

The formula for variance in a sample of size n is slightly different:

$$
s^2 = \frac{1}{n-1}\sum_{i=1}^{n}(x_i - \bar{x})^2
$$

Where:
- $x_i$ represents each value from the data set
- $\bar{x}$ is the mean of the sample
- $n$ is the number of data points in the sample

The difference between these two formulas is the denominator. In the population variance formula, we divide by the number of data points, N. In the sample variance formula, we divide by the number of data points minus 1, n-1. This is known as Bessel's correction, and it corrects the bias in the estimation of the population variance from a sample.

The square root of variance gives us the [[Standard Deviation|standard deviation]], another important measure of spread. The [[Standard Deviation|standard deviation]] is more commonly used because it is in the same units as the data, while variance is in squared units.

Variance is used in many areas of statistics and probability theory. For example, it is used in the calculation of the [[Confidence Interval|confidence interval]], [[Hypothesis Testing|hypothesis testing]], ANOVA, regression analysis, and many other statistical tests and models.

> For more information, you can refer to the following resources:
> - [Variance - Wikipedia](https://www.google.com/search?q=Variance+Wikipedia)
> - [Standard Deviation and Variance - Math Is Fun](https://www.google.com/search?q=Standard+Deviation+and+Variance+Math+Is+Fun)
> - [Bessel's Correction - Wikipedia](https://www.google.com/search?q=Bessel%27s+Correction+Wikipedia)