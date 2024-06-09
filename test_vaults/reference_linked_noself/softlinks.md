---
bad_links: 
aliases: []
tags: [operatingsystems, linux]
---
# Softlinks

Soft links, also known as symbolic links or symlinks, are a type of file in Unix-like operating systems such as Linux. They are essentially pointers or references to other files or directories in the file system. 

A soft link is created using the `ln -s` command followed by the target file or directory and the name of the link. Here's an example:

```bash
ln -s /path/to/original /path/to/link
```

In this example, `/path/to/original` is the file or directory that the link points to, and `/path/to/link` is the name and location of the link itself. 

When a soft link is accessed, the operating system redirects the operation to the file or directory that the link points to. This redirection is transparent to most applications, which means they can operate on a soft link as if it were the original file or directory. 

However, there are some important differences between a soft link and the original file or directory:

1. A soft link does not contain the data of the original file or directory. Instead, it contains a text string that is the path to the original file or directory. 

2. A soft link can span file systems, while a hard link (another type of link in Unix-like operating systems) cannot. 

3. If the original file or directory is moved, renamed, or deleted, the soft link will not update to reflect this change. In this case, the soft link is said to be "broken".

4. The permissions of a soft link are not used when accessing the file or directory it points to. Instead, the permissions of the original file or directory are used. 

The inode (index node) is a data structure in a Unix-style file system that describes a file-system object such as a file or a directory. Each inode stores the attributes and disk block locations of the object's data. File-system object attributes may include metadata (times of last change, access, modification), as well as owner and permission data.

Directories are lists of names assigned to inodes. A directory contains an entry for itself, its parent, and each of its children. There is a distinction between a file name and a path leading to a file. A file name is a name assigned to an inode. A path is a way to a file.

When a symbolic link is created, a new inode is created. The new inode points to the path of the original file, not the inode of the original file. This is the main difference between a hard link and a symbolic link.

> For more information, you may want to read about [File Systems in Linux](https://www.google.com/search?q=File+Systems+in+Linux), [Inodes](https://www.google.com/search?q=Inodes), and [Hard Links vs Soft Links](https://www.google.com/search?q=Hard+Links+vs+Soft+Links).