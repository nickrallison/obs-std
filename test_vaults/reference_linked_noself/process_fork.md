---
bad_links: 
aliases: []
tags: [operatingsystems]
---
# Process Fork

A process fork is a concept in computing where an existing process creates a copy of itself. This is typically used to run different executions concurrently. The newly created process is known as the child process, while the original process is referred to as the parent. Both processes continue to execute the next instruction following the fork [[System Call|system call]]. They are identical but can be told apart by their [[Process ID|process IDs]], as they do not share this attribute. The child process can continue running the same program as the parent or load a new program for execution.

