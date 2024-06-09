---
bad_links: 
aliases: []
tags: [algorithms]
---
# Simplex Method

The Simplex method is an algorithm used for solving linear programming problems. It was developed by George Dantzig in 1947. This method is designed to solve problems that can be represented by a specific type of mathematical model, one that aims to maximize or minimize a linear function subject to a set of linear inequality or equality constraints. The Simplex method works iteratively by moving along the edges of the feasible region (defined by the constraints) towards the optimal solution.

```pseudo
\begin{algorithm}
\caption{Simplex Method}
\begin{algorithmic}
  \Procedure{Simplex}{$A, b, c$}
	\State Initialize a basic feasible solution, $x$, with all nonbasic variables set to 0
	\State Compute the initial objective function value using $c$ and $x$
	\While{there exists a nonbasic variable with positive reduced cost}
	  \State Choose a nonbasic variable, $j$, with positive reduced cost
	  \State Compute the maximum possible increase in $x_j$ without violating feasibility (minimum ratio test)
	  \If{there is no limit on the increase of $x_j$} 
		  \State The problem is unbounded and return infinity
		\Else
		  \State Increase $x_j$ as much as possible and update the basic feasible solution
		  \State Update the objective function value using the new basic feasible solution
		\EndIf
	\EndWhile
    \State Return the optimal solution and its objective function value
  \EndProcedure  
  \end{algorithmic}
\end{algorithm}
```

