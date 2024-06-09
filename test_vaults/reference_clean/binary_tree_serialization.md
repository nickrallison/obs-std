---
bad_links: 
aliases: []
tags: [algorithms]
---
# Binary Tree Serialization

Binary tree serialization is a process of converting a binary tree into a format that can be stored (like file or database) or transmitted across a network and then recreated in the same structure later. This process is important in computer science, especially in areas like distributed systems, where data needs to be transferred across networks. The opposite of serialization is deserialization, which converts the serialized data back into the original binary tree structure.

To serialize a tree, you need to recursively serialize the child nodes, sort them in “alphabetical” order and the resulting serialization would be “1” + child1 + child2 + … + childn + “0". The base case of a single node is just “10"
