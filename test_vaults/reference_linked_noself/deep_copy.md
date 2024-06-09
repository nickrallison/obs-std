---
bad_links: 
aliases: []
tags: [coding]
---
# Deep Copy

Deep copy is a process in computing where the copying of data structure is done along with the values of its elements, including all [[Pointer|pointers]], references and data. This means that any changes made to the copied object or the original object will not affect each other as they are completely separate. It's different from a [[Shallow Copy|shallow copy]] where only the structure is copied, not the elements, making the copied and original object dependent on each other. Deep copy is useful in programming when you want to duplicate complex data structures and ensure that the original and copied objects do not interfere with each other.

This is a non-trivial operation, an example of this in C would be the memcpy function to copy an array vs. creating a new [[Pointer|pointer]] to the array. 
