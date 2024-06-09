---
bad_links:
aliases: []
tags: [probability]
title: Independence
date created: Friday, July 14th 2023, 10:06:18 am
---
# Independence (Statistics)

Independence is a key concept in probability theory and statistics. It refers to the situation where the occurrence of one event does not affect the probability of the occurrence of another event. For example, if you flip a coin, the outcome of one flip does not impact the outcome of the next flip. Each flip is statistically independent from each other.

In more technical terms, two events A and B are said to be statistically independent if the probability of their [[Intersection|intersection]] equals the product of their probabilities: $P(A \cap B) = P(A)P(B)$. If this condition is not met, then A and B are dependent.

Statistical independence is a crucial assumption in many statistical tests and models. For instance, most parametric tests assume that observations are independently and identically distributed (i.i.d). Violations of this assumption can lead to misleading results.

In summary, independence is a fundamental concept in statistics that describes a situation where an events occurrence doesnt influence another events likelihood.

Another formula for independence is:
$$
\forall B \Bigl(\text{P}(A | B) = \text{P}(A)\Bigl)
$$