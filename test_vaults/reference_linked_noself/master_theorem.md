---
bad_links: 
aliases: [master method]
date created: Monday, June 26th 2023, 3:32:29 pm
tags: [algorithms, proofs]
title: Master Theorem
---

# Master Theorem

The Master Theorem is a powerful tool in the field of computer science, specifically in the analysis of [[Divide and Conquer|recursive algorithms]]. It provides a method to determine the [[Big-O Notation|time complexity]] of [[Divide and Conquer|recursive algorithms]] in a straightforward manner.

The Master Theorem deals with recursive [[Binary Relation|relations]] of the following type:

$$
T(n) = aT\left(\frac{n}{b}\right) + f(n)
$$

where:
- $n$ is the size of the problem.
- $a \geq 1$ is the number of subproblems in the recursion.
- $b > 1$ is the factor by which the problem size is divided in each recursive call.
- $f(n)$ is the cost of the work done outside the recursive calls, which includes the cost of dividing the problem and the cost of merging the results.

The Master Theorem provides three cases which cover different types of functions $f(n)$:

1. **Case 1**: If $f(n) = O(n^c)$ where $c < \log_b{a}$, then $T(n) = \Theta(n^{\log_b{a}})$.
2. **Case 2**: If $f(n) = \Theta(n^c \log^k{n})$ where $c = \log_b{a}$ and $k \geq 0$, then $T(n) = \Theta(n^c \log^{k+1}{n})$.
3. **Case 3**: If $f(n) = \Omega(n^c)$ where $c > \log_b{a}$, ()if $a f\left(\frac{n}{b}\right) \leq kf(n)$ for some constant $k < 1$ and sufficiently large $n$, then $T(n) = \Theta(f(n))$.

The Proof of the Master Theorem is beyond the scope of this response due to its complexity and length. However, it relies on the concept of the recursion tree and the properties of geometric series. For a detailed proof, you can refer to the textbook "Introduction to Algorithms" by Cormen, Leiserson, Rivest, and Stein.

# Sources
## Algorithms Illuminated Part 1 The Basics Summary - Pages 105-129

Chapter 4 unveils the "Master Method," a profound tool for assessing the running time of [[Divide and Conquer|recursive algorithms]], predominantly [[Divide and Conquer|divide-and-conquer]] systems. The Master Method facilitates the computation of a maximum running time limit, keying in specific features of the algorithm. The algorithm’s running time of Recursive Integer Multiplication (RecIntMult) and Karatsuba's Integer Multiplication, among others, are constructively determined through this black-box method.

Mathematical analysis, recurrences, and formal statement of the master method pivotally start the chapter, emphasizing the evaluation of algorithmic ideas. This theoretical analysis is wrapped up with proof and a study of MergeSort from section 1.5. The consequence of three cases involved in the Master Method and its relevance is reinforced through the text. Besides, the algorithms of integer multiplication, their functioning, and running time bounds are expansively interpreted, drawing emphasis on note-worthy features like the base case of the recurrence.

Six real-life examples, including MergeSort and [[Binary Search|Binary Search]], illustrate the application of the Master Method, providing an enhanced understanding of the theorem. This theorem enables projection of the upper bound of a standard recurrence, delving into three scenarios based on the parameters of the recurrence. The proofs provided refer to the conceptual significance behind each case of the theorem, focusing on understanding rather than rote learning. The [[Difference Equations|recurrence relation]] employed in the Master Method is also discussed, followed by an introduction to the procedure of recursion trees’ analysis.

The comparison of running times of different algorithms is detailed, expressly underlining that MergeSort's running time prevails above Karatsuba and RecIntMult. The mastered method's application to standard recurrences requires an understanding of certain parameters like the number of recursive calls, input size, and more. This chapter highlights the importance of keeping these parameters as constants, independent of the input size. The analysis extends to forecasting running time bounds with the Master Method for three cases, navigated through work-performed-per-level in recursion trees.

Attempted proof for the Master Method and a detailed understanding of recursion trees are highlighted, developing a robust comprehension of related concepts. Quizzes and practice problems featured in the chapter support a practical understanding of the Master Method, its applications, and impacts.

To conclude, the Master Method is a foundational theorem in assessing the running time of [[Divide and Conquer|recursive algorithms]]. The three cases dictated by the parameters of a standard recurrence can indicate the solution to the time-bound problem. Chapter 4 illustrates the Master Method with a proof, examples, and extensive comparisons, significantly enriching understanding of the theorem and allied algorithmic functions.

> For more information, you can refer to the following resources:
> - [Master Theorem (Wikipedia)](https://www.google.com/search?q=Master+Theorem+Wikipedia)
> - [Master Theorem (GeeksforGeeks)](https://www.google.com/search?q=Master+Theorem+GeeksforGeeks)
> - [Master Theorem (Khan Academy)](https://www.google.com/search?q=Master+Theorem+Khan+Academy)
> - [Introduction to Algorithms (Google Books)](https://www.google.com/search?q=Introduction+to+Algorithms+Google+Books)