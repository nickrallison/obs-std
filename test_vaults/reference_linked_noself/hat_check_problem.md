---
bad_links:
aliases: []
tags: [computerscience, probability]
---
# Hat Check Problem

The Hat Check Problem is a classic problem in probability theory. It involves n people who each have a hat. They throw their hats into a box and then each person randomly picks a hat from the box. The question is: what is the expected number of people who end up with their own hat?

Here's a step-by-step breakdown of the solution:

1. **Define the Random Variables:** Let X be the [[Random Variable.md|random variable]] representing the number of people who get their own hat back. Also, define an indicator random variable $X_i$ for each which equals 1 if person i gets their own hat back and 0 otherwise. Then, we can express X as the sum of all $X_i$'s.

2. **Calculate the [[Expected Value|average]] of $X_i$:** Since each person has an equal chance of picking any hat, the probability that person i gets their own hat back is 1/n. Therefore, the [[Expected Value.md|expected value]] of $X_i$ is $E[X_i] = 1/n$.
3. **Calculate the [[Expected Value|average]] of X:** By linearity of expectation, the expected value of the sum of x_i is the sum of ]expected values of all $X_i$'s. Since there are n people and each $E[X_i] = 1/n$, we have $E[X] = n * (1/n) = 1$.

4. **Calculate the [[Variance]]:** To calculate the [[Variance|variance]] of X, we need to calculate $E[X^2]$ and then use the formula $Var(X) = E[X^2] - (E[X])^2$. To find $E[X^2]$, we need to consider both terms where i equals j ($X_i^2$) and terms where i does not equal j ($X_i * X_j$). For terms where i equals j, we have already found that $E[X_i^2] = 1/n$. For terms where i does not equal j, we need to calculate the probability that both person i and person j get their own hats back. This probability is $(1/n) * (1/(n-1))$ because person i has a 1/n chance of getting their own hat and, given that person i got their own hat, person j has a 1/(n-1) chance of getting their own hat. Therefore, $E[X_i * X_j] = (1/n) * (1/(n-1))$.

5. **Calculate $E[X^2]$:** We have n terms where i equals j and $n*(n-1)$ terms where i does not equal j. Therefore, $E[X^2] = n*(1/n) + n*(n-1)*(1/n)*(1/(n-1)) = 1 + 1 = 2$.

6. **Calculate the [[Variance]] of X:** Finally, we can calculate the [[Variance|variance]] of X using the formula $Var(X) = E[X^2] - (E[X])^2 = 2 - 1^2 = 1$.

So, the expected number of people who get their own hat back is 1 and the [[Variance|variance]] is also 1.

## Sources
<https://www.youtube.com/watch?v=Kycmb2IwV-Y>

