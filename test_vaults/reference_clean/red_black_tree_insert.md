---
bad_links:
aliases: []
tags: [datastructures]
---
# Red Black Tree Insert

A Red-Black Tree is a type of self-balancing binary search tree where each node has an extra attribute: color, which can be either red or black. The tree follows these properties:

1. Every node is either red or black.
2. The root is black.
3. All leaves (NIL) are black.
4. If a node is red, then both its children are black.
5. Every path from a node to its descendant NIL nodes contains the same number of black nodes.

The insertion operation in a Red-Black Tree involves two steps: BST ([[Binary Search Tree]]) insertion and then ensuring the Red-Black Tree properties are maintained.

**BST Insertion**: This is the first step where we follow the standard BST insertion procedure. A new node is always inserted as red.

**Fixing Red-Black Tree Properties**: After the BST insertion, the properties of the Red-Black Tree may be violated, and we need to fix them. There are two types of violations that can occur:

1. A red node has a red parent.
2. The root is not black.

The process of fixing these violations is as follows:

1. If the parent of the new node is black, then there is no violation and we are done.
2. If the parent is red, then we have a violation of the Red-Black Tree properties. We handle this case based on the color of the uncle node (the sibling of the parent node):
   - If the uncle is red, we recolor the parent and uncle as black and the grandparent as red, then repeat the process for the grandparent.
   - If the uncle is black or NIL, we perform rotations and recoloring. The type of rotation (left or right) depends on whether the new node is a left or right child and whether its parent is a left or right child.

The time complexity of Red-Black Tree insertion is $O(log n)$, where $n$ is the number of nodes in the tree. This is because the tree remains balanced after every insertion, and the height of the tree is logarithmic in the number of nodes.

> For a more detailed explanation and visualization of Red-Black Tree insertion, you can refer to this [tutorial](https://www.geeksforgeeks.org/red-black-tree-set-2-insert/). For a deeper understanding of the topic, you might want to read the original paper by Rudolf Bayer, "Symmetric binary B-trees: Data structure and maintenance algorithms" which introduced the concept of Red-Black Trees. Here is a [Google search](https://www.google.com/search?q=Symmetric+binary+B-trees%3A+Data+structure+and+maintenance+algorithms) for the paper.