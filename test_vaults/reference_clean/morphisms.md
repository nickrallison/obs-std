---
bad_links: 
aliases: [morphism]
tags: [grouptheory]
---
# Morphisms

Morphisms are a fundamental concept in the field of abstract algebra, specifically in category theory. A morphism is a kind of transformation from one mathematical object to another. It's a way of mapping or translating between two structures while preserving their essential properties. The most familiar types of morphisms are functions, which map numbers to numbers, but morphisms can also map groups to groups, spaces to spaces, and so on. The key idea is that these mappings preserve the structure: they carry over the properties and relations of the original object to the new one.

Given the context of morphisms in abstract algebra, there isn't a specific formula that represents a morphism, as it's more of a general concept. However, one basic and important aspect of morphisms is that they preserve structure. For functions (which are one type of morphism), this can be represented as follows:

$$
\begin{gather*}
f: A \rightarrow B \\
\forall a_1, a_2 \in A, \: if \: a_1 = a_2 , then \: f(a_1) = f(a_2)
\end{gather*}
$$

This means that if we have two elements in set A that are equal, their images in set B under the function (or morphism) f will also be equal. In other words, the function preserves the "=" operation.

Similarly for group homomorphisms (another type of morphism), which are mappings between groups that preserve the group operation:

$$
\begin{gather*}
f: G \rightarrow H \\
\forall g_1, g_2 \in G , \: f(g_1 * g_2) = f(g_1) *' f(g_2)
\end{gather*}
$$

Here `*` is the group operation in G and `*'` is the group operation in H. This equation says that for any two elements g1 and g2 in G, operating on them first and then applying the function is the same as applying the function to each separately and then performing the operation in H.