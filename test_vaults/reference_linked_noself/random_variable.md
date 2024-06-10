---
aliases: [binary random variables, discrete random variable, continuous random variable]
tags: [probability]
title: Random Variable
date created: Friday, July 14th 2023, 10:27:10 am
bad_links: [Continuity.md]
---
# Random Variable

A **random variable** is a variable whose possible values are numerical outcomes of a random phenomenon. There are two types of random variables, discrete and [[continuity.md|continuous]].

A **discrete random variable** is one which may take on only a countable number of distinct values such as 0,1,2,3,4,… etc. Examples might be the number of heads you get when flipping three coins, or the number of defective light bulbs in a box of ten.

A **[[continuity.md|continuous]] random variable** is one which takes an infinite number of possible values. For example, you could define a random variable X to be the height of students in a class. Since height is a [[continuity.md|continuous]] measurement, X is a [[continuity.md|continuous]] random variable.

The **probability distribution** of a random variable is a function that describes the likelihood of each possible outcome. For discrete random variables, this is often represented as a [[Probability Mass Function|probability mass function]] ([[Probability Mass Function|PMF]]), while for [[continuity.md|continuous]] random variables, it is represented as a [[Probability Density Function|probability density function]] ([[Probability Density Function|PDF]]).

The **[[Expected Value|expected value]]** (or mean) of a random variable is the long-run [[Expected Value|average]] value of repetitions of the experiment it represents. For a discrete random variable, the [[Expected Value|expected value]] is calculated by summing the product of each outcome and its probability. For a [[continuity.md|continuous]] random variable, it's the integral of the product of each outcome and its probability density. The formula for the [[Expected Value|expected value]] of a discrete random variable $X$ is:

$$
E[X] = \sum_{i=1}^{n} x_i P(X = x_i)
$$

where $x_i$ are the possible outcomes and $P(X = x_i)$ is the probability of each outcome.

The **[[Variance|variance]]** of a random variable is a measure of how much values differ from the [[Expected Value|expected value]]. The formula for the [[Variance|variance]] of a random variable $X$ is:

$$
Var[X] = E[(X - E[X])^2] = E[X^2] - (E[X])^2
$$

The **[[Standard Deviation|standard deviation]]** is the square root of the [[Variance|variance]] and provides a measure of the [[Expected Value|average]] distance between the values of the random variable and the [[Expected Value|expected value]].

> For more detailed information, you can refer to the following resources:
> - [Random Variables - Khan Academy](https://www.google.com/search?q=Random+Variables+-+Khan+Academy)
> - [Probability Distributions - Stat Trek](https://www.google.com/search?q=Probability+Distributions+-+Stat+Trek)
> - [Expected Value and Variance - MIT OpenCourseWare](https://www.google.com/search?q=Expected+Value+and+Variance+-+MIT+OpenCourseWare)