---
bad_links: 
aliases: []
tags: [computergraphics, computerscience]
---
# Ray Marching

Ray Marching is a technique used in computer graphics to render a 3D scene. Unlike traditional ray tracing, which calculates the intersection of a ray with a surface, ray marching advances the ray in small steps and checks if it has hit an object at each step. This technique is particularly useful for rendering complex and intricate shapes that are difficult to represent with traditional geometric primitives.

The basic algorithm for ray marching is as follows:

1. For each pixel in the image, cast a ray from the camera through the pixel into the scene.
2. Start at a certain distance along the ray, and take a step along the ray.
3. Check if the ray has intersected an object in the scene. If it has, color the pixel based on the properties of the object.
4. If the ray has not intersected an object, take another step along the ray and repeat step 3. Continue this process until the ray has either hit an object or traveled a certain maximum distance.

The step size in ray marching can be constant, but it is often more efficient to use a variable step size based on the distance to the nearest object. This is known as sphere tracing or distance field ray marching. The step size is determined by a distance function, which gives the shortest distance from any point in space to the nearest surface. The formula for the step size is:

$$
\Delta t = \min(\Delta t_{\text{max}}, D(P))
$$

where $\Delta t_{\text{max}}$ is the maximum step size, $D(P)$ is the distance function evaluated at the current point $P$ along the ray, and $\Delta t$ is the step size.

Ray marching is often used in combination with signed distance functions (SDFs), which not only give the distance to the nearest surface, but also indicate whether the point is inside or outside the surface by the sign of the distance. SDFs can represent complex shapes and perform operations like blending and deformation, making them a powerful tool for ray marching.

One of the key advantages of ray marching is its ability to render volumetric effects like fog, smoke, and clouds, which are challenging to achieve with traditional ray tracing. This is done by accumulating the color and opacity along the ray as it marches through the volume.

> For more in-depth information, you can refer to these resources:
> - [Ray Marching and Signed Distance Functions](https://www.iquilezles.org/www/articles/distfunctions/distfunctions.htm)
> - [Introduction to Ray Marching](https://www.alanzucconi.com/2016/07/01/raymarching/)
> - [Sphere Tracing: A Geometric Method for the Antialiased Ray Tracing of Implicit Surfaces](https://www.google.com/search?q=Sphere+Tracing%3A+A+Geometric+Method+for+the+Antialiased+Ray+Tracing+of+Implicit+Surfaces)

## Source

[Youtube Video](https://www.youtube.com/watch?v=BNZtUB7yhX4)

