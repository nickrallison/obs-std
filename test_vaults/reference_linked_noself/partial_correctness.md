---
bad_links: 
aliases: []
tags: [algorithms]
---
# Partial [[Total Correctness|Correctness]]

Partial [[Total Correctness|Correctness]] is a property of a computer program that, if it terminates, it gives the correct answer. It's a key concept in the field of formal methods and program verification. The term "partial" is used because the property doesn't guarantee that the program will terminate; it only assures that if the program does terminate, the result will be correct.

The concept of partial [[Total Correctness|correctness]] is often explained using [[Hoare Logic|Hoare Logic]], a formal system developed by British computer scientist Tony Hoare. [[Hoare Logic|Hoare Logic]] is used to reason about the [[Total Correctness|correctness]] of computer programs, and it involves the use of preconditions and postconditions.

A precondition is a condition that is assumed to be true before the execution of a program or a part of a program. A postcondition, on the other hand, is a condition that the program guarantees to be true after its execution, if the preconditions were true before its execution.

In [[Hoare Logic|Hoare Logic]], a partial [[Total Correctness|correctness]] assertion for a program `P` is written as:

$$
\{P\} C \{Q\}
$$

Here, `P` is the precondition, `C` is the command (or program), and `Q` is the postcondition. This assertion is read as "if `P` holds before the execution of `C`, and if `C` terminates, then `Q` will hold after `C`'s execution."

The proof of partial [[Total Correctness|correctness]] often involves the use of a [[Loop Invariant|loop invariant]]. A [[Loop Invariant|loop invariant]] is a condition that is initially true before a loop begins and remains true after each iteration of the loop. It's used to show that a loop maintains certain properties each time it runs, which helps in proving the [[Total Correctness|correctness]] of algorithms.

> For further reading, you might find the following resources helpful:
> - [Hoare Logic - Wikipedia](https://www.google.com/search?q=Hoare+Logic+Wikipedia)
> - [Formal Methods - Wikipedia](https://www.google.com/search?q=Formal+Methods+Wikipedia)
> - [Program Verification - Wikipedia](https://www.google.com/search?q=Program+Verification+Wikipedia)
> - [Loop Invariant - Wikipedia](https://www.google.com/search?q=Loop+Invariant+Wikipedia)