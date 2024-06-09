---
bad_links: 
aliases: []
tags: [probability, cryptography, numbertheory]
---
# Pollard's Rho Algorithm

Pollard's Rho algorithm is a probabilistic [[Prime Factorization|factorization]] algorithm, which means it relies on randomization, and the probability of finding a nontrivial factor of a composite number increases with time and does not depend on the size of the input number. It was invented by John Pollard in 1975.

The algorithm is particularly effective for numbers with small factors, but it is not limited to such numbers. It is based on the principle of Floyd's cycle-finding algorithm and the birthday paradox.

The algorithm works as follows:

1. Choose a function $f(x)$ and two random starting values $x_0$ and $x_1$ in the interval $[0, n-1]$.
2. For $i = 1, 2, 3, \ldots$ do the following:
   - Compute $x_i = f(x_{i-1}) \mod n$ and $x_{2i} = f(f(x_{2i-2})) \mod n$.
   - Compute $d = \gcd(|x_i - x_{2i}|, n)$.
   - If $d \neq 1$ and $d \neq n$, then $d$ is a nontrivial factor of $n$.

The function $f(x)$ is often chosen to be $f(x) = x^2 + a \mod n$ for some constant $a$, because this function has good mathematical properties.

The algorithm's name, "rho", refers to the fact that the sequence of $x_i$ values often forms a "rho" shape (like the Greek letter ρ) when plotted on a graph, with a linear "tail" and a circular "loop".

The [[Big-O Notation|time complexity]] of Pollard's Rho algorithm is $O(n^{1/4})$ on [[Expected Value|average]], which makes it one of the fastest known [[Prime Factorization|factorization]] algorithms for large numbers.

Here is a Python implementation of Pollard's Rho algorithm:

```python
import math
import random

def pollards_rho(n):
    if n % 2 == 0:
        return 2
    x = random.randint(1, n-1)
    y = x
    c = random.randint(1, n-1)
    g = 1
    while g == 1:
        x = ((x * x) % n + c + n) % n
        y = ((y * y) % n + c + n) % n
        y = ((y * y) % n + c + n) % n
        g = math.gcd(abs(x-y), n)
    return g
```

> For more information, you can refer to the following resources:
> - [Pollard's Rho Algorithm for Integer Factorization](https://en.wikipedia.org/wiki/Pollard%27s_rho_algorithm)
> - [Floyd's Cycle-Finding Algorithm](https://en.wikipedia.org/wiki/Cycle_detection)
> - [Birthday Paradox](https://en.wikipedia.org/wiki/Birthday_problem)