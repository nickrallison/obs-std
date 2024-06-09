---
bad_links:
aliases: []
tags: [algorithms]
---
# Bucket Sort

Bucket sort is a distribution sort algorithm that works by distributing the elements of an array into a number of buckets. Each bucket is then sorted individually, either using a different sorting algorithm, or by recursively applying the bucket sort algorithm. It is mainly useful when input is uniformly distributed over a range.

Here's a step-by-step breakdown of how the Bucket Sort algorithm works:

1. **Set up an array of initially empty "buckets".**
   - Suppose you have `n` elements in the array to be sorted and you decide to use `m` buckets. The number of buckets is usually chosen such that it is proportional to the size of the input data.

2. **Scatter**: Go over the original array, putting each object in its bucket.
   - The mapping to buckets is often a function of the characteristics of the data and the range of the data. For example, if the input is uniformly distributed over a range \[a, b), we could divide this range into `n` equal-sized subranges, and then the bucket for a value `v` would be `i` if `v` belongs to subrange `i`.

3. **Sort each non-empty bucket**.**
   - This can be done using a different sorting algorithm. For simplicity, one can use insertion sort, which is efficient for small sizes of input data.

4. **Gather**: Visit the buckets in order and put all elements back into the original array.
   - This is done by concatenating the elements of the buckets in the order of the buckets.

The [[Big-O Notation.md|time complexity]] of Bucket Sort is interesting to note. In the best case, where the input is uniformly distributed, the time complexity $O(n + k)$, where `n` is the number of elements to be sorted and `k` is the number of buckets. In the worst case, where all elements end up in the same bucket, the [[Big-O Notation.md|time complexity ]] is $O(n^2)$, as would need to use a comparison sort like insertion sort on the elements.

>]]
>For more in-depth reading, you can refer to the following resources:
> - [Bucket Sort - Wikipedia](https://www.google.com/search?q=bucket+sort+wikipedia)
> - [Bucket Sort - GeeksforGeeks](https://www.google.com/search?q=bucket+sort+geeksforgeeks)
> - [Bucket Sort - Tutorialspoint](https://www.google.com/search?q=bucket+sort+tutorialspoint)