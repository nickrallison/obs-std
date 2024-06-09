---
bad_links: 
aliases: [antisymmetric, antisymmetry]
tags: [logic]
---
# Antisymmetric Relation

An antisymmetric relation is a type of binary relation. A binary relation R on a set A is said to be antisymmetric if for all a, b in A, whenever (a, b) and (b, a) are in R, then a = b. In mathematical notation, this is expressed as:

$$
\forall a, b \in A, ((a, b) \in R \land (b, a) \in R) \rightarrow a = b
$$

This definition implies that if we have a pair (a, b) in the relation with a ≠ b, then the pair (b, a) must not be in the relation.

Let's consider an example to illustrate this concept. Let A = {1, 2, 3} and let R be a relation on A defined as R = {(1, 1), (2, 2), (3, 3), (1, 2), (2, 3)}. This relation is antisymmetric. For every pair (a, b) in R where a ≠ b, the pair (b, a) is not in R. And for pairs where a = b, the condition of antisymmetry is trivially satisfied.

Antisymmetric relations are prevalent in mathematics and computer science. For instance, the "less than or equal to" (≤) relation is an example of an antisymmetric relation. If for two real numbers, a ≤ b and b ≤ a, then it must be the case that a = b.

Antisymmetry is one of the properties that define a partially ordered set, or poset. A poset is a set equipped with a binary relation that is reflexive (every element is related to itself), antisymmetric, and transitive (if a is related to b, and b is related to c, then a is related to c).

> For further reading, you might find the following resources useful:
> - [Antisymmetric Relation - Wikipedia](https://www.google.com/search?q=Antisymmetric+Relation+Wikipedia)
> - [Partially Ordered Set - Wikipedia](https://www.google.com/search?q=Partially+Ordered+Set+Wikipedia)
> - [Binary Relation - Wikipedia](https://www.google.com/search?q=Binary+Relation+Wikipedia)