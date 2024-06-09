---
bad_links:
aliases: []
tags: [coding]
---
# Dynamic Programming

Dynamic programming is a method used in computer science and mathematics for solving complex problems by breaking them down into simpler subproblems. It is a kind of optimization approach that simplifies a complex problem by breaking it down into simpler steps in a recursive manner. The key idea is to save the solutions of these subproblems so that when the same subproblem arises, the solution can be reused instead of being recalculated. This approach is particularly useful for problems where the same subproblems are solved repeatedly. Dynamic programming is widely used in various fields such as operations research, bioinformatics, computer graphics, artificial intelligence, and machine learning.

Problems like trying to find the $n^{th}$ Fibonacci number with often have a lot of repetition when done with dynamic programming. The programmatic algorithm (As opposed to Binet's Formula) to find the $n^{th}$ fibonacci number will have a Big O time complexity of $\Theta(n^\phi)$ where other simplifications like LRU Caching or a simpler solution like Memoization

Dynamic Programming (DP) is a powerful algorithmic paradigm that solves optimization problems by breaking them down into simpler subproblems and storing the solutions to these subproblems to avoid redundant computation. It is primarily used when the problem exhibits the properties of overlapping subproblems and optimal substructure.

1. **Overlapping Subproblems**: This property is present when a recursive algorithm would visit the same subproblems repeatedly. DP solves each subproblem only once and then stores this answer in a table, thereby avoiding the work of recomputing the solution every time.

2. **Optimal Substructure**: A problem has optimal substructure if an optimal solution can be constructed efficiently from optimal solutions of its subproblems.

The general strategy for a DP problem is to solve the problem in a bottom-up fashion, where we first compute solutions to smaller subproblems and then use these solutions to construct solutions to larger subproblems.

The process of Dynamic Programming can be broken down into four steps:

1. **Characterize the structure of an optimal solution.**
2. **Define the value of an optimal solution recursively in terms of smaller subproblems.**
3. **Compute the value of an optimal solution in a bottom-up fashion.**
4. **Construct an optimal solution to the problem from the computed information.**

Let's consider the classic problem of computing the nth Fibonacci number as an example. The Fibonacci sequence is defined as: $F(n) = F(n-1) + F(n-2)$ for $n > 1$, and $F(0) = 0$, $F(1) = 1$.

Without DP, the naive recursive solution would involve a lot of repeated computation (exponential time complexity). With DP, we can store the solutions of the subproblems in an array and use them later, which reduces the time complexity to linear.

Here is a Python code snippet that demonstrates this:

```python
def fibonacci(n):
    # Step 1: Initialize the DP table
    dp = [0, 1] + [0]*(n-1)

    # Step 2: Fill the DP table in a bottom-up manner
    for i in range(2, n+1):
        dp[i] = dp[i-1] + dp[i-2]

    # Step 3: The nth Fibonacci number is the last element in the DP table
    return dp[n]
```

> For further reading, you may want to explore the following resources:
> - [Dynamic Programming | Set 1 (Overlapping Subproblems Property)](https://www.google.com/search?q=Dynamic+Programming+%7C+Set+1+(Overlapping+Subproblems+Property))
> - [Dynamic Programming | Set 2 (Optimal Substructure Property)](https://www.google.com/search?q=Dynamic+Programming+%7C+Set+2+(Optimal+Substructure+Property))
> - [Introduction to Dynamic Programming 1](https://www.google.com/search?q=Introduction+to+Dynamic+Programming+1)
> - [Dynamic Programming – From Novice to Advanced](https://www.google.com/search?q=Dynamic+Programming+%E2%80%93+From+Novice+to+Advanced)