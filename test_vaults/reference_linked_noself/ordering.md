---
bad_links: 
aliases: [“partial ordering”, “total ordering”, Partial Order, Total Order]
tags: [logic, proofs]
---
# Ordering

Partial and total ordering are concepts in mathematics and computer science that describe relationships between elements in a set. 

Total ordering, also known as linear ordering, is a relationship where any two elements can be compared with each other. In a totally ordered set, for any given two elements, one is considered greater than, less than, or equal to the other.

On the other hand, partial ordering is a relationship where not every pair of elements can be compared. It's possible for some pairs of elements to be incomparable. This means that within the set, there may exist certain pairs of elements where neither element precedes the other.

Both total and partial orderings are types of "order [[Binary Relation|relations]]," which are foundational concepts in set theory and many areas of mathematics and computer science.

A [[Binary Relation|relation]] "≤" on a set S is a total order if it satisfies the following for all a, b and c in S:

1. [[Reflexive Relation|Reflexivity]]: $a \leq a$
2. [[Antisymmetric Relation|Antisymmetry]]: If $a \leq b$ and $b \leq a$ then $a = b$
3. [[Transitive Relation|Transitivity]]: If $a \leq b$ and $b \leq c$ then $a \leq c$
4. [[Comparability|Comparability]]: Either $a \leq b$ or $b \leq a$

A partial order is defined similarly, but without the [[Comparability|comparability]] requirement. 

So for any elements 'a', 'b' and 'c' in set 'S':

Total Order:
$$
\begin{gather*}
\forall a,b,c \in S:\\
(a)~ a\leq a\\
(b)~ if~ (a\leq b ~and~ b\leq a) ~then~ (a=b)\\
(c)~ if~ (a\leq b ~and~ b\leq c) ~then~ (a\leq c)\\
(d)~ (a\leq b ~or~ b\leq a)
\end{gather*}
$$

Partial Order:
$$
\begin{gather*}
\forall a,b,c \in S:\\
(a)~ a\leq a\\
(b)~ if~ (a\leq b ~and~ b\leq a) ~then~ (a=b)\\
(c)~ if~ (a\leq b ~and~ b\leq c) ~then~ (a\leq c)
\end{gather*}
$$