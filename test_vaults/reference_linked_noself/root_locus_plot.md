---
bad_links: 
aliases: []
tags: [controlsystems]
date created: Monday, June 26th 2023, 3:32:29 pm
title: Root Locus Plot
---

# Root Locus Plot

A root locus plot is a graphical representation of the possible locations of the poles of a system as a parameter, typically the gain, is varied. It is a useful tool for analyzing the [[LTI System Stability|stability]] and [[Transient Response|transient response]] of a control system.

The basic rules for creating a root locus plot are as follows:

1. Start with the open-loop [[Transfer Function|transfer function]] of the system. This [[Transfer Function|transfer function]] represents the relationship between the input and output of the system without any feedback.

2. Determine the poles and zeros of the [[Transfer Function|transfer function]]. The poles are the values of s that make the denominator of the [[Transfer Function|transfer function]] equal to zero, while the zeros are the values of s that make the numerator equal to zero.

3. Identify the branches of the root locus. Each branch starts at a pole or zero of the [[Transfer Function|transfer function]] and ends at another pole or at infinity. The branches represent the possible locations of the system's poles as the gain is varied.

4. Determine the angles and magnitudes of the branches. The angles of the branches are determined by the complex conjugate poles and zeros. The angles start at the poles and zeros and end at the real axis. The magnitudes of the branches are determined by the gain.

5. Determine the breakaway and break-in points. The breakaway points are the values of the gain at which the poles move from the real axis to the complex plane. The break-in points are the values of the gain at which the poles move from the complex plane to the real axis.

6. Analyze the [[LTI System Stability|stability]] and [[Transient Response|transient response]] of the system. The root locus plot can provide insights into the [[LTI System Stability|stability]] of the system. If all the poles of the system are located in the left half of the complex plane, the system is stable. The [[Transient Response|transient response]] of the system can also be analyzed by examining the damping ratio and natural frequency of the poles.

Root locus plots can be used for designing basic control systems by allowing engineers to determine the appropriate gain values to achieve desired system performance. By analyzing the root locus plot, engineers can select gain values that result in stable systems with desired [[Transient Response|transient response]] characteristics.

For more detailed information and examples, you can refer to the following resources:

1. [Root Locus Plot - Wikipedia](https://en.wikipedia.org/wiki/Root_locus)
2. [Control Systems Engineering by Norman S. Nise](https://www.amazon.com/Control-Systems-Engineering-Norman-Nise/dp/1118170512)

