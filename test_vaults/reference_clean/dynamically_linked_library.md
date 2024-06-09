---
bad_links: 
aliases:
  - shared object library
  - dynamically linked
  - dynamically link
  - dynamic link
  - shared library
  - Dynamic Linking
  - dynamically linked libraries
tags:
  - operatingsystems
  - c
  - cpp
  - compilers
---
# Dynamically Linked Library

A Dynamically Linked Library (DLL) is a feature of Microsoft Windows that allows multiple programs to access shared functions and procedures stored in a single file. These libraries contain code and data that can be used by more than one program at the same time. This can help in saving memory, and also allows users to edit coding functions without having to re-link or re-compile the entire program. DLL files are loaded into memory only when they are called upon by programs, which can help in improving the efficiency of system performance.

The version of DLLs Linux uses are called Shared Object libraries (or .so files). These files are similar to DLLs in Windows in that they contain code and data that can be used by multiple programs simultaneously. Shared Object libraries can be dynamically loaded and linked into a program at runtime, which allows a program to use the most recent version of a specific library, reducing the size of the program and potentially adding or improving functionality.

The Shared Object libraries are typically stored in /lib, /usr/lib, or /usr/local/lib directories. They can be loaded at either load time or runtime. Load time linking is done when the program is initially loaded into memory, before it starts running. Runtime linking is done while the program is executing.

In Linux, the dynamic linker (ld.so or ld-linux.so) takes care of loading these shared object libraries. The dynamic linker checks for dependencies, finds the shared libraries needed, loads them into memory, and links them with the program.

The use of Shared Object libraries in Linux provides several benefits. It promotes code reuse and modularity; the same library can be used by multiple programs, which means that updates or bug fixes in the library benefit all the programs using it. It also saves disk space and memory, as multiple instances of a program will share the same library rather than each having their own copy.

However, there are also potential drawbacks. If a shared library is updated with changes that are not backward-compatible, programs depending on the old version might not work correctly. This problem is known as "dependency hell". To mitigate this issue, Linux uses versioned symbols and maintains backward compatibility in shared libraries.

In conclusion, Shared Object libraries play a crucial role in Linux systems by promoting efficient memory use and code reusability. Despite potential dependency issues, their benefits make them an integral part of Linux programming.