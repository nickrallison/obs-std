---
bad_links:
aliases:
  - average
  - averages
  - expected values
tags:
  - probability
---
# Expected Value

Despite being a simple concept at its core, the calculation of expected value can significantly influence economic and strategic decisions, especially when outcomes involve variability or uncertain results.

## Mathematical Definition

The expected value of a random variable $X$, denoted as $E(X)$, is given by the following equation:
$$
E(X) = \sum_{x} x \cdot P(X = x)
$$
where $x$ represents a possible outcome and $P(X = x)$ is the probability of $X$ taking the value $x$. This formula applies to discrete cases. For [[continuity.md|continuous]] variables, the expected value is calculated as an integral over all possible outcomes.

## Example in Probability

Consider a simple dice game where a player wins an amount in dollars equal to the number rolled on a six-sided die. Let $X$ represent the outcome of the die roll. The expected value of the player's winnings can be calculated as follows:

$$
E(X) = 1 \cdot \frac{1}{6} + 2 \cdot \frac{1}{6} + 3 \cdot \frac{1}{6} + 4 \cdot \frac{1}{6} + 5 \cdot \frac{1}{6} + 6 \cdot \frac{1}{6} = \frac{21}{6} = 3.5
$$

This means, on average, the player can expect to win $3.5 per roll over a large number of rolls.

## Application in Decision Making

In decision-making processes, the expected value can help determine the best course of action when faced with different choices and uncertain outcomes. For instance, if a business project has a 50% chance of earning $200,000 and a 50% chance of earning nothing, the expected value of the project is:
$$
E(X) = 0 \cdot 0.5 + 200,000 \cdot 0.5 = $100,000
$$
This suggests that, on average, the project is expected to generate $100,000, helping decision-makers evaluate whether the risk is worth the potential gain.

## Limitations of Expected Value

While the expected relevance and utility of expected value are immense, it has limitations. It does not reflect the variability or risk involved in any given situation. For example, two different investments might have the same expected value but very different levels of risk. Additional measures like the [[variance.md|variance]] and standard deviation are often used alongside the expected property to assess the variability of outcomes.

In risk management, for instance, understanding both the expected losses and the probability distribution of these losses is crucial. This is why other concepts such as Expected Shortfall (ES) or Value at Risk (VaR) are commonly used in financial sectors to complement the insights provided by expected values.

Overall, the expected value serves as a fundamental building block in the fields of statistics, economics, finance, insurance, and many other domains where probabilistic framework models are applicable. Its simplicity in concept but powerful implications continue to make it a pivotal element in quantitative reasoning and analytical decision-making processes.