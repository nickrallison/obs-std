---
bad_links: 
aliases: [Relation]
tags: [logic]
---
# Binary Relation

A binary relation $R$ from a set $A$ to a set $B$ is a subset of the Cartesian product $A \times B$. In other words, it is a set of ordered pairs where the first element of each pair comes from $A$ and the second element comes from $B$. 

For example, if $A = \{1, 2\}$ and $B = \{3, 4\}$, then the Cartesian product $A \times B = \{(1, 3), (1, 4), (2, 3), (2, 4)\}$. A binary relation from $A$ to $B$ could be $R = \{(1, 3), (2, 4)\}$.

Binary relations are used to model a wide variety of concepts in mathematics and computer science. For example, the "less than" relation $<$ on the set of integers is a binary relation. 

There are several important properties that a binary relation may have:

1. **[[Reflexive Relation.md|Reflexivity]]**: A binary relation $R$ on a set $A$ is reflexive if for every element $a$ in $A$, the pair $(a, a)$ is in $R$. For example, the "less than or equal to" relation $\leq$ on the set of integers is reflexive because every integer is less than or equal to itself.

2. **[[Symmetric Relation.md|Symmetry]]**: A binary relation $R$ on a set $A$ is symmetric if for every pair $(a, b)$ in $R$, the pair $(b, a)$ is also in $R$. For example, the "equal to" relation $=$ on the set of integers is symmetric because if $a = b$, then $b = a$.

3. **[[Transitive Relation.md|Transitivity]]**: A binary relation $R$ on a set $A$ is transitive if for every pair of pairs $(a, b)$ and $(b, c)$ in $R$, the pair $(a, c)$ is also in $R$. For example, the "less than" relation $<$ on the set of integers is transitive because if $a < b$ and $b < c$, then $a < c$.

A binary relation that is reflexive, symmetric, and transitive is called an equivalence relation. Equivalence relations are very important in mathematics because they allow us to "group" elements of a set into equivalence classes.

> For more information, you can refer to the following resources:
> - [Binary Relation - Wikipedia](https://www.google.com/search?q=binary+relation+site:wikipedia.org)
> - [Equivalence Relation - Wikipedia](https://www.google.com/search?q=equivalence+relation+site:wikipedia.org)
> - [Cartesian Product - Wikipedia](https://www.google.com/search?q=cartesian+product+site:wikipedia.org)