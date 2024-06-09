---
bad_links: 
aliases: []
tags: [theoreticalcompsci]
---
# Formal Grammar

Formal grammar is a mathematical framework used to describe the structure and rules of a language. It provides a systematic way to analyze and generate sentences in a language. In formal grammar, a language is defined by a set of rules that specify how sentences can be formed.

One of the most commonly used formal grammars is the Chomsky hierarchy, which classifies formal languages into four types: Type 3 (regular), Type 2 (context-free), Type 1 (context-sensitive), and Type 0 (unrestricted). Each type of grammar has its own set of rules and restrictions.

The most basic type of formal grammar is a regular grammar, which is defined by regular expressions. Regular expressions are patterns that describe a set of strings. They can be used to define simple languages, such as the set of all strings that start with an 'a' and end with a 'b'. Regular grammars can be represented by finite automata, which are mathematical models that recognize or generate strings based on a set of states and transitions.

Context-free grammars are more powerful than regular grammars and are widely used in programming languages and natural language processing. They are defined by a set of production rules, where each rule consists of a non-terminal symbol and a sequence of symbols. Non-terminal symbols can be replaced by other symbols according to the production rules. Context-free grammars can be represented by pushdown automata, which are finite automata with an additional stack memory.

Context-sensitive grammars are even more powerful than context-free grammars. They allow rules to have context-dependent conditions, where the replacement of symbols depends on the surrounding context. Context-sensitive grammars can be represented by linear-bounded automata, which are finite automata with a tape that can be used to store and manipulate symbols.

Finally, unrestricted grammars have no restrictions on the rules and can generate any language. They are the most powerful type of grammar and can be represented by Turing machines, which are theoretical models of computation.

Formal grammar also includes the concept of derivations, which describe how a sentence can be generated from a grammar. Derivations involve applying production rules to non-terminal symbols until only terminal symbols remain. The process of deriving a sentence can be represented as a derivation tree, which shows the hierarchical structure of the sentence.

In summary, formal grammar provides a mathematical framework for analyzing and generating sentences in a language. It includes different types of grammars, such as regular, context-free, context-sensitive, and unrestricted grammars. Each type has its own set of rules and restrictions. Derivations and derivation trees are used to describe how sentences can be generated from a grammar.

> For more information on formal grammar, you can refer to the following resources:
> 
> - [Formal Grammar - Wikipedia](https://en.wikipedia.org/wiki/Formal_grammar)
> - [Chomsky Hierarchy - Wikipedia](https://en.wikipedia.org/wiki/Chomsky_hierarchy)
> - [Regular Expressions - Wikipedia](https://en.wikipedia.org/wiki/Regular_expression)
> - [Finite Automaton - Wikipedia](https://en.wikipedia.org/wiki/Finite_automaton)
> - [Pushdown Automaton - Wikipedia](https://en.wikipedia.org/wiki/Pushdown_automaton)
> - [Linear-Bounded Automaton - Wikipedia](https://en.wikipedia.org/wiki/Linear-bounded_automaton)
> - [Turing Machine - Wikipedia](https://en.wikipedia.org/wiki/Turing_machine)