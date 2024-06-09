---
bad_links: 
aliases: []
tags: [graphtheory, algorithms]
---
# Held-Karp Algorithm

The Held-Karp algorithm is a dynamic programming algorithm used to solve the Traveling Salesman Problem (TSP). The TSP is a classic algorithmic problem in the field of computer science and operations research, focusing on optimization. In this problem, a salesman is given a list of cities and must determine the shortest possible route that allows him to visit each city once and return to his original location.

The Held-Karp algorithm, named after Michael Held and Richard M. Karp who proposed it in 1962, is an improvement over the brute force approach which checks all possible permutations of the cities. The brute force approach has a time complexity of $O(n!)$, which is impractical for large numbers of cities. The Held-Karp algorithm, on the other hand, has a time complexity of $O(n^2 2^n)$, which is significantly more efficient for larger inputs.

The algorithm uses the principles of dynamic programming to break the problem down into smaller subproblems, and then combines the solutions to these subproblems to find the overall solution. The key idea is to use a memoization table to store the results of subproblems so that each subproblem is only solved once.

The algorithm can be described as follows:

1. **Initialization**: Create a memoization table, `C`, with dimensions `(n, 2^n)`, where `n` is the number of cities. Each entry `C[s, i]` represents the minimum cost of a tour that starts at city `i`, visits all cities in set `s` exactly once, and returns to the starting city.

2. **Base Case**: For each city `i`, set `C[{i}, i] = d(1, i)`, where `d(i, j)` is the distance between cities `i` and `j`. This represents the cost of a tour that only visits city `i` and then returns to the starting city.

3. **Recursive Case**: For each set `s` of size `m > 1`, and for each city `i` in `s`, set `C[s, i] = min_{j in s, j != i} {C[s - {i}, j] + d(j, i)}`. This represents the cost of the best tour that visits all cities in `s`, ends at city `i`, and does not include the edge from `i` to the starting city.

4. **Final Solution**: The minimum cost of a tour that visits all cities exactly once and returns to the starting city is given by `min_{i > 1} {C[{1, 2, …, n}, i] + d(i, 1)}`.

The Held-Karp algorithm is a significant improvement over the brute force approach for solving the TSP, but it is still not efficient enough for very large instances of the problem due to its exponential time complexity. However, it serves as a valuable benchmark for comparing other TSP algorithms.

> For further reading, you may want to look into the following resources:
> - [Held-Karp Algorithm - Wikipedia](https://www.google.com/search?q=Held-Karp+algorithm)
> - [Traveling Salesman Problem - Wikipedia](https://www.google.com/search?q=Traveling+Salesman+Problem)
> - [Dynamic Programming - Wikipedia](https://www.google.com/search?q=Dynamic+Programming)
> - [Held, M., & Karp, R. M. (1962). A dynamic programming approach to sequencing problems. Journal of the Society for Industrial and Applied Mathematics, 10(1), 196-210.](https://scholar.google.com/scholar?hl=en&as_sdt=0%2C5&q=A+dynamic+programming+approach+to+sequencing+problems&btnG=)