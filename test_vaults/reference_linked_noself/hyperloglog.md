---
bad_links: 
aliases: []
tags: [probability, algorithms]
---
# HyperLogLog

HyperLogLog is a probabilistic algorithm used for estimating the number of distinct elements in a large data set, often referred to as [[Cardinality|cardinality]]. It was developed to improve on the storage requirements of similar algorithms by dramatically reducing memory usage while still providing a reasonably accurate estimate. This makes it particularly useful in big data applications where memory resources may be limited. HyperLogLog doesn't provide an exact count, but its estimates are generally accurate within a few percentage points.

Similar Algorithms would be the brute force solution of counting every element, or putting the dataset into a [[Hash Map|hash map]]

## Sources

[Youtube Video](https://www.youtube.com/watch?v=lJYufx0bfpw)