---
bad_links: 
aliases: []
tags: [imageprocessing]
---
# Anisotropic Filtering

Anisotropic Filtering (AF) is a technique used in 3D computer graphics that enhances the image quality of textures on surfaces. It works by reducing blur and preserving detail at extreme viewing angles. This method improves the clarity and crispness of textured objects displayed at oblique angles. Anisotropic filtering has a higher computational cost compared to other methods such as bilinear or [[Trilinear Filtering|trilinear filtering]], but it provides superior results, particularly in scenes with highly detailed textures.

```pseudo
\begin{algorithm}
\caption{Anisotropic Filtering}
\begin{algorithmic}
  \Procedure{AnisotropicFilter}{$Image, FilterSize, MaxAngle$}
	\State Initialize an empty output image of the same size as the input image
	\For{each pixel $p$ in $Image$}
	  \State Compute the gradient direction at pixel $p$
	  \State Compute a weighted average of the colors of pixels within a window of size $FilterSize$ around pixel $p$, with weights determined by the angle between their gradient directions and that of pixel $p$. Pixels with angles greater than $MaxAngle$ are given zero weight.
	  \State Set the color of pixel $p$ in the output image to be this weighted average
	\EndFor
  \EndProcedure
  \end{algorithmic}
\end{algorithm}
```
This pseudocode describes a basic version of anisotropic filtering. The actual implementation may involve additional steps such as mipmapping and bilinear [[Linear Interpolation|interpolation.]]