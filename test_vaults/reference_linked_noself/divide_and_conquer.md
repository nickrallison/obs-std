---
aliases:
  - recursive algorithms
  - divide-and-conquer
tags:
  - algorithms
bad_links:
  - Continuity.md
---
# Divide and Conquer

Divide and Conquer is a fundamental algorithmic technique for solving problems. It works by recursively breaking down a problem into two or more sub-problems of the same or related type, until these become simple enough to be solved directly. The solutions to the sub-problems are then combined to give a solution to the original problem.

The Divide and Conquer strategy consists of the following steps:

1. **Divide**: This involves dividing the problem into some sub problem.
2. **Conquer**: Sub problem by calling recursively until sub problem solved.
3. **Combine**: The Sub problem Solved so that we will get find problem solution.

The formal recursive definition is:

$$
T(n) = aT(\frac{n}{b}) + f(n)
$$

where:
- $n$ is the size of the problem.
- $a$ is the number of subproblems in the recursion.
- $n/b$ is the size of each subproblem. All subproblems are assumed to have the same size.
- $f(n)$ is the cost of the work done outside the recursive calls, which includes the cost of dividing the problem and the cost of merging the solutions.

This is known as the [[Master Theorem|Master Theorem]], which provides a method to solve recurrences of this form.

Examples of Divide and Conquer algorithms include Quick Sort, [[Merge Sort|Merge Sort]], [[Binary Search|Binary Search]], and [[Strassen’s Algorithm|Strassen’s Algorithm]].

For instance, consider the [[Merge Sort|Merge Sort]] algorithm. It divides the unsorted list into $n$ sublists, each containing one element (a list of one element is considered sorted), repeatedly merges sublists to produce new sorted sublists until there is only one sublist remaining.

The [[Big-O Notation|time complexity]] of [[Merge Sort|Merge Sort]] is $O(n\text{log}(n))$ in all 3 cases (worst, [[Expected Value|average]] and best) as [[Merge Sort|merge sort]] always divides the array into two halves and takes linear time to merge two halves.

## [[Introduction to Algorithms 4e.pdf]] Summary - Pages 98-147

Chapter 4 extensively explores the divide-and-conquer method in algorithm designs and analysis, covering different techniques for determining recurrence runtime and offering examples of solving those recurrences through various methods. The divide-and-conquer method is widely applicable due to its ability to break problems down into manageable subproblems. The systematic assessment of running time, explored in the same chapter, relies on recurrences, which describe a function based on its value on other arguments. This powerful approach allows for the analysis of a variety of scenarios such as [[Matrix Multiplication|matrix multiplication]], with the notable calculation of running times characterized by the recurrences T(n) = 8T(n/2) + Θ(1) and T(n) = 7T(n/2) + Θ(n^2), yielding solutions T(n) = Θ(n^3) and T(n) = Θ(n^(log7)) respectively.

Several methods of solving these recurrences are discussed, including the [[Substitution Method for Recurrences|substitution method]], the recursion-tree method, the [[Master Theorem|master method]], and the [[Akra-Bazzi Theorem|Akra-Bazzi method]]. A consistent focus is on the importance of clear base case definition and the choice of a sufficiently large threshold constant. A detailed exploration of partitioning, index calculation, and base and recursive cases in a divide-and-conquer strategy for square-matrix multiplication is illuminated using the MATRIX-MULTIPLY-RECURSIVE procedure as an example.

[[Strassen’s Algorithm|Strassen’s algorithm]], an advanced technique for reducing [[Matrix Multiplication|matrix multiplications]] for matrix calculations, is studied in depth, navigating through its steps and equational dependencies. This algorithm achieves lower asymptotic running times by eliminating one recursive [[Matrix Multiplication|matrix multiplication]]. The algorithm represented by the recurrence T(n) = 7T(n/2) + Θ(n^2) denoted as [[Strassen’s Algorithm|Strassen’s algorithm]], offers an efficient tradeoff of one [[Matrix Multiplication|matrix multiplication]] for a constant number of matrix additions.

Further explored is the process of utilizing different ways to multiply matrices of varying sizes via the divide-and-conquer method of algorithm development. Different techniques are prevalent in divide and conquer strategy, including the [[Substitution Method for Recurrences|substitution method]] with mid-study examples that help explain the best-case scenarios for applying specific methods. Utilization of the substitution technique enables the derivation of an asymptotic upper bound on a recurrence. It's also highlighted that finding the tightest asymptotic solution might require experience, creativity, and estimating both upper and lower bounds.

The [[Master Theorem|Master Theorem]] is provided as another effective approach for determining the asymptotic behavior of recurrences. Filled with a number of cases based on the kind of problem presented, this theorem offers a structured way to approach and solve recurrences of the form T(n) = aT(n/b) + f(n). Precise equations are provided to elaborate on how to apply the theorem. However, it is noted that the [[Master Theorem|Master Theorem]] cannot be applied to all scenarios and alternate methods such as the [[Substitution Method for Recurrences|substitution method]] or [[Akra-Bazzi Theorem|Akra-Bazzi method]] are recommended in those instances.

The [[Akra-Bazzi Theorem|Akra-Bazzi method]] is discussed, providing an approach to solving recurrences that may involve different-sized subproblems. The section advises that ignoring floors and ceilings in recurrences has limitations that can be bypassed through the polynomial-growth condition. The [[continuity.md|Continuous]] [[Master Theorem|Master Theorem]] is also discussed, explaining the various cases of algorithmic recurrences, their derivations, and solutions.

Finally, the divide-and-conquer method's practicality is exemplified in [[Matrix Multiplication|matrix multiplications]] and chip testing problem scenarios. Cases like these, where problem sizes range unevenly, further underline the need for different methods such as [[Akra-Bazzi Theorem|Akra-Bazzi's]]. In conclusion, Chapter 4 emphasizes the significance of carefully understanding and implementing the divide-and-conquer strategy, different methods for solving recurrences related to such algorithms, and the impact of various approaches on algorithm efficiency. The chapter enables a deeper understanding of when and how to distinctly employ these strategies, either individually or combined, to optimize an algorithm's runtime.

> For more in-depth reading, you may refer to the following resources:
> - [Divide and Conquer Introduction](https://www.google.com/search?q=Divide+and+Conquer+Introduction)
> - [Master Theorem](https://www.google.com/search?q=Master+Theorem)
> - [Merge Sort](https://www.google.com/search?q=Merge+Sort)
> - [Quick Sort](https://www.google.com/search?q=Quick+Sort)
> - [Binary Search](https://www.google.com/search?q=Binary+Search)
> - [Strassen’s Algorithm](https://www.google.com/search?q=Strassen%E2%80%99s+Algorithm)