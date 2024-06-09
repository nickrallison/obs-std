---
bad_links: 
tags: [logic]
date created: Sunday, July 9th 2023, 11:51:28 pm
title: Conjunctive Normal Form
aliases: ["CNF", "Product of Sums", "Product-of-Sums", "POS", "Product of Sum", "Product-of-Sum"]
---

# Conjunctive Normal Form

Conjunctive Normal Form (CNF) (Or Product of Sums) is a standardized form of representing logical formulas in [[Truth Functional Logic|propositional logic]]. In CNF, a formula is a conjunction of clauses, where each clause is a disjunction of literals. Essentially, it is an AND of ORs. For example, the CNF for the expression (A AND B) OR (C AND D) would be ((A OR C) AND (B OR D)). This form is widely used in various fields such as artificial intelligence and computational logic.

$$
\begin{gather*} 
(A \land B) \lor (C \land D) \equiv ((A \lor C) \land (B \lor D))
\end{gather*}
$$

The Minimal CNF can be obtained via a karnaugh map.