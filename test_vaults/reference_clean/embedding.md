---
bad_links: 
aliases: []
tags: [machinelearning]
---
# Embedding

"Embedding" is a term used in machine learning and data science. It refers to the process of converting complex, high-dimensional data into a lower-dimensional format that can be more easily processed by machine learning algorithms. This is often done with text or categorical data, where each unique word or category is mapped to a vector of real numbers. These embeddings represent word meanings in a high dimensional vector space. These vectors capture the relationships between different words or categories, making it easier for algorithms to understand and use this data. Embedding is a crucial part of many natural language processing tasks, such as sentiment analysis, text classification, and machine translation.

A common way embeddings are generated is a trained neural network on m inputs into n outputs, reducing the dimensionality from m to n

An example of how embeddings are used is that their goal is to preserve some form of “vectorness” from the high dimensional data. If for example we have a word $w$ and the embedding of that word if $E(w)$, then the embedding should provide some form of addition to the data. For instance, the following should be preserved:

$$
E(\text{king}) - E(\text{man}) + E(\text{woman}) \approx E(\text{queen})
$$