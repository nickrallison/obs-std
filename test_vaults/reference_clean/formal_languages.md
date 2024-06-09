---
bad_links: 
aliases: [Chomsky hierarchy]
tags: [computerscience, theoreticalcompsci]
---
# Formal Languages

Formal Languages are a fundamental concept in computer science and mathematics. They provide a precise way to describe and analyze the structure and behavior of various systems, such as programming languages, communication protocols, and natural languages.

A formal language is a set of strings composed of symbols from a given alphabet. The alphabet is a finite set of symbols, and a string is a finite sequence of symbols from the alphabet. For example, consider an alphabet consisting of the symbols {0, 1}. The set of all binary strings, such as "0101" or "111000", is a formal language over this alphabet.

Formal languages are often defined using formal grammars. A formal grammar consists of a set of production rules that specify how to generate valid strings in the language. The most commonly used formal grammar is the context-free grammar (CFG). A CFG consists of a set of non-terminal symbols, terminal symbols, and production rules. Non-terminal symbols represent abstract entities, while terminal symbols represent the actual symbols in the language. Production rules define how to rewrite non-terminal symbols into sequences of terminal and non-terminal symbols.

## Chomsky Hierarchy

The Chomsky hierarchy is a classification of formal languages based on the expressive power of the grammar used to define them. It consists of four levels:

1. Type-3 (Regular Languages): These languages can be described by regular expressions or finite automata. Regular languages are closed under union, concatenation, and Kleene star operations.

2. Type-2 (Context-Free Languages): These languages can be described by context-free grammars. Context-free languages are closed under union, concatenation, and Kleene star operations, but not under complementation.

3. Type-1 (Context-Sensitive Languages): These languages can be described by context-sensitive grammars. Context-sensitive languages are closed under union, concatenation, Kleene star, and complementation operations.

4. Type-0 (Recursively Enumerable Languages): These languages can be described by unrestricted grammars or Turing machines. Recursively enumerable languages are closed under union, concatenation, Kleene star, complementation, and intersection operations.

The Pumping Lemma is a powerful tool used to prove that a language is not regular. It states that for any regular language, there exists a pumping length such that any string longer than the pumping length can be divided into five parts, where the middle three parts can be repeated any number of times while still remaining in the language.

Formal languages have numerous applications in computer science, including compiler design, natural language processing, and formal verification. They provide a rigorous framework for analyzing and manipulating strings and are essential for understanding the theoretical foundations of computation.

For a more comprehensive understanding of formal languages, you may find the following resources helpful:

- [Formal Languages and Automata Theory](https://en.wikipedia.org/wiki/Formal_languages_and_automata_theory) (Wikipedia)
- [Introduction to the Theory of Computation](https://www.amazon.com/Introduction-Theory-Computation-Michael-Sipser/dp/113318779X) by Michael Sipser (Book)
- [Formal Languages and Their Relation to Automata](https://www.amazon.com/Formal-Languages-Relation-Automata-Addison-Wesley/dp/0201441241) by John E. Hopcroft and Jeffrey D. Ullman (Book)

