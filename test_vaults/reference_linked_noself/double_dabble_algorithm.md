---
bad_links: 
aliases:
  - double dabble
tags:
  - computerarchitecture
  - algorithms
---
# Double Dabble Algorithm

The Double Dabble Algorithm is a binary-to-decimal conversion method used in digital computing. It works by shifting binary digits from the source binary number to the destination [[binary_coded_decimal.md|BCD]] ([[Binary Coded Decimal|Binary-coded decimal]]) number. This algorithm treats individual digit calculations as separate [[Binary Operation|binary operations]], which are then combined to form the final decimal output. The "double" refers to the doubling of the [[binary_coded_decimal.md|BCD]] number during each shift, while "dabble" refers to the adjustment made when a digit reaches or exceeds 5. This algorithm is often used in computer systems due to its efficiency and simplicity in implementation.

```pseudo
\begin{algorithm}
\caption{Double Dabble Algorithm}
\begin{algorithmic}
  \Procedure{DoubleDabble}{$n, BCD$}
	\State Initialize shift counter, $S$, to 0
	\State Initialize [[Binary Coded Decimal.md|Binary Coded Decimal]] (BCD) array with all zeros
	\While{$S < n$}
	  \For{$i$ from size of BCD - 1 down to 0}
	    \If{BCD[$i$] > 4}
		  \State Add 3 to BCD[$i$]
		\EndIf
	  \EndFor
	  \State Shift the BCD array one bit to the left, inserting a zero at the rightmost bit
	  \State Increment $S$
	\EndWhile
  \EndProcedure

  \end{algorithmic}
\end{algorithm}
```
In this pseudocode, $n$ represents the number of bits in the binary number to be converted and $BCD$ is an array representing the [[Binary Coded Decimal|Binary Coded Decimal]] result. The algorithm works by shifting bits from the binary number into the [[binary_coded_decimal.md|BCD]] array and adding three to any digit in the [[binary_coded_decimal.md|BCD]] array that exceeds four after a shift. This continues until all bits from the binary number have been shifted into the [[binary_coded_decimal.md|BCD]] array.