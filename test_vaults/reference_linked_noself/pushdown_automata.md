---
bad_links: 
aliases: [PDA, Pushdown Automaton]
tags: [theoreticalcompsci]
---
# Pushdown [[Automata Theory|Automata Theory]]

A Pushdown [[Automata Theory|Automaton]] (PDA) is a theoretical model of computation that extends the capabilities of a [[Finite Automata|finite automaton]] by incorporating a stack. It is used to recognize [[Context-Free Language|context-free languages]], which are a subset of [[Formal Languages|formal languages]].

A PDA consists of the following components:
1. A finite set of states.
2. An input alphabet.
3. A stack alphabet.
4. A transition function that specifies how the PDA transitions between states based on the current input symbol and the top symbol of the stack.
5. A start state.
6. A set of accepting states.

The behavior of a PDA is determined by its transition function, which is defined as follows:

$\delta: Q \times (\Sigma \cup \{\varepsilon\}) \times (\Gamma \cup \{\varepsilon\}) \rightarrow 2^{Q \times \Gamma^*}$

where:
- $Q$ is the set of states.
- $\Sigma$ is the input alphabet.
- $\Gamma$ is the stack alphabet.
- $\varepsilon$ represents the empty string.
- $2^{Q \times \Gamma^*}$ denotes the [[Powerset|power set]] of $Q \times \Gamma^*$, which represents the set of possible transitions.

The transition function $\delta$ takes as input the current state, the current input symbol (or $\varepsilon$ for an empty input), and the top symbol of the stack (or $\varepsilon$ for an empty stack). It returns a set of possible transitions, each consisting of a new state and a string of symbols to be pushed onto the stack.

A PDA operates as follows:
1. It starts in the start state with an empty stack.
2. It reads an input symbol and the top symbol of the stack.
3. Based on the current state, input symbol, and stack symbol, it applies the transition function to determine the next state and the symbols to be pushed onto the stack.
4. It repeats steps 2 and 3 until it reaches a state where no further transitions are possible.
5. If the PDA is in an accepting state and the input has been fully consumed, it accepts the input; otherwise, it rejects the input.

Pushdown [[Automata Theory|Automata]] are closely related to context-free grammars. In fact, for every context-free grammar, there exists an equivalent PDA, and vice versa. This relationship is known as the Chomsky-Schützenberger theorem.

Pushdown [[Automata Theory|Automata]] can be represented using various formalisms, such as transition diagrams, transition tables, or transition functions. Additionally, they can be simulated using programming languages or implemented in hardware.

If you would like to explore more about Pushdown [[Automata Theory|Automata]], I recommend the following resources:
- [Pushdown Automaton - Wikipedia](https://en.wikipedia.org/wiki/Pushdown_automaton)
- [Introduction to the Theory of Computation by Michael Sipser](https://www.amazon.com/Introduction-Theory-Computation-Michael-Sipser/dp/113318779X)

> "A PDA is a theoretical model of computation that extends the capabilities of a [[Finite Automata|finite automaton]] by incorporating a stack. It is used to recognize [[Context-Free Language|context-free languages]]." - [Wikipedia](https://en.wikipedia.org/wiki/Pushdown_automaton)