---
bad_links: 
aliases: []
tags: [algorithms]
---
# Binary Tree

A Binary Tree is a type of data structure that has a maximum of two children for each parent node. The two children are typically referred to as the left child and the right child. The topmost node in the tree is known as the root. 

A Binary Tree is defined recursively, meaning that every node in the tree is the root of a smaller binary tree - the subtree. This recursive property is fundamental to many of the algorithms and operations that are performed on binary trees.

The depth (or level) of a node is the number of edges from the root to the node. The height of a node is the number of edges from the node to the deepest leaf. The height of a tree is a height of the root. 

A Binary Tree is "Full" or "Proper" if every node has either 0 or 2 children. A Binary Tree is "Complete" if all levels are completely filled except possibly for the last level, which is filled from left to right.

The number of nodes $n$ in a full binary tree of height $h$ is $n = 2^{h+1} - 1$. This can be proven by [[Induction Proofs|induction]].

The number of leaf nodes in a full binary tree is $l = 2^h$. This is because each level $i$ of the tree adds $2^i$ nodes, and the last level (the leaves) is level $h$.

Binary Trees are used in many areas of computer science, including operating systems, graphics, database systems, and computer networking. Tree traversal algorithms, such as in-order, pre-order, and post-order, are important methods used to visit all the nodes of a binary tree.

> For more information, you can refer to the following resources:
> - [Binary Trees in Data Structures](https://www.google.com/search?q=Binary+Trees+in+Data+Structures)
> - [Tree Traversal](https://www.google.com/search?q=Tree+Traversal)
> - [Binary Trees in Computer Science](https://www.google.com/search?q=Binary+Trees+in+Computer+Science)