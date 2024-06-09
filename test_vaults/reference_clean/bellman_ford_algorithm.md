---
bad_links: 
aliases: [Bellman-Ford, bellmanford]
tags: [graphtheory, algorithms]
---
# Bellman-Ford Algorithm

The Bellman-Ford Algorithm is a graph-based algorithm used in computer science for finding the shortest path between a starting node and all other nodes in a weighted graph. This algorithm is capable of handling graphs in which some of the edge weights are negative. However, it cannot process graphs with negative cycles, where the overall sum of the cycle edges is negative. The algorithm works by progressively reducing an estimate of the shortest path from the starting vertex to each point until it reaches the actual shortest path. It is named after Richard Bellman and Lester Ford Jr., who first published this method.

```pseudo
\begin{algorithm}
\caption{Bellman-Ford Algorithm}
\begin{algorithmic}
  \Procedure{BellmanFord}{$Graph, source$}
    \State Initialize distance array, $dist$, with infinity for all vertices except source (set to 0)
    \State Initialize predecessor array, $prev$, with null for all vertices
    \For{$i$ from 1 to size of $Graph.vertices$ - 1} 
      \For{each edge $(u, v)$ in $Graph.edges$} 
        \If{$dist[u] + weight(u, v) < dist[v]$} 
          \State Update $dist[v]$ to be the new smaller distance
          \State Update $prev[v]$ to be $u$
        \EndIf
      \EndFor
    \EndFor

    \For{each edge $(u, v)$ in $Graph.edges$} 
      \If{$dist[u] + weight(u, v) < dist[v]$} 
        \State Return "Graph contains a negative-weight cycle"
      \EndIf
    \EndFor

    \State Return the distance and predecessor arrays
  \EndProcedure
  
\end{algorithmic}
\end{algorithm}
```