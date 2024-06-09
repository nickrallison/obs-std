---
bad_links: 
aliases: []
tags: [algorithms]
---
# Fractional Knapsack Problem

The Fractional Knapsack Problem is a problem in combinatorial optimization. This problem can be solved by greedy algorithms, which make the locally optimal choice at each step in the hope that these local choices will lead to a global optimum.

The problem can be described as follows: Given a set of items, each with a weight and a value, determine the number of each item to include in a collection so that the total weight is less than or equal to a given limit and the total value is as large as possible. Unlike the 0/1 Knapsack problem, you are allowed to break items for maximizing the total value of knapsack.

The greedy strategy for the fractional knapsack problem is to calculate the ratio (value/weight) for each item and sort the item on basis of this ratio. Then take the item with the highest ratio and add them until we can’t add the next item as a whole and at the end add the next item as much as we can. Which will always be the optimal solution to this problem.

A formal mathematical representation of the problem is as follows:

Given:
- $n$ items, each with a weight $w_i$ and a value $v_i$ for $i = 1, 2, …, n$.
- A maximum weight capacity $W$.

Find:
- A vector $x = (x_1, x_2, …, x_n)$ where $0 \leq x_i \leq 1$ represents the fraction of item $i$ to include in the knapsack, such that the total weight $\sum_{i=1}^{n} x_i w_i \leq W$ and the total value $\sum_{i=1}^{n} x_i v_i$ is maximized.

The greedy algorithm to solve this problem is as follows:

1. Compute the value/weight ratio $r_i = v_i / w_i$ for each item.
2. Sort the items in decreasing order of $r_i$.
3. Initialize the knapsack as empty: $x_i = 0$ for all $i$.
4. For each item in the sorted list (from highest to lowest $r_i$):
   - If the item can be added to the knapsack without exceeding the weight limit, add it: $x_i = 1$.
   - If the item cannot be added without exceeding the weight limit, add as much of it as possible: $x_i = (W - \sum_{j=1}^{i-1} x_j w_j) / w_i$.
   - Update the remaining weight limit: $W = W - x_i w_i$.

This algorithm assumes that the items can be divided into fractions, which is why it's called the "fractional" knapsack problem. The [[Big-O Notation|time complexity]] of this algorithm is $O(n \log n)$ due to the sorting step.

> For more information, you can refer to the following resources:
> - [Fractional Knapsack Problem](https://www.google.com/search?q=Fractional+Knapsack+Problem)
> - [Greedy Algorithms](https://www.google.com/search?q=Greedy+Algorithms)
> - [Combinatorial Optimization](https://www.google.com/search?q=Combinatorial+Optimization)