---
bad_links: 
aliases: []
tags: [numbertheory]
---
# Sieve of Eratostheness

The Sieve of Eratosthenes is an ancient algorithm for finding all prime numbers up to any given limit. It does so by iteratively marking the multiples of each prime number, starting from 2. The multiples of a given prime are generated as a sequence of numbers starting from that prime, with a constant difference between them that is equal to that prime. This is the key distinction between using trial division to sequentially test each candidate number for divisibility by each prime.

The algorithm can be expressed in pseudocode as follows:

```
Algorithm Sieve of Eratosthenes(n)
    Let A be an array of Boolean values, indexed by integers 2 to n,
    initially all set to true.
    
    for i = 2, 3, 4, ..., not exceeding √n:
        if A[i] is true:
            for j = i^2, i^2+i, i^2+2i, i^2+3i, ..., not exceeding n :
                A[j] := false

    return all i such that A[i] is true.
```

The [[Big-O Notation|time complexity]] of the algorithm is $O(n \log \log n)$, which makes it one of the most efficient ways to find all primes smaller than n when n is smaller than 10 million or so.

The Sieve of Eratosthenes is a specific example of a broader category of algorithms, known as "sieve algorithms". Other examples of sieve algorithms include the Sieve of Sundaram and the Sieve of Atkin, which are used for the same purpose but with different methods.

The Sieve of Eratosthenes is also related to the concept of prime number theory, as it is a practical method for obtaining prime numbers, which are a fundamental concept in number theory. Prime numbers have several unique properties and are used in various fields, including cryptography.

> For more context and reading, you can refer to the following resources:
> - [Sieve of Eratosthenes - Wikipedia](https://www.google.com/search?q=Sieve+of+Eratosthenes)
> - [Prime Numbers - Wikipedia](https://www.google.com/search?q=Prime+Numbers)
> - [Sieve Theory - Wikipedia](https://www.google.com/search?q=Sieve+Theory)
> - [Sieve of Sundaram - Wikipedia](https://www.google.com/search?q=Sieve+of+Sundaram)
> - [Sieve of Atkin - Wikipedia](https://www.google.com/search?q=Sieve+of+Atkin)
> - [Prime Number Theorem - Wikipedia](https://www.google.com/search?q=Prime+Number+Theorem)
> - [Cryptography - Wikipedia](https://www.google.com/search?q=Cryptography)