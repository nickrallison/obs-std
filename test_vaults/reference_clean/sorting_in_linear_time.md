---
bad_links: 
aliases: [Linear Time Sorting]
tags: [Algorithms]
---
# Sorting in Linear Time

Sorting in linear time is a concept in computer science that refers to the ability to sort a list of elements in O(n) time, where n is the number of elements in the list. This is faster than comparison-based sorting algorithms, which have a lower bound of O(n log n) time complexity. Linear time sorting algorithms are not comparison-based and instead rely on specific properties of the elements being sorted.

One of the most common linear time sorting algorithms is . This algorithm assumes that each of the n input elements is an integer in the range 0 to k, for some integer k. Here's a brief overview of how it works:

1. Initialize an array of size k+1 to all zeros. This array will be used to count the occurrence of each integer in the input array.
2. For each element in the input array, increment the corresponding index in the count array.
3. Iterate over the count array, and for each non-zero count, output the corresponding integer the number of times indicated by the count.

The time complexity of Counting Sort is O(n+k), which is linear when k = O(n). However, the space complexity is also O(n+k), which can be prohibitive if k is much larger than n.

Another linear time sorting algorithm is Radix Sort. This algorithm assumes that each of the n input elements has d digits, where each digit can take one of b possible values. The algorithm works by performing a stable sort on each digit, starting from the least significant and moving to the most significant. The stable sort can be done in O(n+b) time using Counting Sort, so the total time complexity of Radix Sort is O(d(n+b)). If d and b are both O(1), then this is linear time.

 is another linear time sorting algorithm that works by dividing the range of input elements into n equal-sized buckets, and then distributing the elements into these buckets. Each bucket is then sorted individually, either using another sorting algorithm or recursively applying the bucket sort. If the input is uniformly distributed, then we can expect the elements to be evenly distributed into the buckets, and the time complexity will be O(n).

Here are some tangentially related items:

- The concept of lower bounds in computational complexity theory. This is the idea that there is a minimum amount of time that any algorithm solving a certain problem must take. For comparison-based sorting algorithms, this lower bound is O(n log n).
- The concept of stable sorting algorithms. A sorting algorithm is stable if it maintains the relative order of equal elements. This is an important property for many applications.
- The concept of in-place sorting algorithms. An in-place sorting algorithm sorts the elements directly in the input array, without needing to use additional space proportional to the size of the input.

> For more information, you can refer to the following resources:
> - [Counting Sort](https://www.google.com/search?q=Counting+Sort)
> - [Radix Sort](https://www.google.com/search?q=Radix+Sort)
> - [Bucket Sort](https://www.google.com/search?q=Bucket+Sort)
> - [Computational Complexity Theory](https://www.google.com/search?q=Computational+Complexity+Theory)
> - [Stable Sorting Algorithms](https://www.google.com/search?q=Stable+Sorting+Algorithms)
> - [In-place Sorting Algorithms](https://www.google.com/search?q=In-place+Sorting+Algorithms)