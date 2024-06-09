---
bad_links: 
aliases: []
tags: [theoreticalcompsci]
---
# Turing Machine.md|Nondeterministic Turing Machine]]

A Turing Machine.md|Nondeterministic Turing Machine]] (NTM) is a theoretical model of computation. It extends the concept of a deterministic Turing machine by allowing multiple possible next states for each state and symbol pair in its transition function. This means that for a given input, an NTM can have multiple possible computational paths and outcomes.

The formal definition of an NTM is similar to that of a deterministic Turing machine. An NTM M is a 7-tuple $(Q, \Sigma, \Gamma, \delta, q_0, q_{accept}, q_{reject})$ where:

- $Q$ is a finite set of states
- $\Sigma$ is a finite set of input symbols
- $\Gamma$ is a finite set of tape symbols, where $\Sigma \subseteq \Gamma$ and the blank symbol $\sqcup \in \Gamma - \Sigma$
- $\delta: Q \times \Gamma \rightarrow P(Q \times \Gamma \times \{L, R\})$ is the transition function, where $P$ denotes the power set
- $q_0 \in Q$ is the start state
- $q_{accept} \in Q$ is the accept state
- $q_{reject} \in Q$ is the reject state

The transition function $\delta$ is what makes an NTM nondeterministic. Instead of mapping to a single next state, symbol, and movement direction, it maps to a set of such tuples. This allows the machine to "branch" into multiple computational paths at each step.

An NTM accepts an input if there exists at least one computational path that leads to the accept state. It rejects an input if all computational paths lead to the reject state. If some paths lead to neither state, the behavior of the machine is undefined.

The concept of nondeterminism is crucial in the theory of computation, particularly in the study of computational complexity. The class of problems that can be solved by an NTM in polynomial time is known as NP (nondeterministic polynomial time). The famous P vs NP problem asks whether the class P of problems solvable by a deterministic Turing machine in polynomial time is the same as NP.

> For more information, you may want to read about the [P vs NP problem](https://www.google.com/search?q=P+vs+NP+problem), [deterministic Turing machines](https://www.google.com/search?q=deterministic+Turing+machines), and [computational complexity theory](https://www.google.com/search?q=computational+complexity+theory).