---
bad_links: 
aliases: []
tags: [datastructures]
---
# Van Emde Boas Trees

Van Emde Boas Trees (vEB Trees) are a type of data structure that support many dynamic set operations in $O(\log \log u)$ time, where $u$ is the size of the universe from which keys are drawn. They were first introduced by Dutch computer scientist Peter van Emde Boas in 1975.

The vEB tree is a tree-based data structure that is used to implement operations like insert, delete, search, minimum, maximum, successor, and predecessor in $O(\log \log u)$ time. The vEB tree is a generalization of a [[Heap Data Structure|Heap]] Data Structure and can be used to speed up [[Dijkstra's Algorithm|Dijkstra's algorithm]].

The vEB tree is a recursive data structure, each vEB tree $T$ contains:

- A minimum and maximum element
- A summary vEB tree $T.summary$
- An array $T.cluster$ of $\sqrt{u}$ vEB trees

The keys in the vEB tree are drawn from a universe $U$ of size $u = 2^k$, where $k$ is an integer. The universe is divided into $\sqrt{u}$ clusters, each of size $\sqrt{u}$.

The vEB tree uses the high (most significant) $\lceil \log_2 \sqrt{u} \rceil$ bits of a key to determine which cluster it belongs to, and the low (least significant) $\lfloor \log_2 \sqrt{u} \rfloor$ bits to determine its position within the cluster.

The summary vEB tree keeps track of which clusters are non-empty, and each cluster vEB tree stores its elements.

Here are the key operations of a vEB tree:

1. **Search**: To search for an element $x$, we first check if $x$ is the minimum or maximum. If not, we determine the cluster $i$ that $x$ belongs to and search for $x$ in $T.cluster[i]$.

2. **Insert**: To insert an element $x$, we first insert $x$ into the appropriate cluster, and then update the summary to indicate that the cluster is non-empty.

3. **Delete**: To delete an element $x$, we remove $x$ from its cluster, and if the cluster becomes empty, we update the summary.

4. **Minimum and Maximum**: The minimum and maximum elements are stored directly in the vEB tree, so these operations take constant time.

5. **Successor and Predecessor**: To find the successor of an element $x$, we first determine the maximum element in $x$'s cluster that is greater than $x$. If such an element exists, it is the successor. If not, we find the next non-empty cluster using the summary, and its minimum element is the successor.

The vEB tree achieves its [[Big-O Notation|time complexity]] by reducing the size of the problem by a square root at each step, rather than by a constant factor as in most other data structures.

> For more detailed information, you can refer to the following resources:
> - [Van Emde Boas Tree | Set 1 (Background and Introduction)](https://www.google.com/search?q=Van+Emde+Boas+Tree+Set+1+Background+and+Introduction)
> - [Van Emde Boas Tree | Set 2 (Insert and Delete)](https://www.google.com/search?q=Van+Emde+Boas+Tree+Set+2+Insert+and+Delete)
> - [Van Emde Boas Tree | Set 3 (Minimum, Maximum, Successor and Predecessor)](https://www.google.com/search?q=Van+Emde+Boas+Tree+Set+3+Minimum+Maximum+Successor+and+Predecessor)
> - [Van Emde Boas Trees](https://www.google.com/search?q=Van+Emde+Boas+Trees+site:en.wikipedia.org) on Wikipedia
> - [Introduction to Algorithms, 3rd Edition](https://www.google.com/search?q=Introduction+to+Algorithms+3rd+Edition) by Thomas H. Cormen, Charles E. Leiserson, Ronald L. Rivest, and Clifford Stein, specifically Chapter 20: The Van Emde Boas Tree.