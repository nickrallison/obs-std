---
bad_links: 
aliases: []
tags: [algorithms]
---
# Amortized Constant Time

Amortized constant time in algorithmic theory refers to an algorithm's [[Expected Value|average]] running time taken over a sequence of operations. It might not perform every operation in constant time, however, when averaged over a large number of operations, the [[Expected Value|average]] operation time becomes constant. This concept is useful in cases where an operation can be expensive at times but not always. The cost of the expensive operation is 'amortized' over all the operations, resulting in a constant [[Expected Value|average]] time.

An example of this is the [[Push Operation|push operation]]. When an object is pushed, if it will not fit in the size of the array, the length of the array is doubled. But when averaged out over all of the pushes, it [[Expected Value|averages]] to constant time.