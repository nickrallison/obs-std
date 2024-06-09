---
bad_links: 
aliases: []
tags: [algorithms]
---
# Counting Sort

Counting Sort is a non-comparative integer sorting algorithm that operates by counting the number of objects that have distinct key values. It is often used when the variation in keys is not significantly greater than the number of items. More specifically, it's efficient when the range of input values (keys) is not significantly greater than the number of values to be sorted.

The general idea of Counting Sort is to determine, for each input element, the number of smaller elements and use this [[Information Theory|information]] to place the element in its correct position in the output array.

Here's a step-by-step breakdown of the algorithm:

1. **Counting Phase**: Count the occurrence of each element in the input array. This is usually done by initializing an auxiliary array of size `k` (where `k` is the range of input) with all zeros, and then iterating over the input array, using each value as an index into the auxiliary array and incrementing the corresponding value.

2. **Transformation Phase**: Transform the count array to reflect the actual positions of elements in the output array. This is done by iterating over the count array and updating each element to be the sum of itself and the previous element. This results in each element of the count array now holding the cumulative count of elements.

3. **Placement Phase**: Place each element from the input array in its correct position in the output array. This is done by iterating over the input array from the end, using each value as an index into the count array to find its position in the output array, placing the element at the correct position in the output array, and decrementing the corresponding count.

The [[Big-O Notation|time complexity]] of Counting Sort is $O(n + k)$, where $n$ is the number of elements in the input array and $k$ is the range of input. This makes it very efficient when $k$ is close to $n$. However, it's not suitable for sorting large data sets with non-integer keys or keys that have large ranges, as the auxiliary count array's size is dependent on the key range.

> For more in-depth reading, you might find the following resources helpful:
> - [Counting Sort - Wikipedia](https://www.google.com/search?q=Counting+Sort+Wikipedia)
> - [Counting Sort - GeeksforGeeks](https://www.google.com/search?q=Counting+Sort+GeeksforGeeks)
> - [Counting Sort - Khan Academy](https://www.google.com/search?q=Counting+Sort+Khan+Academy)