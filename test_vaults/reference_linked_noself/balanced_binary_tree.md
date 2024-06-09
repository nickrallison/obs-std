---
bad_links: 
aliases: []
tags: [algorithms, datastructures]
---
# Balanced [[Binary Tree|Binary Tree]]

A **Balanced [[Binary Tree|Binary Tree]]** is a specific type of [[Binary Tree|binary tree]] where the difference between the heights of the left and right subtrees of any node in the tree is not more than one. This property ensures that the tree remains approximately balanced, leading to an optimal search time of $O(\log n)$, where $n$ is the number of nodes in the tree.

The height of a node in a [[Binary Tree|binary tree]] is defined as the number of edges on the longest path from the node to a leaf. The height of a leaf node is 0. The height of a [[Binary Tree|binary tree]] is the height of its root node.

The balance factor of a node in a [[Binary Tree|binary tree]] is the difference between the height of its left subtree and the height of its right subtree. In a balanced [[Binary Tree|binary tree]], the balance factor of every node is -1, 0, or 1.

There are several types of balanced [[Binary Tree|binary trees]], including [[AVL Tree|AVL trees]], [[Red Black Trees|Red-Black trees]], and B-trees, each with their own specific balancing rules and use cases.

**[[AVL Tree|AVL Trees]]**: Named after their inventors Adelson-Velsky and Landis, [[AVL Tree|AVL trees]] are a type of [[Binary Search|binary search]] tree where the difference between the heights of the left and right subtrees (the balance factor) of every node is -1, 0, or 1. If at any time they differ by more than one, rebalancing is done to restore this property. Rebalancing is done through rotations, including left rotation, right rotation, left-right rotation, and right-left rotation.

**[[Red Black Trees|Red Black Trees]]**: [[Red Black Trees|Red-Black trees]] are another type of balanced [[Binary Search|binary search]] tree. They maintain balance by coloring each node red or black and ensuring that the tree satisfies the following red-black properties:
1. Every node is either red or black.
2. The root is black.
3. All leaves (null or NIL) are black.
4. If a node is red, then both its children are black.
5. Every path from a node to its descendant leaves contains the same number of black nodes.

**B-Trees**: B-trees are balanced search trees designed for systems with large amounts of data and are widely used in databases and file systems. A B-tree of order $m$ is a tree which satisfies the following properties:
1. Every node has at most $m$ children.
2. Every non-leaf node (except root) has at least $\lceil m/2 \rceil$ child nodes.
3. The root has at least two children if it is not a leaf node.
4. A non-leaf node with $k$ children contains $k-1$ keys.
5. All the leaves appear in the same level, and carry no information.

Here are some additional resources for further reading:

> - [Balanced Binary Trees](https://www.google.com/search?q=Balanced+Binary+Trees)
> - [AVL Trees](https://www.google.com/search?q=AVL+Trees)
> - [Red-Black Trees](https://www.google.com/search?q=Red-Black+Trees)
> - [B-Trees](https://www.google.com/search?q=B-Trees)
> - [Tree Rotations](https://www.google.com/search?q=Tree+Rotations)
> - [Binary Tree Balance Factor](https://www.google.com/search?q=Binary+Tree+Balance+Factor)