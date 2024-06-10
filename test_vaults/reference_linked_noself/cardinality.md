---
bad_links: 
aliases: [cardinal]
tags: [settheory]
---
# Cardinality

In set theory, the concept of cardinality provides a way to compare the sizes of sets, including infinite sets, in a precise manner. It is a measure of the "number of elements" of the set. For sets with a finite number of elements, this is straightforward and coincides with our intuitive understanding of size. However, for sets that are infinite, cardinality offers a deeper and more nuanced view of "size."

## Finite Sets

The cardinality of a finite set is simply the count of distinct elements in the set. If a set $A$ has $n$ elements, where $n$ is a non-negative integer, we say that the cardinality of $A$, denoted as $|A|$, is $n$. For example, for the set $A = \{2, 4, 6\}$, the cardinality $|A| = 3$.

## Infinite Sets

The concept of cardinality becomes more interesting when we deal with infinite sets. In this context, two sets are said to have the same cardinality if there exists a one-to-one correspondence between the elements of the two sets. This means that every element of one set can be paired with exactly one element of the other set, and vice versa, without any element being left unpaired in either set.

### Countably Infinite Sets

A set is considered *countably infinite* if its elements can be placed in one-to-one correspondence with the natural numbers $\mathbb{N} = \{0, 1, 2, 3, …\}$. For example, the set of all even positive integers $\{2, 4, 6, 8, …\}$ is countably infinite because we can map each even positive integer to a natural number (e.g., $2 \to 1, 4 \to 2, 6 \to 3$, and so on). All countably infinite sets share the same cardinality, denoted by $\aleph_0$ (read as "aleph-null" or "aleph-zero").

### Uncountably Infinite Sets

An infinite set that is not countable is called *uncountably infinite*. A classic example is the set of real numbers $\mathbb{R}$. The proof that $\mathbb{R}$ is uncountably infinite involves showing that there is no way to list all real numbers in a sequence that could be matched one-to-one with the natural numbers, a result famously demonstrated by Cantor's diagonal argument.

The cardinality of the set of real numbers is denoted by $2^{\aleph_0}$, indicating a "higher order" of infinity than that of countably infinite sets. Interestingly, between any two real numbers, no matter how close, there is an infinite number of real numbers, underscoring the profound difference in cardinality between infinite sets.

## Comparing Cardinalities

To determine if one set has a greater cardinality than another, mathematicians attempt to establish whether a one-to-one correspondence exists between the two sets. If such a correspondence can be found, the sets are said to have the same cardinality. If not, and assuming a correspondence can be found that includes all elements of one set but leaves out elements of the other, the latter set is considered to have a greater cardinality.

## Applications and Implications

The concept of cardinality finds applications across various fields of mathematics and computer science, especially in understanding the properties of different types of infinities, in the theory of [[computability.md|computability]], and in the analysis of algorithms and data structures. Its implications reach into philosophy and logic, provoking fundamental questions about the nature of mathematics and reality.

Cardinality not only serves as a foundational concept in set theory but also illustrates the beauty and depth of mathematical thought, revealing a universe where even infinity itself is distinguished and categorized.