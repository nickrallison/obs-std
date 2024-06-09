---
bad_links: 
aliases: []
tags: [coding]
---
# [[Caching|Cache]] Python.md|LRU [[Caching|Cache]] Python]]

An LRU (Least Recently Used) [[Caching|Cache]] in Python is a type of data structure that stores the most recently used items and discards the least recently used items when the [[Caching|cache]] reaches its capacity limit. It follows the principle of "least recently used" where it eliminates the least recently used items first. This mechanism helps to manage memory in an optimized way, especially in systems where memory size is limited. In Python, you can implement an LRU [[Caching|cache]] using various methods, including collections.OrderedDict module or via lru_cache decorator from functools module. These methods allow faster access and insertion of data by keeping track of the order of insertion and access.

Sure, here are some examples of implementing an LRU [[Caching|Cache]] in Python.

Using collections.OrderedDict:

```python
from collections import OrderedDict

class LRUCache:
    def __init__(self, capacity):
        self.cache = OrderedDict()
        self.capacity = capacity

    def get(self, key):
        if key not in self.cache:
            return -1
        else:
            self.cache.move_to_end(key)
            return self.cache[key]

    def put(self, key, value):
        if key in self.cache:
            self.cache.move_to_end(key)
        else:
            if len(self.cache) >= self.capacity:
                self.cache.popitem(last=False)
        self.cache[key] = value
```

And using lru_cache decorator from functools module:

```python
from functools import lru_cache

@lru_cache(maxsize=2)
def get_data(key):
    # Simulate time delay while fetching data
    print(f"Fetching: {key}")
    return f"Data for {key}"

# Access the data
print(get_data("key1"))  # Fetches and prints "Data for key1"
print(get_data("key2"))  # Fetches and prints "Data for key2"
print(get_data("key1"))  # Returns cached "Data for key1"
print(get_data("key3"))  # Fetches and prints "Data for key3". Also removes "Data for key2" from [[Caching.md|cache]] as maxsize is 2.
print(get_data("key2"))  # Fetches again as it was removed from the [[Caching.md|cache]] earlier.
```

Note: In the second example with lru_cache, when you try to access more data than the maxsize, it will automatically remove the least recently used data.

## Related

Other tools from the python functools package include

1. lru_cache: This is a decorator to wrap a function with a memoizing callable that saves up to the maxsize most recent calls. It can save time when an expensive or I/O bound function is periodically called with the same arguments.

2. total_ordering: Given a class defining one or more rich comparison [[Ordering|ordering]] methods, this class decorator supplies the rest.

3. partial: Returns a new partial object which when called will behave like func called with the positional arguments args and keywords.

4. partialmethod: Method descriptor with partial application of the given arguments and keywords.

5. reduce: Apply function of two arguments cumulatively to the items of sequence, from left to right, so as to reduce the sequence to a single output.

6. singledispatch: Single-dispatch generic function decorator.

7. update_wrapper: Update a wrapper function to look like the wrapped function

8. wraps: This is a convenience function for invoking update_wrapper() as a function decorator when defining a wrapper function.

9. cmp_to_key: Transform an old-style comparison function to a key function used for sorting routines.

10. cached_property: Decorator that converts a method into a lazy property, effectively replacing a method call with an attribute lookup on first access, while also providing an opportunity for optimization in repeated access scenarios.

All these tools help in making code more readable, efficient and easy to debug and maintain by providing common programming patterns in Pythonic way.
