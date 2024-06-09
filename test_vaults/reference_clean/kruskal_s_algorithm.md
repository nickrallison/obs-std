---
bad_links: 
aliases: [Kruskal]
tags: [algorithms]
---
# Kruskal’s Algorithm

Kruskal’s Algorithm is a popular minimum spanning tree algorithm which is used to determine the minimum cost spanning tree in a connected, undirected graph. It works by sorting all the edges from the lowest weight to the highest. It then adds the smallest edges that connect any two trees in the forest, creating a minimum spanning tree, while ensuring that a cycle is not formed. This process continues until the spanning tree includes all the vertices. It is a greedy algorithm in the sense that the next step is the most beneficial one. This algorithm is named after the mathematician Joseph Kruskal.

```pseudo
\begin{algorithm}
\caption{Kruskals Algorithm}
\begin{algorithmic}
  \Procedure{Kruskal}{$G$} \Comment{$G$ is the input graph}
	\State $A \gets$ empty set \Comment{$A$ will ultimately contain the edges of the minimum spanning tree}
	\ForAll{$v \in G.V$} 
	  \State MAKE-SET($v$) 
	\EndFor
	\State Sort the edges of $G.E$ into non-decreasing order by weight $w$
	\ForAll{$(u, v) \in G.E$, taken in non-decreasing order by weight}
	  \If{FIND-SET($u$) $\neq$ FIND-SET($v$)}
		\State Add $(u, v)$ to set $A$
		\State UNION($u, v$)
	  \EndIf
	\EndFor
  \EndProcedure
  \Procedure{MAKE-SET}{$x$} 
    \State create a new set whose only member (and thus representative) is $x$
  \EndProcedure
  \Procedure{FIND-SET}{$x$}
    \State return the representative of the (unique) set containing $x$
  \EndProcedure
  \Procedure{UNION}{$x, y$}
    \State merge the two sets that contain $x$ and $y$, respectively, into a new set that is the union of these two sets.
  \EndProcedure
  \end{algorithmic}
\end{algorithm}
```