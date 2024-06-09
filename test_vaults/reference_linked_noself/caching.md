---
bad_links: 
aliases: [Cache, memoization]
tags: [computerarchitecture, computerscience]
---
# Caching

Caching is a technique used in computer systems to improve performance by storing frequently accessed data in a faster and closer location to the processor. The idea behind caching is to reduce the time it takes to access data by keeping a copy of it in a faster memory hierarchy.

In computer systems, the memory hierarchy consists of multiple levels, with each level having different access times and capacities. The levels typically include registers, cache, main memory, and secondary storage (such as hard drives). The closer a memory level is to the processor, the faster it can be accessed but with a smaller capacity.

The most commonly used cache organization is the **CPU cache**, which is a small, fast memory located on the processor chip. The CPU cache is divided into multiple levels, such as L1, L2, and L3 caches, with each level having different sizes and access times. The cache operates on the principle of locality, which states that programs tend to access data and instructions that are spatially or temporally close to each other.

The **cache hit** occurs when the requested data is found in the cache, resulting in a faster access time. On the other hand, a **cache miss** occurs when the requested data is not found in the cache, requiring a slower access to the next level of the memory hierarchy.

To determine whether a requested data item is present in the cache, a **cache index** is used. The cache index is calculated based on the [[Pointer|memory address]] of the requested data item. The cache index is used to locate the corresponding cache line, which contains multiple data items.

The cache uses a **cache replacement policy** to decide which data items to evict from the cache when it is full. One commonly used replacement policy is the **Least Recently Used (LRU)** policy, which evicts the data item that has not been accessed for the longest time.

The performance of a cache can be evaluated using various metrics, such as **hit rate** and **miss rate**. The hit rate is the percentage of cache accesses that result in a cache hit, while the miss rate is the percentage of cache accesses that result in a cache miss. The hit rate and miss rate can be calculated using the following formulas:

$$
\text{{Hit Rate}} = \frac{{\text{{Number of Cache Hits}}}}{{\text{{Number of Cache Accesses}}}}
$$

$$\text{{Miss Rate}} = \frac{{\text{{Number of Cache Misses}}}}{{\text{{Number of Cache Accesses}}}}$$

The **[[Expected Value|average]] memory access time (AMAT)** is another important metric used to evaluate cache performance. It takes into account the access time of each level of the memory hierarchy and the hit rate of the cache. The AMAT can be calculated using the following formula:

$$\text{{AMAT}} = \text{{Cache Access Time}} + \text{{Miss Rate}} \times \text{{Miss Penalty}}$$

where the miss penalty is the additional time required to access the next level of the memory hierarchy in case of a cache miss.

Caching is a fundamental concept in computer systems and plays a crucial role in improving performance. It is used in various domains, including processors, databases, web servers, and file systems, to name a few.

For more [[Information Theory|information]] on caching, you can refer to the following resources:

- [Cache (computing) - Wikipedia](https://en.wikipedia.org/wiki/Cache_(computing))
- [Computer Organization and Design: The Hardware/Software Interface](https://www.amazon.com/Computer-Organization-Design-Hardware-Software/dp/0123747503)

