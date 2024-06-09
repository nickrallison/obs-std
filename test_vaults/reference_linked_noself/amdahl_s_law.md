---
bad_links: 
aliases: []
tags: [computerscience]
---
# Amdahl's Law

Amdahl's Law is a formula used to identify the maximum improvement possible by enhancing a particular part of a system. In computing, it's often used to determine the maximum benefit of adding more processors to perform tasks. The law is named after Gene Amdahl, an American computer scientist. It essentially states that the overall improvement gained by optimizing a part of the system is limited by the fraction of time that part is actually utilized. Thus, if a system spends only 20% of its time on a particular function, optimizing this function will not significantly improve overall system performance.

The formula for Amdahl's Law is typically written as follows:

$$
S_{\text{latency(s)}} = \frac{1}{(1-p) + \frac{p}{s}}
$$

Where:
- $S_{\text{latency(s)}}$ is the theoretical speedup of the execution of the whole task;
- $s$ is the speedup of the part of the task that benefits from improved system resources;
- $p$ is the proportion of execution time that the part benefiting from improved resources occupied before improvement.