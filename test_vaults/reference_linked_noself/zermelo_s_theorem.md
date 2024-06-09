---
aliases: []
tags: [logic, gametheory]
bad_links: [Ordering.md]
---
# Zermelo's Theorem

Zermelo's Theorem is a fundamental result in game theory, specifically in the study of finite two-person games of perfect information. Ernst Zermelo, a German mathematician, first published this theorem in 1913. The theorem states that in any finite two-person game of perfect [[Information Theory|information]] in which the players move alternately and in which chance does not affect the decision making process, if the game cannot end in a draw, then one of the two players must have a winning strategy.

To understand this theorem, it's important to clarify some terms:

- **Finite game**: A game with a finite number of possible positions.
- **Two-person game**: A game with exactly two players.
- **Perfect information**: A game where each player, when making any decision, is fully aware of all the events that have previously occurred.
- **Winning strategy**: A strategy that guarantees a win for a player, no matter what the opponent does.

The proof of Zermelo's theorem is based on the concept of backward [[Induction Proofs|induction]] and the [[Well Ordering Principle|well-ordering principle]]. 

The [[Well Ordering Principle|well-ordering principle]] is a fundamental [[Mathematical Axioms|axiom]] in set theory, which states that every non-empty set of positive integers contains a least element. This principle is used to prove that there exists a sequence of optimal moves leading to an end position.

Backward [[Induction Proofs|induction]], on the other hand, is a decision-making process where one first considers the last possible decision and chooses the optimal action, then moves backward to the second last decision, and so on.

The proof of Zermelo's theorem proceeds by showing that, given the game's perfect [[Information Theory|information]] structure and the [[Well Ordering Principle|well-ordering principle]], one can use backward [[Induction Proofs|induction]] to construct a winning strategy for one of the players.

Here is a simplified version of the proof:

1. Consider the set of all positions from which the first player can force a win. If the initial position is in this set, then the first player has a winning strategy.
2. If the initial position is not in this set, then no matter what the first player does, the second player can always respond with a move to a position outside this set. Hence, the second player can ensure that the game position always stays outside this set, and therefore, the second player can force a win.

This proof is a high-level overview and the actual mathematical proof requires a deep understanding of set theory and game theory.

> For a more detailed understanding, you may want to read the original paper by Ernst Zermelo titled "[On an Application of Set Theory to the Theory of the Game of Chess](https://www.google.com/search?q=On+an+Application+of+Set+Theory+to+the+Theory+of+the+Game+of+Chess)".

> You may also find it helpful to study the concepts of "[Backward Induction](https://www.google.com/search?q=Backward+Induction)" and the "[Well-Ordering Principle](https://www.google.com/search?q=Well-Ordering+Principle)" in more depth.