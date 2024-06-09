---
bad_links: 
aliases: []
tags: [algorithms, graphtheory]
title: Tree Data Structure
date created: Monday, July 24th 2023, 5:32:05 pm
---
# Tree Data Structure

A tree data structure is a non-linear hierarchical data structure that consists of nodes connected by edges. It simulates a tree structure with a set of linked nodes. Each node contains a value or data, and it can have a child node or multiple child nodes. The topmost node is called the root, and the nodes directly connected to another node are its children. The connecting line between two nodes is called an edge.

Tree data structures are used in various applications such as in databases for faster retrieval of data, in routers for storing routing tables, in operating systems for managing files in directories, etc. They are also used in algorithms and artificial intelligence for decision-making processes.

The main types of trees include binary trees (each node has at most two children), binary search trees (left child has lesser value and right child has greater value than the parent), AVL trees (self-balancing binary search tree), B-trees (used in databases and file systems), etc.

```tikz
\begin{document}
\begin{tikzpicture}[
    level distance = 1.5cm,
    level 1/.style={sibling distance=3cm},
    level 2/.style={sibling distance=1.5cm}]
    
\node {Root}
    child {node {Child 1}
        child {node {GC 1}}
        child {node {GC 2}}
    }
    child {node {Child 2}
        child {node {GC 3}}
        child {node {GC 4}}
    };

\end{tikzpicture}
\end{document}
```
