---
bad_links: 
aliases: [loop invariants]
tags: [algorithms]
---
# Loop Invariant

A loop invariant is a condition that holds true before and after each iteration of a loop. It is a useful tool in program verification and analysis, as it helps ensure the [[Total Correctness|correctness]] of loop-based algorithms. Loop invariants are typically used in formal methods, such as program verification and formal proofs.

The loop invariant is a property that remains unchanged throughout the execution of a loop. It is often expressed as a logical formula or a set of conditions that must hold true at specific points within the loop. The loop invariant can be used to reason about the [[Total Correctness|correctness]] of the loop and the algorithm it implements.

To understand the concept of a loop invariant, let's consider a simple example. Suppose we have a loop that calculates the factorial of a given number `n`. The loop invariant in this case could be that the variable `result` always contains the factorial of the numbers processed so far.

Here's an example loop that calculates the factorial of `n`:

```python
result = 1
i = 1
while i <= n:
    result *= i
    i += 1
```

In this example, the loop invariant is that `result` contains the factorial of the numbers from 1 to `i-1` at the beginning of each iteration. This invariant holds true before the loop starts (`result = 1`), and it is maintained throughout the loop execution.

Loop invariants are useful for proving the [[Total Correctness|correctness]] of algorithms. By establishing a loop invariant, we can reason about the behavior of the loop and ensure that it terminates correctly and produces the expected result. In the case of the factorial example, the loop invariant guarantees that `result` will contain the correct factorial of `n` at the end of the loop.

To prove the [[Total Correctness|correctness]] of a loop, we typically need to show three things:

1. **Initialization**: The loop invariant holds true before the loop starts. In our example, the loop invariant is true before the loop starts because `result` is initialized to 1.

2. **Maintenance**: If the loop invariant holds true before an iteration, it remains true after the iteration. In our example, the loop invariant is maintained because `result` is multiplied by `i` at each iteration.

3. **Termination**: The loop terminates eventually. In our example, the loop terminates when `i` becomes greater than `n+1`.

By establishing these three properties, we can prove that the loop invariant holds true before and after each iteration, ensuring the [[Total Correctness|correctness]] of the loop.

Loop invariants are a powerful tool in program verification and analysis. They help ensure the [[Total Correctness|correctness]] of loop-based algorithms and provide a formal way to reason about the behavior of loops. By understanding and utilizing loop invariants, programmers can write more reliable and robust code.

> For more [[Information Theory|information]] on loop invariants and their applications, you can refer to the following resources:
> 
> - [Loop Invariants - Wikipedia](https://en.wikipedia.org/wiki/Loop_invariant)
> - [Loop Invariants - Stanford University](https://web.stanford.edu/class/archive/cs/cs161/cs161.1168/lecture9.pdf)
> - [Loop Invariants - Cornell University](https://www.cs.cornell.edu/courses/cs3110/2019sp/textbook/proof/loop_invariants.html)