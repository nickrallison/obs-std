---
bad_links:
aliases: []
tags: [datastructures]
---
# Red Black Tree Delete

Red-Black Trees are a type of self-balancing binary search tree where each node contains an extra bit for denoting the color of the node, either red or black. A Red-Black Tree satisfies the following properties:

1. Every node is either red or black.
2. The root is black. This rule is sometimes omitted. Since the root can always be changed from red to black, but not necessarily vice versa, this rule has little effect on analysis.
3. All leaves (NIL) are black.
4. If a node is red, then both its children are black.
5. Every path from a given node to any of its descendant NIL nodes contains the same number of black nodes.

The deletion operation in a Red-Black Tree is complex because simply deleting a node may violate the Red-Black properties. Therefore, we need to go through a series of rotations and color changes to ensure the properties are preserved. The deletion operation can be divided into two stages: BST deletion and Fixing Red-Black properties.

**BST Deletion**: This is the standard deletion operation in a binary search tree where the node deleted is replaced by the in-order predecessor (maximum in its left subtree) or the in-order successor (minimum in its right subtree). After this deletion, the Red-Black properties may be violated, and we move to the second stage.

**Fixing Red-Black properties**: If the deleted node is red, the Red-Black properties still hold. However, if the deleted node is black, we have the following cases to consider:

1. **Case 1**: The sibling is red. In this case, we perform a rotation to move the old sibling up, recolor the old parent and the old sibling, and then revisit the case.

2. **Case 2**: The sibling is black and at least one of sibling’s children is red. We perform a rotation so that the red child of the sibling becomes the sibling or we perform rotation to move the red child up at the sibling’s place. This case can be divided into two subcases:
    - **Case 2A**: The red child is an outer grandchild. In this case, a single rotation can be performed.
    - **Case 2B**: The red child is an inner grandchild. In this case, a double rotation is needed.

3. **Case 3**: The sibling is black and both of sibling’s children are black. In this case, we pull the black up which makes the sibling red. If the parent was red, we make it black which is fine. But if the parent was black, we recursively go up.

The time complexity of delete operation in a Red-Black Tree is O(log n) where n is the number of nodes in the tree.

> For more detailed information, you can refer to the following resources:
> - [Red-Black Tree | Set 3 (Delete)](https://www.geeksforgeeks.org/red-black-tree-set-3-delete-2/)
> - [Introduction to Red Black Trees (Includes Deletion as well)](https://www.coursera.org/lecture/data-structures/introduction-to-red-black-trees-TdnnZ)
> - [Deletion from a Red Black Tree](https://www.hackerearth.com/practice/data-structures/trees/red-black-tree/tutorial/)