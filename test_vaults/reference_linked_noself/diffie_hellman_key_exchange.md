---
bad_links: 
aliases: [diffie hellman]
tags: [numbertheory, cryptography]
---
# Diffie-Hellman Key Exchange

 Diffie-Hellman Key Exchange is a method of securely exchanging cryptographic keys over a public channel. It was one of the first public-key protocols and was invented by Whitfield Diffie and Martin Hellman in 1976. The protocol is based on the mathematical problem of calculating discrete logarithms in a finite field, which is considered computationally difficult, providing the basis for the security of the protocol.

The protocol allows two parties, Alice and Bob, to each generate a public-private key pair and then exchange their public keys. After the exchange, both parties can compute a shared secret, which can be used as a key in subsequent [[Symmetric Encryption|symmetric encryption]] of messages.

Here's a simplified version of the protocol:

1. Alice and Bob agree on a large prime number, $p$, and a base, $g$, in advance. Both of these numbers are public and can be known by everyone.
2. Alice chooses a secret integer, $a$, and sends Bob $A = g^a \mod p$.
3. Bob chooses a secret integer, $b$, and sends Alice $B = g^b \mod p$.
4. Alice computes $s = B^a \mod p$.
5. Bob computes $s = A^b \mod p$.
6. Both Alice and Bob are now in possession of the same secret, $s$.

The security of the protocol relies on the fact that while it's relatively easy (computationally) to calculate powers modulo a prime, it's very difficult to do the reverse: given $p$, $g$, and $g^a \mod p$, it's computationally infeasible to determine $a$. This is known as the [[Discrete Logarithm Problem|Discrete Logarithm Problem]].

The Diffie-Hellman protocol is susceptible to a man-in-the-middle attack. If a third party, Eve, is able to intercept and replace the messages that Alice and Bob send each other, she can establish two distinct shared keys with Alice and Bob, read their messages, and relay them, all without Alice or Bob knowing. This is why Diffie-Hellman is often used in combination with other protocols that can authenticate the parties to each other.

> For more information, you can refer to the original paper by Diffie and Hellman, ["New Directions in Cryptography"](https://scholar.google.com/scholar?hl=en&as_sdt=0%2C5&q=New+Directions+in+Cryptography&btnG=), or a more accessible explanation in the book ["Cryptography Made Simple"](https://www.google.com/search?q=Cryptography+Made+Simple). For a deeper dive into the mathematics, you might find the book ["A Course in Number Theory and Cryptography"](https://www.google.com/search?q=A+Course+in+Number+Theory+and+Cryptography) useful.