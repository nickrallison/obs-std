---
bad_links:
aliases:
  - entail
  - entails
tags:
  - logic
---
# Entailment

Entailment in logic is a fundamental concept that describes the relationship between statements (or propositions) in a way that reflects how the truth of one or more statements guarantees the truth of another statement. It is a key concept in deductive reasoning, a process used to reach conclusions that are logically unavoidable given a set of premises.

## Formal Definition

Formally, entailment can be defined as follows: A set of statements \(S\) entails a statement \(P\) if and only if, in all cases where the statements in \(S\) are true, the statement \(P\) is also true. This is symbolically represented as \(S \models P\), where the symbol \(\models\) stands for 'entails'.

## Examples of Entailment

To illustrate, consider the following statements for a simple example:

1. All humans are mortal. (Statement A)
2. Socrates is a human. (Statement B)

From these statements, it logically follows that:

1. Socrates is mortal. (Statement C)

Here, statements A and B together entail statement C. This example demonstrates classical syllogistic reasoning, a form of deductive reasoning.

## Properties of Entailment

Entailment has several important properties that characterize its logical behavior:

- **[[transitive_relation.md|Transitivity]]**: If \(A \models B\) and \(B \models C\), then \(A \models C\).
- **[[reflexive_relation.md|Reflexivity]]**: Any set of statements \(S\) always entails itself.
- **Monotonicity**: If \(S \models P\) and \(S \subseteq T\), then \(T \models P\). This means that adding more premises does not invalidate an entailment relationship.

## Types of Entailment

There are different types of entailment that cater to different logics, including but not limited to:

- **Logical Entailment**: Entailment under the purview of classical logic, focusing on the logical consequence of statements.
- **Semantic Entailment**: Focuses on the meanings of the involved propositions beyond their logical form.
- **Syntactic Entailment**: Concerns the derivation of statements using formal systems or rules.

## Entailment in Mathematical Logic

In mathematical logic, entailment is rigorously defined using [[Formal Languages.md|formal languages]] like truth tables, models, and proof systems. It is particularly valuable in proving theorems, where demonstrating that premises entail a conclusion is central to mathematical reasoning.

## Entailment and Inference

Entailment is closely related to the process of inference, where one deduces new statements from a given set of premises. In logical systems, inference rules are defined to facilitate the derivation of entailments in a systematic way.

## Conclusion

Entailment is a cornerstone concept in logic, encapsulating the essence of logical deduction and reasoning. It enables the understanding and application of logical consequences in diverse areas, including mathematics, philosophy, computer science, and more. Understanding entailment is crucial for anyone delving into logical reasoning or related fields.