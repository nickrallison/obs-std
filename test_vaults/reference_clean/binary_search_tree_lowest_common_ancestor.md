---
bad_links:
aliases: [Lowest Common Ancestor, LCA]
tags: [algorithms]
---
# Binary Search Tree Lowest Common Ancestor

One of the method to find the Lowest Common Ancestor (LCA) of two nodes in a Binary Search Tree (BST) involves the following steps:

1. Start from the root of the tree, since the root node can always be a common ancestor to every other node.
2. Check if both the input nodes fall in left subtree or right subtree based on the values. If they do, proceed further into that subtree.
   - If both input nodes are greater than root, then the LCA must be in the right subtree. So, we move to the right.
   - If both inputs are less than root, LCA must be in the left subtree. So, we move to the left.
3. If the input nodes are in different subtrees of root, then root is the LCA.
4. For the case where one of the input nodes itself is a root or any node during our traversal, that node is the LCA because a node is considered as the descendant of itself.
5. Repeat the process until you find the LCA.

The key understanding is when the input nodes split into different subtrees, the node where the split happens is the LCA.

The time complexity complexity of this problem is O(log n), since we only visit one node per level in the tree (assuming that tree is reasonably balanced). The tree height is typically log n levels. The space complexity is O(1), no additional data structures are required.

## Sources
<https://www.youtube.com/watch?v=gs2LMfuOR9k>