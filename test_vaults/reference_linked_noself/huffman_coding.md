---
bad_links: 
aliases: [Huffman code]
tags: [theoreticalcompsci, signalprocessing]
---
# Huffman Coding

Huffman Coding is a popular method used for lossless data compression. The key idea behind Huffman Coding is to assign variable-length codes to input characters, lengths of the assigned codes are based on the frequencies of corresponding characters. The most frequent character gets the smallest code and the least frequent character gets the largest code.

The algorithm involves the following steps:

1. Create a leaf node for each unique character and build a min [[Heap Data Structure|heap]] of all leaf nodes (Min [[Heap Data Structure|Heap]] is used as a [[Priority Queue|priority queue]]. The value of frequency field is used to compare two nodes in min [[Heap Data Structure|heap]]. Initially, the least frequent character is at the root of min [[Heap Data Structure|heap]]).

2. Extract two nodes with the minimum frequency from the min [[Heap Data Structure|heap]].

3. Create a new internal node with a frequency equal to the sum of the two nodes frequencies. Make the first extracted node as its left child and the other extracted node as its right child. Insert this node into the min [[Heap Data Structure|heap]].

4. Repeat steps 2 and 3 until the [[Heap Data Structure|heap]] contains only one node. The remaining node is the root node and the tree is a Huffman Tree.

Let's denote the frequency of character $c$ as $f(c)$, and the length of the Huffman code for character $c$ as $l(c)$. The [[Expected Value|average]] length $L$ of the Huffman code is given by:

$$
L = \sum_{c} f(c) \cdot l(c)
$$

The Huffman coding algorithm guarantees that this [[Expected Value|average]] length is minimized, making the coding scheme as efficient as possible.

The proof of this optimality involves a swapping argument similar to that used in proving the greedy-choice property for the [[Fractional Knapsack Problem|Fractional Knapsack Problem]]. For a more detailed proof, you can refer to this [Google search for "Proof of optimality of Huffman Coding"](https://www.google.com/search?q=Proof+of+optimality+of+Huffman+Coding).

> For more in-depth understanding, you might want to check out these resources:
> - [Huffman Coding | Greedy Algo-3](https://www.google.com/search?q=Huffman+Coding+site:geeksforgeeks.org)
> - [A Method for the Construction of Minimum-Redundancy Codes](https://www.google.com/search?q=A+Method+for+the+Construction+of+Minimum-Redundancy+Codes) by David A. Huffman
> - [Huffman's Algorithm](https://www.google.com/search?q=Huffman%27s+Algorithm+site:brilliant.org) on Brilliant.org
> - [Huffman Coding](https://www.google.com/search?q=Huffman+Coding+site:wikipedia.org) on Wikipedia
