---
bad_links: 
aliases: []
tags: [operatingsystems, coding]
---
# Parallel Programming

Parallel programming is a type of computation in which multiple calculations or processes are carried out simultaneously. It is largely used in high performance computing, but it's also beneficial in the realm of interactive applications where it helps to make better use of multi-core CPUs. Parallel programming involves dividing a problem into subproblems that can be solved concurrently, with each part being executed on different cores or processors. This type of programming increases efficiency and speed, as tasks can be completed faster when they are divided and executed simultaneously. However, it also makes program design more complex and requires careful planning and synchronization to avoid conflicts or inconsistencies.

It is not always possible that every program can be sped up by parallel processing if the algorithm is not parallelizable (See [[Amdahl's Law|Amdahl's Law]]).

```Python
# This is a simple example of parallel programming using Python's multiprocessing module.

import multiprocessing

def worker(num):
    """Thread worker function"""
    print('Worker:', num)

if __name__ == '__main__':
    jobs = []
    for i in range(5):
        p = multiprocessing.Process(target=worker, args=(i,))
        jobs.append(p)
        p.start()
```

In this example, the main process is creating five worker processes. Each worker process runs independently and concurrently to each other. This demonstrates the basic idea of parallel programming - splitting up a larger task into many smaller tasks that can be run at the same time on different cores or processors.