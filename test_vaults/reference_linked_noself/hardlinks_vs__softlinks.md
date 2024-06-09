---
bad_links: 
aliases: []
tags: [operatingsystems, linux]
---
# [[Hardlinks|Hardlinks]] Vs. [[Softlinks|Softlinks]]

[[Hardlinks|Hardlinks]] and [[Softlinks|softlinks]] are two types of links in Unix/Linux filesystems that allow multiple filenames to refer to the same file content. They are used for various purposes, such as creating shortcuts, avoiding duplication, and maintaining backward compatibility.

**[[Hardlinks|Hardlinks]]**:

A [[Hardlinks|hardlink]] is essentially an additional name for an existing file on Unix/Linux filesystems. All [[Hardlinks|hardlinks]] to a file actually refer to the same file, and it's impossible to tell which name is the "original" and which are "links". 

In Unix/Linux, a file is a collection of data stored on a disk. The data is organized into blocks, and the filesystem keeps track of which blocks belong to which files. Each file also has an inode (index node), which is a data structure that contains [[Information Theory|information]] about the file, such as its size, permissions, and the location of its data blocks. 

When you create a [[Hardlinks|hardlink,]] you're creating a new directory entry (filename) that points to the existing file's inode. The inode keeps a count of how many [[Hardlinks|hardlinks]] point to it. When the count drops to zero (i.e., when all [[Hardlinks|hardlinks]] are deleted), the data blocks are marked as free and the file is deleted.

Here's a simplified representation of how [[Hardlinks|hardlinks]] work:

```
file1 ----> inode ----> data blocks
  |
file2 ----> 
```

**[[Softlinks|Softlinks]] (Symbolic Links)**:

A softlink, also known as a symbolic link or symlink, is a special type of file that serves as a reference to another file or directory. Unlike a [[Hardlinks|hardlink,]] a symlink is a separate file with its own inode. The data blocks of a symlink contain the path to the file it links to.

When you access a symlink, the filesystem transparently redirects you to the file the symlink points to. If the target file is deleted, the symlink becomes a "dangling" link that points to a non-existent file. 

Here's a simplified representation of how symlinks work:

```
symlink ----> inode ----> data blocks (containing path to target file)
  |
target file ----> inode ----> data blocks
```

**Differences between [[Hardlinks|Hardlinks]] and Softlinks**:

1. **Referential Integrity**: [[Hardlinks|Hardlinks]] maintain referential integrity. Even if the original filename is deleted, the [[Hardlinks|hardlink]] still provides access to the file content. In contrast, if the target of a symlink is deleted, the symlink becomes a dangling link.

2. **Filesystem Boundaries**: [[Hardlinks|Hardlinks]] can only be created for files in the same filesystem. Symlinks can span filesystems as they simply store the path to the target file.

3. **Linking to Directories**: By default, most Unix/Linux systems don't allow creating [[Hardlinks|hardlinks]] to directories to prevent potential filesystem corruption (e.g., circular references). Symlinks can link to both files and directories.

4. **Storage Space**: [[Hardlinks|Hardlinks]] use less space as they only create a new directory entry. Symlinks require space for a new inode and data blocks to store the target path.

5. **File Attributes**: [[Hardlinks|Hardlinks]] share the same attributes (permissions, ownership, timestamps) as the original file because they point to the same inode. Symlinks have their own attributes.

In terms of formulas, derivations, and proofs, these concepts are more applicable to the underlying data structures and algorithms used in filesystems, such as B-trees for indexing, rather than the concept of links itself.

> For further reading, you may want to explore the following resources:
> - [Inode Pointer Structure in Unix File System](https://www.geeksforgeeks.org/unix-file-system/)
> - [Symbolic Links vs Hard Links](https://www.geeksforgeeks.org/soft-hard-links-unixlinux/)
> - [Understanding UNIX / Linux symbolic (soft) and hard links](https://www.cyberciti.biz/tips/understanding-unixlinux-symbolic-soft-and-hard-links.html)