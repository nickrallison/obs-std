---
bad_links: 
aliases: [Rivest-Shamir-Adleman]
tags: [cryptography, numbertheory]
---
# RSA Encryption

RSA (Rivest-Shamir-Adleman) is one of the first public-key cryptosystems and is widely used for secure data transmission. The acronym RSA comes from the surnames of Ron Rivest, Adi Shamir, and Leonard Adleman, who first publicly described the algorithm in 1977.

The RSA algorithm involves four steps: key generation, key distribution, encryption, and decryption.

**Key Generation**  
The key generation in RSA involves the following steps:

1. Choose two distinct prime numbers $p$ and $q$. These values are kept secret.
2. Compute $n = pq$. $n$ is used as the modulus for both the public and private keys.
3. Compute the [[Euler's Totient Function|totient function]] $\phi(n) = (p-1)(q-1)$.
4. Choose an integer $e$ such that $1 < e < \phi(n)$ and $gcd(\phi(n), e) = 1$; i.e., $e$ and $\phi(n)$ are [[Coprimality|coprime]]. $e$ is released as the public key exponent.
5. Determine $d$ as $d \equiv e^{-1} \mod \phi(n)$; i.e., $d$ is the modular multiplicative inverse of $e$ modulo $\phi(n)$. $d$ is kept as the private key exponent.

The public key consists of the modulus $n$ and the public (or encryption) exponent $e$. The private key consists of the modulus $n$ and the private (or decryption) exponent $d$, which must be kept secret.

**Key Distribution**  
The public key is openly distributed, while the private key is kept secret.

**Encryption**  
The plaintext message $M$ is converted to an integer $m$, such that $0 \leq m < n$. The ciphertext $c$ is computed as:

$$
c \equiv m^e \mod n
$$

**Decryption**  
The plaintext can be recovered from the ciphertext $c$ using the private key exponent $d$ as follows:

$$
m \equiv c^d \mod n
$$

**Proof of [[Total Correctness|Correctness]]**  
The [[Total Correctness|correctness]] of RSA is a consequence of the following theorem in number theory:

If $n = pq$ for primes $p$ and $q$, and if $x$ is [[Coprimality|relatively prime]] to $n$, then:

$$
x^{\phi(n)+1} \equiv x \mod n
$$

Given the encryption and decryption processes:

$$
c \equiv m^e \mod n
$$

and

$$
m \equiv c^d \mod n
$$

We can substitute $c$ into the decryption process:

$$
m \equiv (m^e)^d \mod n
$$

which simplifies to:

$$
m \equiv m^{ed} \mod n
$$

Since $ed \equiv 1 \mod \phi(n)$, we can say that $ed = k\phi(n) + 1$ for some integer $k$. Substituting this into the equation gives:

$$
m \equiv m^{k\phi(n) + 1} \mod n
$$

which can be rewritten as:

$$
m \equiv (m^{\phi(n)})^k \cdot m \mod n
$$

By [[Euler’s Theorem|Euler's theorem]], if $gcd(m,n) = 1$, then $m^{\phi(n)} \equiv 1 \mod n$. Therefore, the equation simplifies to:

$$
m \equiv 1^k \cdot m \mod n = m \mod n
$$

This shows that the decrypted message is equivalent to the original plaintext message, proving the [[Total Correctness|correctness]] of RSA.

> For more information, you can refer to the original paper titled ["A Method for Obtaining Digital Signatures and Public-Key Cryptosystems"](https://www.google.com/search?q=A+Method+for+Obtaining+Digital+Signatures+and+Public-Key+Cryptosystems) by Rivest, Shamir, and Adleman. You may also find the [Wikipedia page on RSA](https://www.google.com/search?q=site:wikipedia.org+RSA+%28cryptosystem%29) helpful for a broad overview and additional details.