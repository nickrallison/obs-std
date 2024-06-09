---
bad_links: 
aliases: [UCB]
tags: [machinelearning, probability]
---
# Upper Confidence Bound

The Upper Confidence Bound (UCB) is a term used in statistics to refer to the upper limit of a confidence interval. A confidence interval is a range of values, derived from a statistical procedure, that is likely to contain the true value of an unknown parameter. The UCB is often used in decision-making processes where uncertainty exists, such as in the Multi-Armed Bandit problem in reinforcement learning.

The UCB algorithm is a strategy to solve the exploration-exploitation dilemma in the multi-armed bandit problem. The idea is to choose the action that has the maximum upper confidence bound, as it represents the action with the potential for being the most rewarding.

The UCB for each action is calculated using the formula:

$$
UCB = \bar{X} + \sqrt{\frac{2 \ln n}{N}}
$$

where:
- $\bar{X}$ is the average reward obtained from the action so far,
- $n$ is the total number of times any action has been selected, and
- $N$ is the number of times the current action has been selected.

The term $\sqrt{\frac{2 \ln n}{N}}$ represents the uncertainty or variance in the estimate of the action's reward. As $N$ increases (i.e., the action is selected more often), the uncertainty decreases and the UCB gets closer to the average reward $\bar{X}$.

The derivation of the UCB formula is based on the Hoeffding's inequality, which provides an upper bound on the probability that the sum of bounded independent random variables deviates from its expected value by more than a certain amount.

For more in-depth understanding, you may want to look into the following resources:

> - [Understanding the Upper Confidence Bound](https://www.google.com/search?q=Understanding+the+Upper+Confidence+Bound)
> - [Hoeffding's Inequality](https://www.google.com/search?q=Hoeffding%27s+Inequality)
> - [Multi-Armed Bandit Problem](https://www.google.com/search?q=Multi-Armed+Bandit+Problem)
> - [Exploration-Exploitation Dilemma](https://www.google.com/search?q=Exploration-Exploitation+Dilemma)