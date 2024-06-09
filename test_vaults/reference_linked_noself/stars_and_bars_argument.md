---
bad_links: 
aliases: ["Stars and Bars"]
tags: [probability]
---
# Stars and Bars Argument

The Stars and Bars Argument, also known as the balls and urns model, is a combinatorial method used to solve problems of distributing identical items (stars) into distinct groups (bars). It's a powerful tool in combinatorics, the branch of mathematics concerned with counting, arrangement, and combination.

The basic formula for the Stars and Bars Argument is given by the binomial coefficient:

$$
{n + k - 1 \choose k - 1}
$$

where $n$ is the number of identical items (stars) and $k$ is the number of distinct groups (bars). This formula represents the number of ways to distribute $n$ identical items into $k$ distinct groups.

The derivation of this formula is based on the concept of a [[Bijective|bijection]], or a one-to-one correspondence, between the problem of distributing stars among bars and the problem of choosing $k - 1$ elements from a set of $n + k - 1$ elements.

Consider a sequence of $n$ stars and $k - 1$ bars. Any arrangement of these $n + k - 1$ symbols corresponds to a unique way of distributing the $n$ stars into $k$ groups. For example, if $n = 7$ and $k = 3$, an arrangement like $\star \star | \star \star \star | \star \star$ corresponds to distributing 7 stars into 3 groups with 2, 3, and 2 stars respectively.

The number of such arrangements is given by the binomial coefficient ${n + k - 1 \choose k - 1}$, which is the number of ways to choose $k - 1$ places for the bars among the $n + k - 1$ total places.

The Stars and Bars Argument can be extended to solve more complex problems, such as distributing items with restrictions on the groups, or distributing distinct items among groups.

> For further reading, you may want to explore the following resources:
> - [Stars and Bars (Combinatorics) - Wikipedia](https://www.google.com/search?q=Stars+and+Bars+(Combinatorics)+Wikipedia)
> - [Combinatorial Proofs - Art of Problem Solving](https://www.google.com/search?q=Combinatorial+Proofs+-+Art+of+Problem+Solving)
> - [Binomial Coefficients - Wolfram MathWorld](https://www.google.com/search?q=Binomial+Coefficients+-+Wolfram+MathWorld)