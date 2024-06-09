---
bad_links:
aliases: []
tags: [probability]
title: Poisson Distribution
date created: Friday, July 14th 2023, 10:36:24 am
---
# Poisson Distribution

The Poisson distribution is a statistical distribution that shows how many times an event is likely to occur within a specified period of time. It is used to predict the probability of certain events from happening when you know how often the event has occurred. The Poisson distribution can also be used for the number of events in other specified intervals such as distance, area or volume.

For example, if the average number of calls in a call center per hour is 12, the Poisson distribution can be used to calculate the probability of having exactly 17 calls in a given hour.

The key conditions for using Poisson are that the events are independent (one event does not affect another), they occur at a constant rate, and they occur one at a time rather than in bunches.

Named after French mathematician Siméon Denis Poisson, this concept is widely used in fields like telecommunications, insurance, astronomy and finance.

The formula for the Poisson distribution is:

$$
P(k \text{ events in interval}) = \frac{\lambda^{k}e^{-\lambda}}{k!}
$$

Where:

- $P$ is the probability of $k$ events in an interval
- $\lambda$ is the expected number or rate
- $k$ is the actual number of successes
- $e$ is Euler's number (approximately equal to 2.71828)
- $k!$ is the factorial of $k$.