---
aliases: []
tags: [grouptheory]
bad_links: [Binary Representation.md]
---
# Binary Operation

A binary operation is a calculation that combines two elements (called operands) to produce another element. More formally, a binary operation on a set $S$ is a function that maps elements of the Cartesian product $S \times S$ to $S$:

$$
*: S \times S \rightarrow S
$$

In other words, if $a$ and $b$ are elements of $S$, then the binary operation $*$ takes $a$ and $b$ and produces another element in $S$, which we denote as $a * b$.

Common examples of binary operations include addition, subtraction, multiplication, and division in the set of real numbers $\mathbb{R}$. For instance, addition $(+)$ is a binary operation defined on $\mathbb{R}$ because for any two real numbers $a$ and $b$, their sum $a + b$ is also a real number.

Binary operations are fundamental to many areas of mathematics, including algebra, calculus, and number theory. They are used to define mathematical structures such as groups, rings, and fields.

A **group** is a set $G$ together with a binary operation $*$ that satisfies the following properties:

1. **[[Closure.md|Closure]]**: For all $a, b \in G$, the result of the operation $a * b$ is also in $G$.
2. **[[Associativity.md|Associativity]]**: For all $a, b, c \in G$, $(a * b) * c = a * (b * c)$.
3. **[[Identity Element.md|Identity element]]**: There is an element $e \in G$ such that for every element $a \in G$, the equations $e * a$ and $a * e$ return $a$.
4. **[[Inverse Element.md|Inverse element]]**: For each element $a \in G$, there exists an element $b \in G$ such that $a * b = b * a = e$.

A **ring** is a set $R$ equipped with two binary operations, usually denoted as addition $(+)$ and multiplication $(\cdot)$, that satisfy the properties of a group under addition and some additional properties under multiplication.

A **field** is a set $F$ equipped with two binary operations, usually denoted as addition $(+)$ and multiplication $(\cdot)$, that satisfy the properties of a ring and some additional properties, including the existence of a multiplicative inverse for every non-zero element.

> For more information, you may want to read about [Binary Operations](https://en.wikipedia.org/wiki/Binary_operation), [Groups](https://en.wikipedia.org/wiki/Group_(mathematics)), [Rings](https://en.wikipedia.org/wiki/Ring_(mathematics)), and [Fields](https://en.wikipedia.org/wiki/Field_(mathematics)) on Wikipedia.