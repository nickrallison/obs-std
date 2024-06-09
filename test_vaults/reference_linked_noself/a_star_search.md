---
bad_links: 
aliases: [A-Star]
tags: [robotics, graphtheory, algorithms]
title: A-Star Search
date created: Monday, July 24th 2023, 9:12:19 pm
---
# A-Star Search

A-Star Search, also known as A\* Search, is a popular and effective pathfinding algorithm in computer science. It is used to find the shortest possible path from a starting point to a goal point. The algorithm uses a heuristic approach to estimate the cost of moving from one point to another, which helps in determining the most efficient route. This makes it particularly useful in applications such as GPS navigation, game programming and robotics. A-Star Search combines the strengths of [[Dijkstra's Algorithm|Dijkstra’s Algorithm]] (which is good at finding shortest paths) and Greedy Best-First-Search (which is good at quickly exploring areas).

```pseudo
\begin{algorithm}
\caption{A-Star Search Algorithm}
\begin{algorithmic}
  \Procedure{AStar}{$Graph, start, goal$}
	\State Initialize the open set with the start node
	\State Initialize the closed set to be empty
	\State For each node, initialize gScore (cost from start to this node) to be infinity and fScore (gScore plus heuristic cost from this node to goal) to be infinity. Set gScore of start node to 0 and fScore of start node to be the heuristic cost from start to goal.
	
	\While{the open set is not empty}
	  \State $current \gets $ \Call{NodeWithLowestFScore}{$OpenSet$}
	  
	  \If{$current$ is $goal$} 
		  \State Return reconstructed path from start to goal
	  \EndIf
	  
	  \State Remove $current$ from open set and add it to closed set
	  
	  \For{each neighbor of $current$}
	    \If{neighbor is in the closed set} 
		  \State Ignore it
		\Else
		  \State Calculate tentative gScore for neighbor: $gScore[current]$ $+$ distance between current and neighbor
		  
		  \If{neighbor is not in open set} 
		    \State Add neighbor to open set
		  \Elif{tentative $gScore \ge gScore[neighbor]$}
		    \State Ignore it as this is not a better path
		  \EndIf
		  
		  \State If we're here, then this path is the best so far. Record it!
		  \State Update gScore[neighbor] and fScore[neighbor]
		\EndIf
		
	  \EndFor
	
	\EndWhile
	
  \EndProcedure
  
  \Procedure{NodeWithLowestFScore}{$OpenSet$}
    \State Remove and return the node with smallest fScore from OpenSet.
  \EndProcedure
  
  \end{algorithmic}
\end{algorithm}
```