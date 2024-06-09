---
bad_links: 
aliases: [Hardlink]
tags: [operatingsystems]
---
# Hardlinks

Hard links in Linux are essentially directory entries or pointers that associate a name with an inode number on a filesystem. An inode (index node) is a data structure that stores all the information about a file, except its name and actual data. This information includes the file's size, permissions, ownership, timestamps, and the location of the file's data blocks.

When you create a hard link, you are creating a new directory entry that points to the same inode as an existing file. This means that the hard link and the original file both refer to the same physical location on the disk where the file's data is stored. They are indistinguishable from each other, and changes made to the file are reflected in all its hard links.

The command to create a hard link in Linux is `ln source_file hardlink`, where `source_file` is the name of the existing file and `hardlink` is the name of the hard link you want to create.

Here are some important characteristics of hard links:

1. Hard links share the same inode and data blocks as the original file. They are not separate copies of the file.
2. Deleting a hard link does not delete the file's data until all hard links to the file are deleted.
3. Hard links cannot span different filesystems because inode numbers are unique within a filesystem but not across different filesystems.
4. Hard links cannot be created for directories to prevent the possibility of creating filesystem loops.

Tangentially related to hard links are symbolic links (or soft links), which are a different type of link in Linux. Unlike hard links, symbolic links are separate files that contain a reference to the path of another file or directory. They do not share the same inode as the file they link to.

There are no specific formulas, derivations, or proofs related to hard links, as they are a feature of filesystems and operating systems rather than a mathematical or theoretical concept.

> For more information, you can refer to the following resources:
> - [Hard Link vs Soft Link](https://www.google.com/search?q=Hard+Link+vs+Soft+Link)
> - [Inode in Linux](https://www.google.com/search?q=Inode+in+Linux)
> - [Filesystem Hierarchy Standard](https://www.google.com/search?q=Filesystem+Hierarchy+Standard)