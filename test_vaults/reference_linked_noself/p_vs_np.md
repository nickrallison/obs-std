---
bad_links: 
aliases: [p=np, p = np]
tags: [theoreticalcompsci]
---
# P Vs [[Nondeterministic Polynomial Time|NP]]

The P vs [[Nondeterministic Polynomial Time|NP]] problem is one of the most fundamental unsolved problems in computer science. It deals with the question of computational complexity, specifically the relationship between problems that can be solved quickly (in "polynomial time") and problems for which a solution can be checked quickly.

**P** ([[Polynomial Time|Polynomial time]]) is a class of problems for which an algorithm can find a solution in [[Polynomial Time|polynomial time]]. The [[Big-O Notation|time complexity]] of these problems is represented as $O(n^k)$, where $n$ is the size of the input and $k$ is a constant. Examples of P problems include sorting a list of numbers or finding the shortest path in a graph.

**[[Nondeterministic Polynomial Time|NP]]** ([[Nondeterministic Polynomial Time|Nondeterministic Polynomial time]]) is a class of problems for which a solution can be verified in [[Polynomial Time|polynomial time]]. In other words, if you're given a 'certificate' (a possible solution), you can check whether it's a correct solution in [[Polynomial Time|polynomial time]]. An example of an [[Nondeterministic Polynomial Time|NP]] problem is the traveling salesperson problem: given a list of cities and the distances between each pair of cities, what is the shortest possible route that visits each city exactly once and returns to the origin city?

The P vs [[Nondeterministic Polynomial Time|NP]] question asks whether every problem for which a solution can be checked quickly ([[Nondeterministic Polynomial Time|NP]]) can also be solved quickly (P). In other words, is P equal to [[Nondeterministic Polynomial Time|NP]] (P=NP), or are there problems that can be checked quickly but not solved quickly (P≠NP)?

The P vs [[Nondeterministic Polynomial Time|NP]] problem is not just an academic question. It has profound implications for fields such as cryptography, operations research, database theory, AI, and more. For example, most modern cryptographic systems rely on the assumption that P≠NP, i.e., certain problems (like factoring large numbers) are hard to solve but easy to check.

Despite the efforts of many brilliant minds, the P vs [[Nondeterministic Polynomial Time|NP]] problem remains unsolved. It is one of the seven "Millennium Prize Problems" for which the Clay Mathematics Institute offers a $1 million prize for a correct solution.

> For more in-depth reading, you might find the following resources helpful:
> - ["P vs NP Problem"](https://www.google.com/search?q=P+vs+NP+Problem) on Google Search
> - ["P vs NP Problem"](https://en.wikipedia.org/wiki/P_versus_NP_problem) on Wikipedia
> - ["The Status of the P versus NP Problem"](https://www.google.com/search?q=The+Status+of+the+P+versus+NP+Problem) by Lance Fortnow
> - ["The P vs NP Page"](https://www.google.com/search?q=The+P+vs+NP+Page) by Scott Aaronson
> - ["Computational Complexity: A Modern Approach"](https://www.google.com/search?q=Computational+Complexity%3A+A+Modern+Approach) by Sanjeev Arora and Boaz Barak
