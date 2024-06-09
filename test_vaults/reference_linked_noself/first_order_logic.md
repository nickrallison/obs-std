---
bad_links: 
aliases: [FOL, predicate logic, predicate calculus]
tags: [proofs, logic]
title: First Order Logic
date created: Wednesday, July 12th 2023, 10:50:54 am
---

# First Order Logic

First Order Logic (FOL), also known as Predicate Logic or First-Order Predicate Calculus, is a formal logical system used in mathematics, philosophy, linguistics, and computer science. It extends [[Truth Functional Logic|propositional logic]] by allowing quantified variables over non-logical objects and using predicates to include them in propositions. The variables can refer to objects in the problem domain and the predicates can express properties about the objects. FOL is powerful enough to formalize a large portion of mathematics and it's the basis of most modern theorem proving software.

FOL is the basis of modern mathematics alongside [[Zermelo-Fraenkel Set Theory|Zermelo-Fraenkel Set Theory]] which is used to prove most results in modern mathematics.

Suppose we have the following knowledge base (KB):

1. All humans are mortal.
2. Socrates is a human.

We want to prove that Socrates is mortal.

In first-order logic, we can represent the above statements as follows:

1. ∀x(Human(x) → Mortal(x))
2. Human(Socrates)

We want to prove: Mortal(Socrates)

Here's how we can solve it:

$$
\begin{gather*} 
M(x) \rightarrow \text{x is mortal} \\
H(x) \rightarrow \text{x is Human} \\\\
\forall x(H(x) \rightarrow M(x)) \\
H(Socrates) \\
\Downarrow \text{(Applying Universal Instantiation on the first statement)} \\
H(Socrates) \rightarrow M(Socrates) \\
\Downarrow \text{(Modus Ponens with second statement)} \\
M(Socrates)
\end{gather*}
$$
So, based on our knowledge base, we have proved that Socrates is mortal using first-order logic.
