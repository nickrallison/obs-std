---
bad_links: 
aliases: []
tags: [machinelearning, probability]
---
# Multi-Armed Bandit

The Multi-Armed Bandit (MAB) problem is a classic reinforcement learning problem that models a decision-maker's choice problem. The name comes from a hypothetical scenario involving a gambler at a row of slot machines (also known as "one-armed bandits"), who must decide which machines to play, how many times to play each machine and in which order to play them, to maximize his reward.

In a more formal definition, a MAB problem is a tuple $(A, R)$ where $A$ is a finite set of actions (the arms of the bandit) and $R = \{R_a: a \in A\}$ is a set of unknown probability distributions, one for each arm. When action $a$ is selected, a reward $r \sim R_a$ is received. The objective is to find a policy (a mapping from historical actions and rewards to actions) that maximizes the expected total reward over some time horizon.

The challenge in the MAB problem lies in the trade-off between "exploration" and "exploitation". To obtain a lot of reward, a learning algorithm must exploit the arms that it has tried out and found to be effective. However, to discover such arms, it has to explore the arms that it has not selected much, which could result in less reward.

There are several strategies to solve the MAB problem, including but not limited to:

1. ****: This strategy plays the best arm a majority of the time but explores the other arms with a small probability $\epsilon$.

2. ** ([[Upper Confidence Bound.md|UCB]])**: This strategy selects the arm that has the highest upper confidence bound on the expected reward. The UCB for arm $a$ at time $t$ is given by:

$$
UCB_a(t) = \bar{X}_a(t) + \sqrt{\frac{2 \ln t}{n_a(t)}}
$$

where $\bar{X}_a(t)$ is the average reward received from arm $a$ up to time $t$, $n_a(t)$ is the number of times arm $a$ has been played up to time $t$, and the square root term represents the uncertainty or variance in the estimate of arm $a$'s reward.

1. **Thompson Sampling**: This is a probabilistic strategy that chooses the arm with the highest expected reward with respect to a randomly drawn belief.

For more in-depth understanding, you may want to explore the following resources:

> - [Multi-Armed Bandit Problem and Its Solutions](https://www.google.com/search?q=Multi-Armed+Bandit+Problem+and+Its+Solutions)
> - [A Tutorial on Thompson Sampling](https://www.google.com/search?q=A+Tutorial+on+Thompson+Sampling)
> - [The Multi-Armed Bandit Problem and Its Solutions: A Survey](https://www.google.com/search?q=The+Multi-Armed+Bandit+Problem+and+Its+Solutions%3A+A+Survey)

Tangentially, the MAB problem has found applications in various fields such as economics, computer science, and clinical trials. It is also a stepping stone to more complex reinforcement learning problems where the state of the environment changes over time.

## [[Reinforcement Learning - An Introduction.pdf]] - Pages 47-68 Summary

The document's second chapter delves into the concept of multi-armed bandits within the field of reinforcement learning. The key premise is that reinforcement learning bases its training information on the evaluation of enacted actions rather than on the provision of correct ones. This context prompts a need for active exploration in the quest for optimal behavior. These issues come sharply into focus through an examination of the k-armed bandit problem - a nonassociative setting where learners must balance evaluation of the potential reward of various actions to optimize results. Essential learning methodologies for this problem are explored, and the chapter emphasizes the balance of exploration and exploitation as a keystone of effective reinforcement learning.

Several strategies for action selection are discussed, including the greedy and epsilon-greedy method. These methods favor immediate reward and less time spent in sampling inferior actions while allowing occasional random actions to avoid stagnation. While the efficacy of these methods can fluctuate, the $\epsilon$-greedy methods showed superior long-term performance in a 10-armed testbed compared to greedy methods. Furthermore, to reduce memory and computation requirements, the document advocates the use of incremental formulas for updating averages, even introducing a novel update rule.

Several theories geared towards fostering exploration in a 10-armed bandit problem context are considered. Among these are the utilization of optimistic initial values and the Upper-Confidence-Bound (UCB) action selection method. Additionally, the gradient bandit algorithms, which employ numerical preferences rather than action-value estimates, are presented. These techniques offer various solutions to handle the exploration-exploitation dilemma inherent in the bandit problem.

The exploration of the gradient bandit algorithm affirms its use in multi-armed bandit problems, showing the algorithm as an embodiment of stochastic gradient ascent with robust convergence properties. The document also explores associative search tasks, which involve the matching of actions with situations.

To balance exploration and exploitation in multi-armed bandit problems, different methods such as $\epsilon$-greedy, UCB methods, and gradient bandit algorithms are suggested. A small-scale study placed UCB as the best performer. However, despite their effectiveness, these methods do not provide a complete solution to the exploration-exploitation problem.

The chapter concludes by acknowledging the work of several researchers, including Thathachar and Sastry, Sutton, Lai and Robins, Kaelbling, and Agrawal; all of whom significantly contributed to the development and understanding of bandit problems. They addressed these challenges from myriad perspectives, including psychology, engineering, and statistics. The study concludes by suggesting that the better understanding of sample complexity for exploration efficiency and the potential of approximate reinforcement learning methods may provide improvements to currently applied solutions.