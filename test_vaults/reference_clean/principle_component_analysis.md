---
aliases: []
tags: [probability]
title: Principle Component Analysis
date created: Saturday, July 15th 2023, 7:06:46 pm
bad_links: [Symmetric Relation.md]
---
# Principle Component Analysis

Principal Component Analysis (PCA) is a statistical procedure that uses an orthogonal transformation to convert a set of observations of possibly correlated variables into a set of values of linearly uncorrelated variables called principal components. This transformation is defined in such a way that the first principal component has the largest possible variance (that is, accounts for as much of the variability in the data as possible), and each succeeding component in turn has the highest variance possible under the constraint that it is orthogonal to the preceding components.

The steps involved in PCA are as follows:

1. **Standardize the data**: PCA is affected by scale, so you need to scale the features in your data before applying PCA. Use StandardScaler in scikit-learn to standardize the dataset’s features onto unit scale (mean = 0 and variance = 1).

2. **Compute the [[Covariance.md|Covariance matrix]]**: The aim of this step is to understand how the variables of the input data set are varying from the mean with respect to each other, or in other words, to see if there is any relationship between them. The covariance matrix is a p × p symmetric matrix (where p is the number of dimensions) that has as entries the covariances associated with all possible pairs of the initial variables. For each pair of coordinates $(X_i, X_j)$, the covariance is given by the following formula:

$$
\sigma_{ij} = \frac{1}{n}\sum_{k=1}^{n}(x_{ki} - \bar{x_i})(x_{kj} - \bar{x_j})
$$

1. **Compute the [[Eigenvalue.md|Eigenvalues]] and [[Eigenvectors.md|Eigenvectors]] of the [[Covariance.md|covariance matrix]]**: These are required for the next step of PCA. The eigenvectors (principal components) determine the directions of the new feature space, and the eigenvalues determine their magnitude. In other words, the eigenvalues explain the variance of the data along the new feature axes.

2. **Sort [[Eigenvalue.md|eigenvalues]] and corresponding [[Eigenvectors.md|eigenvectors]]**: The next step is to sort the eigenvalues in descending order and choose the top k eigenvectors that correspond to the k largest eigenvalues where k is the number of dimensions of the new feature subspace (k≤p).

3. **Transform the original dataset**: This is the final step of PCA, and it is also the easiest. Once we have chosen the principal components (eigenvectors), we just need to take their dot product with the original dataset.

The mathematical derivation of PCA involves the concepts of eigenvalues and eigenvectors. The covariance matrix of a data set is symmetric, and all symmetric matrices have a set of orthogonal eigenvectors. Therefore, the covariance matrix can be orthogonally diagonalized. If we order the eigenvectors by their corresponding eigenvalues, we get the principal components of the data set.

For further reading, you might find the following resources helpful:

> - [Principal Component Analysis Explained Visually](http://setosa.io/ev/principal-component-analysis/)
> - [A Step by Step Explanation of Principal Component Analysis](https://builtin.com/data-science/step-step-explanation-principal-component-analysis)
> - [Principal Component Analysis 4 Dummies: Eigenvectors, Eigenvalues and Dimension Reduction](https://georgemdallas.wordpress.com/2013/10/30/principal-component-analysis-4-dummies-eigenvectors-eigenvalues-and-dimension-reduction/)
> - [Principal Component Analysis in 3 Simple Steps](https://sebastianraschka.com/Articles/2015_pca_in_3_steps.html)