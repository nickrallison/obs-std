---
bad_links:
aliases: [Hoare Triple]
tags: [theoreticalcompsci]
---
# Hoare Logic

Hoare Logic is a formal system used to reason about the [[Total Correctness|correctness]] of computer programs. It was developed by Tony Hoare in the late 1960s and has since become an important tool in program verification and formal methods.

At its core, Hoare Logic uses a set of logical rules to reason about the behavior of programs. The main idea is to express properties of programs using pre and post-conditions. A pre-condition describes the state of the program before it executes a certain piece of code, while a post-condition describes the expected state after the execution.

The basic rule of Hoare Logic is called the Hoare Triple, and it has the following form:

$$
\{P\}\ C\ \{Q\}
$$

Here, $P$ is the pre-condition, $C$ is the code or program fragment, and $Q$ is the post-condition. The triple states that if the pre-condition $P$ holds before executing the code $C$, then the post-condition $Q$ will hold after the execution.

To reason about programs using Hoare Logic, we use a set of logical rules. Some of the most important rules include:

- **Assignment Rule**: If we have an assignment statement $x := E$, where $x$ is a variable and $E$ is an expression, we can derive the following Hoare Triple:

$$
\{P[E/x]\}\ x := E\ \{P\}
$$

Here, $P[E/x]$ denotes the substitution of all occurrences of $x$ in $P$ with $E$.

- **Composition Rule**: If we have two code fragments $C_1$ and $C_2$ with preand post-conditions $P$, $Q$, and $R$ respectively, we can derive the following Hoare Triple:

$$
\{P\}\ C_1\ \{Q\}\ \{Q\}\ C_2\ \{R\}\ \Rightarrow\ \{P\}\ C_1;C_2\ \{R\}
$$

This rule allows us to reason about the behavior of sequential code.

- **Conditional Rule**: If we have a conditional statement $if\ B\ then\ C_1\ else\ C_2$, where $B$ is a boolean expression and $C_1$, $C_2$ are code fragments, we can derive the following Hoare Triple:

$$
\{P \land B\}\ C_1\ \{Q\}\ \{P \land \neg B\}\ C_2\ \{Q\}\ \Rightarrow\ \{P\}\ if\ B\ then\ C_1\ else\ C_2\ \{Q\}
$$

This rule allows us to reason about the behavior of conditional code.

These are just a few examples of the rules used in Hoare Logic. There are many more rules that allow us to reason about loops, function calls, and other programming constructs.

Hoare Logic provides a rigorous framework for reasoning about program [[Total Correctness|correctness]]. By using these logical rules, we can prove properties of programs and ensure that they behave as intended. It has been widely used in the development of critical software systems, where [[Total Correctness|correctness]] is of utmost importance.

For more in-depth information on Hoare Logic, you can refer to the following resources:

- [Hoare Logic - Wikipedia](https://en.wikipedia.org/wiki/Hoare_logic)
- [An Introduction to Hoare Logic](https://www.cs.cmu.edu/~crary/819-f09/Hoare69.pdf)
- [Hoare Logic and Its Applications](https://www.cs.cmu.edu/~crary/819-f09/Hoare69.pdf)
