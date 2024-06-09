---
aliases: 
tags:
  - coding
bad_links:
---
# Mutex

A Mutex, short for mutual exclusion, is a program object that allows multiple program threads to share the same resource, such as file access, but not simultaneously. When a program is running, it may be split into several threads of execution, all occurring at the same time, to perform different tasks. However, when these threads attempt to access the same resource or data, it could lead to race conditions where the outputs depend on the non-deterministic sequence of execution of operations. This situation could cause data inconsistency and make the software behave unpredictably.

## How Mutex Works

A Mutex works by ensuring that only one thread can access a particular piece of code or data at a time. When a thread wants to access a shared resource, it attempts to lock the mutex associated with that resource. If the mutex is unlocked, the thread locks it, accesses the resource, and then unlocks the mutex when it's done. If the mutex is already locked, the thread must wait until the mutex becomes available (i.e., unlocked by the owner thread) before it can access the resource. This way, mutexes ensure that data consistency is preserved, and race conditions are avoided.

## Key Properties of Mutexes

### Mutual Exclusion

Only one thread can hold the mutex lock at any given time, which enforces mutual exclusion on the access to the shared resource.

### Ownership

Mutex locks have ownership, which means that only the thread that locked the mutex can unlock it. This prevents a thread from unlocking a mutex it doesn't own, which could lead to errors.

### Blocking and Non-blocking Locks

Mutex implementations can be blocking or non-blocking. A blocking mutex causes a thread to wait if it tries to acquire the lock while the mutex is already locked by another thread. In contrast, a non-blocking attempt to acquire a mutex (try-lock) may fail immediately if the mutex is locked, allowing the thread to do something else instead of waiting.

## Usage in Various Programming Languages

Mutex support and the specific API may vary across different programming languages and frameworks, but the core concept remains the same. Here are brief examples of how mutexes can be used in a few programming languages:

### C++

```cpp
#include <iostream>
#include <mutex>
#include <thread>

std::mutex mtx; // mutex for critical section

void print_foo(int n) {
    mtx.lock();
    for (int i = 0; i < n; ++i) {
        std::cout << "foo ";
    }
    std::cout << "\n";
    mtx.unlock();
}

int main() {
    std::thread t1(print_foo, 10);
    std::thread t2(print_foo, 10);
    t1.join();
    t2.join();
    return 0;
}
```

### Python

```python
from threading import Thread, Lock

mutex = Lock()

def print_bar(n):
    with mutex:
        for i in range(n):
            print("bar ", end="")
        print()

if __name__ == "__main__":
    t1 = Thread(target=print_bar, args=(10,))
    t2 = Thread(target=print_bar, args=(10,))
    t1.start()
    t2.start()
    t1.join()
    t2.join()
```

## Conclusion

Mutexes are fundamental constructs in the world of concurrent programming, essential for preventing race conditions and ensuring the consistency of shared data. Proper use of mutexes can help create robust, multi-threaded applications that are safe from the pitfalls of unsynchronized access to shared resources.