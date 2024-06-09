---
bad_links: 
aliases: []
tags: [algorithms]
---
# AVL Tree

An AVL (Adelson-Velsky and Landis) tree is a self-balancing binary search tree (BST) where the difference between heights of left and right subtrees cannot be more than one for all nodes. It was invented by two Soviet mathematicians, G.M. Adelson-Velsky and E.M. Landis, in 1962.

The balance factor of a node in an AVL tree is the difference between the height of the left subtree and the height of the right subtree. It can be calculated as follows:

$$
\text{Balance Factor} = \text{Height of Left Subtree} - \text{Height of Right Subtree}
$$

The balance factor of any node in an AVL tree is always -1, 0, or 1. If the balance factor becomes less than -1 or greater than 1 then the tree is balanced using some rotation techniques.

There are four types of rotations to balance a tree in AVL:

1. **Left-Left Rotation (LL Rotation)**: This is a single rotation for left-heavy subtrees. This rotation is performed when the left child is left-heavy.
2. **Right-Right Rotation (RR Rotation)**: This is a single rotation for right-heavy subtrees. This rotation is performed when the right child is right-heavy.
3. **Left-Right Rotation (LR Rotation)**: This is a double rotation for left-heavy subtrees. This rotation is performed when the left child is right-heavy.
4. **Right-Left Rotation (RL Rotation)**: This is a double rotation for right-heavy subtrees. This rotation is performed when the right child is left-heavy.

The AVL tree maintains its height balanced after performing insertions or deletions. The height of an AVL tree is always logarithmic in the number of nodes in the tree. The height of an AVL tree with N nodes is O(Log N).

The time complexity for searching, insertion, and deletion in an AVL tree on average and worst-case scenarios is O(Log N), where N is the number of nodes.

> For more detailed information, you can refer to the following resources:
> - [AVL Tree Introduction](https://www.google.com/search?q=AVL+Tree+Introduction)
> - [AVL Tree Rotations](https://www.google.com/search?q=AVL+Tree+Rotations)
> - [AVL Tree Insertion](https://www.google.com/search?q=AVL+Tree+Insertion)
> - [AVL Tree Deletion](https://www.google.com/search?q=AVL+Tree+Deletion)
> - [AVL Tree Time Complexity](https://www.google.com/search?q=AVL+Tree+Time+Complexity)