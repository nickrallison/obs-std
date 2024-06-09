---
bad_links:
aliases: []
tags: [logic, computerarchitecture]
---
# Boolean Algebra

Boolean algebra is a branch of mathematics that deals with operations on logical values. It was introduced by George Boole in the mid-19th century. Boolean algebra involves variables that can take two values: true or false, 1 or 0. It is fundamental in the design of digital circuits and computer programming, as it helps to describe the manipulation and processing of binary information. The main operations in Boolean algebra are conjunction (and), disjunction (or), and negation (not).

## Rules

Boolean algebra follows certain laws and properties that are similar to regular algebra but are specifically designed for binary operations. These rules include:

1. Idempotent Law: This law states that a value combined with itself will always result in the original value.

$$
A \land A = A
$$
$$
A \lor A = A
$$

1. Null Law: This law states that a value combined with its negation will always result in the null value. For example, A AND (NOT A) = 0, A OR (NOT A) = 1.

$$
A \land \lnot A = 0
$$
$$
A \lor \lnot A = 1
$$

1. Annulment Law: This law states that a value combined with its complement will always result in the identity value. For example, A AND (NOT A) = 0, A OR (NOT A) = 1.

$$
A \land \lnot A = 0
$$
$$
A \lor \lnot A = 1
$$

1. Dominance Law: This law states that a value combined with a dominant value will always result in the dominant value. For example, A AND 0 = 0, A OR 1 = 1.
   $$
   A \land 0 = 0
   $$
   $$
   A \lor 1 = 1
   $$

2. Involution Law: This law states that the complement of the complement of a variable is equal to the variable itself. For example, NOT (NOT A) = A.

$$
\lnot \lnot A = A
$$

1. Absorption law:

$$
A \land (A \lor B)) = A
$$
$$
A \lor (A \land B)) = A
$$

1. De Morgan’s law
   $$
   \lnot(A \land B) = (\lnot A \lor \lnot B)
   $$
   $$
   \lnot(A \lor B) = (\lnot A \land \lnot B)
   $$