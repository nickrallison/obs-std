---
bad_links: 
aliases: []
tags: [probability]
title: Geometric Distribution
date created: Friday, July 14th 2023, 10:32:41 am
---
# Geometric Distribution

The Geometric Distribution is a type of probability distribution that describes the number of trials needed to get the first success in repeated, independent Bernoulli trials. Each trial is independent of each other and has only two possible outcomes: success or failure. The probability of success ($p$) remains constant from trial to trial.

The Geometric Distribution is used to model scenarios where we are interested in the number of failures before the first success. For example, it can be used to model situations such as the number of times you need to flip a coin before it lands on heads, or the number of calls a salesperson needs to make before closing a sale.

The probability density function for a geometric distribution is given by $P(X = x) = (1-p)^{x-1} * p$, where $X$ represents the number of trials until the first success, $p$ is the probability of success on any given trial, and $x$ is any positive integer.

Its important to note that while geometric distribution deals with discrete outcomes (success or failure), its different from binomial distribution which counts the number of successes in a fixed number of trials. Instead, geometric distribution focuses on how many trials it takes until you achieve your first success.
