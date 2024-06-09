---
bad_links: 
aliases: []
tags: [cryptography]
---
# Elliptic Curve Cryptography

Elliptic Curve Cryptography (ECC) is a type of public key cryptography that relies on the mathematics of elliptic curves. It was introduced in the mid-1980s by Neal Koblitz and Victor Miller. ECC provides the same level of security as traditional public key methods like RSA but with shorter keys, making it more efficient.

An elliptic curve is defined by the equation:

$$
y^2 = x^3 + ax + b
$$

where $4a^3 + 27b^2 \neq 0$ (to avoid singularities). The set of points $(x, y)$ that satisfy this equation, together with a special point at infinity, form an elliptic curve.

In ECC, we consider the group of points on the elliptic curve under an operation called "point addition". Given two points $P$ and $Q$ on the curve, the sum $P + Q$ is defined geometrically: draw the line through $P$ and $Q$ (or the tangent at $P$ if $P = Q$), and it will intersect the curve in exactly one more point. Reflect that point in the x-axis, and that's $P + Q$.

The key insight for ECC is that this operation is commutative ($P + Q = Q + P$), associative ($(P + Q) + R = P + (Q + R)$), and has an identity element (the point at infinity). This makes the set of points on the curve into a mathematical group, and it's the hard problem of computing "point multiples" in this group that ECC security is based on.

The hard problem is known as the Elliptic Curve Discrete Logarithm Problem (ECDLP). Given points $P$ and $Q$, and knowing that $Q = nP$ for some integer $n$, it's computationally infeasible to find $n$. This is the "one-way function" that ECC is based on.

ECC is used in a variety of cryptographic protocols, including key exchange (ECDH), digital signatures (ECDSA), and encryption (ECIES). It's also the basis for Bitcoin's cryptography.

> For more in-depth information, you can refer to the following resources:
> - [Elliptic Curve Cryptography: a gentle introduction](https://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/)
> - [A (Relatively Easy To Understand) Primer on Elliptic Curve Cryptography](https://arstechnica.com/information-technology/2013/10/a-relatively-easy-to-understand-primer-on-elliptic-curve-cryptography/)
> - [Elliptic Curve Cryptography (ECC) Explained](https://www.youtube.com/watch?v=dCvB-mhkT0w) (YouTube video)
> - [Elliptic Curve Cryptography](https://en.wikipedia.org/wiki/Elliptic-curve_cryptography) (Wikipedia article)