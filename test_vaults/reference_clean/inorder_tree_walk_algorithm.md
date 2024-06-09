---
bad_links:
aliases: [inorder tree walk, tree walk]
tags: [algorithms]
---
# Inorder Tree Walk Algorithm

The Inorder Tree Walk is an algorithm used to traverse (visit all nodes of) a binary tree in a specific order: Left Node, Root Node, Right Node. This algorithm is particularly useful when the binary tree is a BST, as it will output the nodes in ascending order.

Here's a pseudocode representation of the Inorder Tree Walk Algorithm:

```python
def inorder_tree_walk(node):
    if node is not None:
        inorder_tree_walk(node.left)
        print(node.key)
        inorder_tree_walk(node.right)
```

In this pseudocode, `node` is the current node being visited, `node.left` is the left child of the current node, and `node.right` is the right child of the current node. The function is recursive, meaning it calls itself to traverse the tree.

The time complexity of the Inorder Tree Walk Algorithm is O(n), where n is the number of nodes in the tree. This is because each node is visited exactly once.

The space complexity is O(h), where h is the height of the tree. This is due to the maximum amount of space needed at any point in time being the height of the tree, which is the maximum number of recursive calls that are in progress at the same time.

Tangentially related items include other types of tree traversal algorithms, such as:

- Preorder Tree Walk: Root Node, Left Node, Right Node
- Postorder Tree Walk: Left Node, Right Node, Root Node

These algorithms are similar to the Inorder Tree Walk, but visit the nodes in a different order.

> For more information, you can refer to the [Binary Tree Traversal](https://www.google.com/search?q=binary+tree+traversal) and [Inorder Tree Walk](https://www.google.com/search?q=inorder+tree+walk) topics on Google.