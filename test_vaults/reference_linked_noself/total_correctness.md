---
bad_links: 
aliases: [correctness, algorithmic correctness]
tags: [algorithms, proofs]
---
# Total Correctness

Total Correctness is a concept in computer science that ensures both the termination and correctness of a program. It guarantees that a program will always produce the correct output if it terminates. In other words, a program is totally correct if it satisfies both [[Partial Correctness|partial correctness]] and termination.

To formally define Total Correctness, we use [[Hoare Logic|Hoare logic]], which is a formal system for reasoning about the correctness of computer programs. [[Hoare Logic|Hoare triples]] are used to express preconditions, postconditions, and the effect of a program.

A [[Hoare Logic|Hoare triple]] is written as {P} C {Q}, where P is the precondition, C is the program, and Q is the postcondition. P and Q are logical assertions that describe the state of the program before and after execution, respectively.

To prove Total Correctness, we need to show that the program satisfies both [[Partial Correctness|partial correctness]] and termination.

[[Partial Correctness|Partial Correctness]] is the property that if the program terminates, it produces the correct output. It can be expressed as {P} C {Q}, where P is the precondition, C is the program, and Q is the postcondition. To prove [[Partial Correctness|partial correctness]], we use the following rules:

- Assignment Rule: If {P\[e/x\]} x := e {P}, where P\[e/x\] denotes the substitution of e for x in P, then {P} x := e {P}.
- Composition Rule: If {P} C1 {Q} and {Q} C2 {R}, then {P} C1; C2 {R}.
- Conditional Rule: If {P ∧ B} C1 {Q} and {P ∧ ¬B} C2 {Q}, then {P} if B then C1 else C2 {Q}.
- Loop Rule: If {P ∧ B} C {P}, then {P} while B do C {P ∧ ¬B}.

Termination is the property that the program will eventually halt. To prove termination, we use techniques such as [[Loop Invariant|loop invariants]], which are assertions that hold true before and after each iteration of a loop.

In addition to [[Hoare Logic|Hoare logic]], we can also use formal methods such as model checking and theorem proving to verify the total correctness of a program. These methods involve mathematical techniques and automated tools to analyze the program and prove its correctness.

In summary, Total Correctness ensures both the termination and correctness of a program. It is defined using [[Hoare Logic|Hoare logic]] and requires proving both [[Partial Correctness|partial correctness]] and termination. Formal methods can be used to verify the total correctness of a program.

> For more [[Information Theory|information]] on Total Correctness and [[Formal Verification|formal verification]] in computer science, you can refer to the following resources:
> 
> - [Formal Methods: State of the Art and Future Directions](https://www.google.com/search?q=Formal+Methods%3A+State+of+the+Art+and+Future+Directions+site%3Aacm.org)
> - [Hoare Logic](https://www.google.com/search?q=Hoare+Logic+site%3Astanford.edu)
> - [Model Checking](https://www.google.com/search?q=Model+Checking+site%3Amit.edu)
> - [Theorem Proving](https://www.google.com/search?q=Theorem+Proving+site%3Astanford.edu)