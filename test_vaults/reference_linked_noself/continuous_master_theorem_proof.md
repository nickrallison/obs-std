---
bad_links: 
aliases: []
tags: [algorithms]
---
# [[Continuity|Continuous]] [[Master Theorem|Master Theorem]] Proof
## [[Introduction to Algorithms 4e.pdf]] - Pages 129-136 Summary

The following composition explores a simplified version of the [[Continuity|continuous]] master theorem—a variant of the master theorem—for understanding algorithmic efficiency. The proof process bypasses complexities like floors and ceilings, and highlights the necessity of a robust mathematical foundation. The theorem revolves around two essential lemmas, which respectively offer a simplified master recurrence and lay down asymptotic [[Limits|limits]] for the evaluation of any given summation. In addition, the document enlarges upon the theorem by dissecting it into its constituent three possibilities that are determined by the dispersion of costs throughout the various levels of the recursion tree. 

The [[Continuity|continuous]] [[Master Theorem|master theorem]] establishes a framework for deducing the asymptotic properties of a given algorithmic recurrence, T(n). There are three possibilities: If a constant greater than zero exists that verifies f(n) = $O(n^{log_b a})$, then T(n) = $\Theta(n^log _b a)$. A second situation arises if a constant k≥0 exists to validate $f(n) = \Theta(n^{\text{log}_b a} lg^k n)$, then T(n) would equal $\Theta(n ^{\text{log}_b a} lg^k(C_1 n)).$ Finally, when there is a constant greater than zero that satisfies $f(n) = \Omega(n^{\text{log}_b a})$ and f(n) also adheres to the regularity condition af(n/b) හcf(n) for some constant c < 1 and all adequately large n, it can be interpreted that T(n) = ‚(f(n)).

The theorem's proof implies the application of Lemma 4.3 to establish [[Limits|limits]] for the summation provided in Lemma 4.2. Two key auxiliary functions, T0(n) and f0(n), are clarified, all of which comply with the specifications of Lemma 4.2. Upon solving the recurrence for T0(n) by employing Lemma 4.2, it's determined that T0(n) = ‚(n log b a) + the confined summation. Thereafter, the function f0(n) is scrutinized under each hypothetical scenario that the theorem formulates. Outsized contributions to the understanding of computational complexity theory are made through this theorem, with its three cases corresponding to various growth rates signifying a pivotal role in advancing critical interpretations of algorithmic performance.