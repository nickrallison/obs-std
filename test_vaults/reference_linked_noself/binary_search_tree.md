---
bad_links:
aliases: []
tags: [datastructures]
---
# Binary Search Tree

A Binary Search Tree (BST) is a node-based [[Binary Tree.md|binary tree]] data structure which has the following properties:

- The left subtree of a node contains only nodes with keys less than the node’s key.
- The right subtree of a node contains only nodes with keys greater than the node’s key.
- Both the left and right subtrees must also be binary search trees.

The above properties of BST provide an ordering among keys so that the operations like search, minimum and maximum can be done fast. If there is no ordering, then we may have to compare every key to search a given key.

## Binary Search Tree Representation

A BST is typically visualized as a binary tree, where each node contains a key, and has two "children" which are also nodes. The left child node contains a key less than its parent node, while the right child node contains a key greater than its parent node.

Here is a simple representation of a BST:

```
    8
   / \
  3   10
 / \    \
1   6    14
   /  \  /
  4    7 13
```

## Operations on BST

There are several fundamental operations that can be executed on a BST in $O(h)$ time complexity, where $h$ is the height of the tree:

1. **Search**: Starting from the root, we traverse the tree. If the current node is null, the key is not present in the tree. Otherwise, if the current node's key equals the target key, we return the current node. If the current node's key is less than the target key, we move to the right child of the current node. Otherwise, we move to the left child.

2. **Insertion**: Similar to search, but whenever we reach a null position, we insert a new node with the given key at that position.

3. **Deletion**: This is more complex. There are three cases to consider:
   - Node to be deleted is a leaf: Simply remove from the tree.
   - Node to be deleted has only one child: Copy the child to the node and delete the child.
   - Node to be deleted has two children: Find inorder successor of the node. Copy contents of the inorder successor to the node and delete the inorder successor. Note that inorder predecessor can also be used.

4. **Traversal**: There are three common ways to traverse a BST:
   - Inorder (Left, Root, Right)
   - Preorder (Root, Left, Right)
   - Postorder (Left, Right, Root)