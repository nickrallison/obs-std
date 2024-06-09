---
bad_links: 
aliases: [Elliptic Curve Discrete Logarithm Problem]
tags: [cryptography]
---
# Discrete Logarithm Problem

The Discrete Logarithm Problem (DLP) is a mathematical problem that serves as the basis for several protocols in public-key cryptography. It is considered hard to solve, which is what makes it useful for cryptographic applications.

The problem can be stated as follows: Given a finite cyclic group $G$, a generator $g$ of that group, and an element $h$ in the group, find the integer $x$ such that $g^x = h$. This is easy to compute in the 'forward' direction (given $g$ and $x$, compute $h$), but believed to be difficult in the 'reverse' direction (given $g$ and $h$, compute $x$). This asymmetry is what makes the DLP useful in cryptography.

The difficulty of the DLP is related to the size of the group $G$. In general, the larger the group, the harder the problem. This is why cryptographic applications that use the DLP (like the [[Diffie-Hellman Key Exchange|Diffie-Hellman key exchange]] or the ElGamal encryption scheme) typically operate in large finite fields or [[Elliptic Curves|elliptic curve]] groups.

The DLP is related to several other problems in computational number theory, such as the integer [[Prime Factorization|factorization]] problem. These problems are all believed to be hard, but no formal proof of their hardness exists. This is one of the major open problems in computational complexity theory.

The DLP also has a 'decisional' variant, known as the Decisional Discrete Logarithm Problem (DDLP). The DDLP asks not for the discrete logarithm $x$ itself, but merely whether a given value is the correct logarithm. More formally, given $g$, $h$, and a value $y$, the DDLP asks whether $g^y = h$. The DDLP is believed to be at least as hard as the DLP, and is also used in cryptography.

Here are some additional resources for further reading:

> - ["Discrete Logarithm Problem"](https://www.google.com/search?q=Discrete+Logarithm+Problem) on Google
> - ["Discrete logarithm records"](https://www.google.com/search?q=Discrete+logarithm+records) on Google
> - ["Computational complexity of mathematical operations"](https://www.google.com/search?q=Computational+complexity+of+mathematical+operations) on Google
> - ["Diffie-Hellman key exchange"](https://www.google.com/search?q=Diffie-Hellman+key+exchange) on Google
> - ["ElGamal encryption scheme"](https://www.google.com/search?q=ElGamal+encryption+scheme) on Google
> - ["Decisional Discrete Logarithm Problem"](https://www.google.com/search?q=Decisional+Discrete+Logarithm+Problem) on Google
> - ["Integer factorization problem"](https://www.google.com/search?q=Integer+factorization+problem) on Google
> - ["Computational complexity theory"](https://www.google.com/search?q=Computational+complexity+theory) on Google
