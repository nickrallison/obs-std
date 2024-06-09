---
aliases:
tags:
  - probability
bad_links:
---
# Beta Distribution

The Beta distribution is a [[Continuous Distribution.md|continuous probability distribution]] that is a part of the family of Beta functions. It is defined on the interval \([0, 1]\) and parameterized by two positive parameters, denoted as \( \alpha \) (alpha) and \( \beta \) (beta), which shape the distribution. The Beta distribution is widely used in probability theory and statistics, especially in Bayesian statistics, where it is often used to model the distribution of probabilities.

## Mathematical Definition

The probability density function (pdf) of the for a [[Random Variable.md|random variable ]]variable\(X \) in the interval\( [0, 1] \) is given by:

$$
f(x; \alpha, \beta) = \frac{x^{\alpha - 1} (1 - x)^{\beta - 1}}{B(\alpha, \beta)}
$$

where \( B(\alpha, \beta) \) is the Beta function, defined as:

$$
B(\alpha, \beta) = \int_0^1 t^{\alpha - 1} (1 - t)^{\beta - 1} dt
$$

This function ensures that the area under the [[Probability Density Function|pdf]] over the interval \( [0, 1] \) is equal to 1, satisfying the property of a probability density function.

## Parameters and Shapes

The shape of the Beta distribution is determined by the parameters \( \alpha \) and \( \beta \):

- When \( \alpha = \beta = 1 \), the Beta distribution is uniform across the interval \([0, 1]\).
- When \( \alpha > 1 \) and \( \beta > 1 \), the distribution is bell-shaped, with a mode (peak) within the interval \((0, 1)\).
- When \( \alpha < 1 \) and \( \beta < 1 \), the distribution is U-shaped, with higher densities near 0 and 1.
- When one parameter is larger than the other, the distribution skews towards the side of the interval associated with the smaller parameter.

## Moments

The expected value [[Expected Value.md|(mean)]] of a Beta-distributed random are given by:

- **Mean**: $\mu = \frac{\alpha}{\alpha + \beta}$
- **Variance**: $\sigma^2 = \frac{\alpha \beta}{(\alpha + \beta)^2 (\alpha + \beta + 1)}$

This equation helps quantify the spread or dispersion of the distribution around its mean.

## Applications

The Beta distribution is particularly useful in scenarios where probabilities are modelled on a fixed interval from 0 to 1. Common applications include:

- **Bayesian Inference**: In Bayesian probability, the Beta distribution is often used as a prior distribution for binomial proportions (such as the probability of success in a series of Bernoulli trials). This makes it extremely useful in updating beliefs about the parameter as evidence accumulates.
- **Quality Control**: It is used in quality control processes to model the variability of the percentage of defectives in a production process.
- **Project Management**: Project managers use the Beta distribution to model the completion times of tasks within a project, especially when only estimates of the minimum, most likely, and maximum times are known.
- **Genetics**: In genetics, the Beta distribution can model the distribution of gene frequencies.

## Properties

The Beta distribution has a number of interesting properties:

- It is conjugate to the binomial and Bernoulli distributions in Bayesian inference, simplifying the analysis and calculation of the posterior distribution.
- It is closed under order statistics, meaning that the minimum and maximum of a sample of Beta-distributed variables are also Beta-distributed, albeit with different parameters.
- Its flexibility in shape makes it suitable for modelling various kinds of data distributions, from very skewed to symmetric.

## Conclusion

Understanding the Beta distribution and its parameters \( \alpha \) and \( \beta \) allows for a deep insight into the behavior of proportions and probabilities bounded within [0, 1]. Its role in Bayesian statistics as a prior distribution makes it critical in the field of machine learning, where probabilistic models often underpin sophisticated predictive algorithms. The overview provided outlines key aspects from its mathematical foundation to applied use-cases, equipping practitioners to utilize the distribution effectively in their respective fields.