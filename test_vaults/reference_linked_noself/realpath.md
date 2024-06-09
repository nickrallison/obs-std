---
bad_links: 
aliases: []
tags: [linux]
---
# Realpath

The `realpath` function is a utility in Unix and Unix-like operating systems that converts a file path that may include symbolic links, relative path components (like `.` and `..`), and duplicate slashes, into an absolute path that names the same location in a file system. This absolute path does not include any symbolic links, `.` or `..` components, or duplicate slashes.

The `realpath` function is often used in shell scripts and C programs to ensure that a file path is absolute and canonical, which can simplify file handling code and make it more robust.

Here is an example of how `realpath` might be used in a shell script:

```bash
#!/bin/bash
file_path=$(realpath "$1")
echo "The absolute path is: $file_path"
```

In this script, `realpath` is used to convert the first command line argument (`$1`) into an absolute path, which is then printed to the console.

The `realpath` function is part of the POSIX standard, which means it should be available on any Unix-like operating system. However, not all systems include it by default, so it may need to be installed separately.

The `realpath` function is closely related to the `readlink -f` command, which also resolves symbolic links and relative path components. However, `readlink -f` is not part of the POSIX standard, so it may not be available on all systems.

> For more information on the `realpath` function, you can refer to the [realpath man page](https://man7.org/linux/man-pages/man3/realpath.3.html). For a deeper understanding of file paths and symbolic links in Unix, you might find the following resources helpful:
> - [Filesystem Hierarchy Standard](https://en.wikipedia.org/wiki/Filesystem_Hierarchy_Standard)
> - [Symbolic link](https://en.wikipedia.org/wiki/Symbolic_link)
> - [Path (computing)](https://en.wikipedia.org/wiki/Path_(computing))