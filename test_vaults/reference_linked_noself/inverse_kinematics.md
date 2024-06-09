---
bad_links: 
aliases: []
tags: [robotics]
title: Inverse Kinematics
date created: Monday, July 10th 2023, 2:52:23 pm
---
# Inverse Kinematics

Inverse Kinematics (IK) is a fundamental concept in robotics and computer graphics that deals with the computational problem of determining the input parameters of a mechanical system to achieve a desired position or orientation. In simpler terms, it's about figuring out how to move a robotic arm (or any articulated system) to a specific point in space.

The "inverse" in Inverse Kinematics refers to the fact that we're working backwards from the goal (the desired end position) to the source (the current position). This is the opposite of Forward Kinematics, where we start with the source and calculate the end position.

The mathematical representation of a robotic system is often done using Denavit-Hartenberg parameters, which describe the relative positions and orientations of adjacent links in the chain. 

The IK problem can be stated as follows: Given a desired end-effector position $P_d = [x_d, y_d, z_d]^T$ and orientation $R_d$, find the joint parameters $q = [q_1, q_2, …, q_n]^T$ that will result in the end-effector reaching the desired position and orientation.

This problem is often solved using numerical methods, such as the [[Jacobians|Jacobian]] Inverse or [[Jacobians|Jacobian]] Transpose methods. The [[Jacobians|Jacobian matrix]] $J(q)$ of a robotic system is a matrix that represents the differential changes in position and orientation of the end-effector with respect to differential changes in the joint parameters. 

The [[Jacobians|Jacobian]] Inverse method uses the inverse of the [[Jacobians|Jacobian matrix]] to calculate the joint parameters:

$$
\Delta q = J^{-1}(q) \cdot \Delta X
$$

where $\Delta X = [P_d - P(q), R_d - R(q)]^T$ is the difference between the desired and current end-effector position and orientation, and $\Delta q$ is the change in joint parameters.

However, the [[Jacobians|Jacobian matrix]] may not always be invertible. In such cases, the [[Jacobians|Jacobian]] Transpose or Pseudo-Inverse methods can be used. The [[Jacobians|Jacobian]] Transpose method is simpler and involves using the transpose of the [[Jacobians|Jacobian matrix]]:

$$
\Delta q = J^T(q) \cdot \Delta X
$$

The Pseudo-Inverse method uses the Moore-Penrose pseudo-inverse of the [[Jacobians|Jacobian matrix]]:

$$
\Delta q = J^+(q) \cdot \Delta X
$$

where $J^+(q) = (J^T(q) \cdot J(q))^{-1} \cdot J^T(q)$ is the pseudo-inverse of the [[Jacobians|Jacobian matrix]].

These methods iteratively adjust the joint parameters until the end-effector reaches the desired position and orientation.

> For more in-depth understanding, you may want to read the following resources:
> - ["Robotics, Vision and Control: Fundamental Algorithms in MATLAB"](https://www.google.com/search?q=Robotics%2C+Vision+and+Control%3A+Fundamental+Algorithms+in+MATLAB) by Peter Corke
> - ["Introduction to Inverse Kinematics with Jacobian Transpose, Pseudoinverse and Damped Least Squares methods"](https://www.google.com/search?q=Introduction+to+Inverse+Kinematics+with+Jacobian+Transpose%2C+Pseudoinverse+and+Damped+Least+Squares+methods) by Samuel Buss
> - ["A Mathematical Introduction to Robotic Manipulation"](https://www.google.com/search?q=A+Mathematical+Introduction+to+Robotic+Manipulation) by Richard M. Murray, Zexiang Li, and S. Shankar Sastry
