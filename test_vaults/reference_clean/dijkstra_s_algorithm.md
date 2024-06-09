---
bad_links: 
aliases: [Dijkstras Algorithm]
tags: [graphtheory, algorithms]
title: "Dijkstra's Algorithm"
date created: Monday, July 24th 2023, 9:07:22 pm
---
# Dijkstra's Algorithm

Dijkstra's Algorithm is a popular method used in computing and graph theory for finding the shortest path between two nodes in a graph. Named after its creator, Dutch computer scientist Edsger Dijkstra, the algorithm works by visiting vertices in the graph starting from the object's starting point and spreading out slowly like a wave until it reaches the destination. Dijkstra’s Algorithm maintains a set of unvisited nodes and calculates a tentative distance from one node to another, updating these distances if it finds a shorter path. This algorithm is widely used in network routing protocols, notably IS-IS (Intermediate System to Intermediate System) and Open Shortest Path First (OSPF).

```pseudo
\begin{algorithm}
\caption{Dijkstra's Algorithm}
\begin{algorithmic}
  \Procedure{Dijkstra}{$Graph, source$}
	\State Initialize distance array, $dist$, with infinity for all vertices except source (set to 0)
	\State Initialize a priority queue, $Q$, and insert the source node with distance 0
	\While{$Q$ is not empty}
	  \State $u \gets $ \Call{ExtractMin}{$Q$}
	  \For{each neighbor $v$ of $u$ in $Graph$}
	    \If{$dist[u] + weight(u, v) \leq dist[v]$} 
		  \State Update $dist[v]$ to be the new smaller distance
		  \State Update priority of $v$ in $Q$
		\EndIf
	  \EndFor
	\EndWhile
  \EndProcedure
  
  \Procedure{ExtractMin}{$Q$}
    \State Remove and return the node with the smallest distance from $Q$
  \EndProcedure
  
  \end{algorithmic}
\end{algorithm}
```
