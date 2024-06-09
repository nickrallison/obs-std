---
bad_links: 
aliases: [garbage collection]
tags: [coding, operatingsystems]
---
# Garbage Collector

A Garbage Collector (GC) is a form of automatic memory management used in many modern programming languages. Its role is to automatically reclaim the memory occupied by objects or variables that are no longer in use by the program. This helps in preventing [[Memory Leak|memory leaks]] and can also help improve the performance of the program. The garbage collector works by tracking each object in the system, identifying which ones are still accessible or needed and which are not, and freeing up the memory used by the unneeded objects.

In general, the garbage collection algorithm works why viewing a program and its memory as a graph, as soon as a node in the graph is no longer reachable, the memory would be automatically freed. This can be viewed as searching for an island in a disconnected graph