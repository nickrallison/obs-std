---
bad_links: 
aliases: []
tags: [algorithms]
---
# [[Binary Tree|Binary Tree]] [[Binary Serialization|Serialization]]

[[Binary Tree|Binary tree]] [[Binary Serialization|serialization]] is a process of converting a [[Binary Tree|binary tree]] into a format that can be stored (like file or database) or transmitted across a network and then recreated in the same structure later. This process is important in computer science, especially in areas like distributed systems, where data needs to be transferred across networks. The opposite of [[Binary Serialization|serialization]] is [[Serialization|deserialization,]] which converts the serialized data back into the original [[Binary Tree|binary tree]] structure.

To [[Serialization|serialize]] a tree, you need to recursively [[Serialization|serialize]] the child nodes, sort them in “alphabetical” order and the resulting [[Binary Serialization|serialization]] would be “1” + child1 + child2 + … + childn + “0". The base case of a single node is just “10"
