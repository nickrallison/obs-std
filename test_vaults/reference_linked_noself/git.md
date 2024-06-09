---
bad_links: 
aliases: [github]
tags: [coding]
---
# Git

Git is a distributed version control system (DVCS) that was created by Linus Torvalds, the creator of Linux, in 2005. It is used to track changes in source code during software development, allowing multiple developers to work on the same project simultaneously without overwriting each other's changes. Git is designed to handle everything from small to very large projects with speed and efficiency.

The core of Git's philosophy is the concept of a "snapshot". Unlike other version control systems that track the differences between versions of files, Git takes a snapshot of what all the files in your project look like at that moment and stores a reference to that snapshot. If files have not changed, Git doesn’t store the file again—just a link to the previous identical file it has already stored. This makes Git more like a mini filesystem with some incredibly powerful tools built on top of it, rather than simply a VCS.

Git has three main states that your files can reside in: committed, modified, and staged. 

- **Committed** means that the data is safely stored in your local database. 
- **Modified** means that you have changed the file but have not committed it to your database yet. 
- **Staged** means that you have marked a modified file in its current version to go into your next commit snapshot.

These lead us to the three main sections of a Git project: the Git directory, the working tree, and the staging area.

- The **Git directory** is where Git stores the metadata and object database for your project. This is the most important part of Git, and it is what is copied when you clone a repository from another computer.
- The **working tree** is a single checkout of one version of the project. These files are pulled out of the compressed database in the Git directory and placed on disk for you to use or modify.
- The **staging area** is a simple file, generally contained in your Git directory, that stores [[Information Theory|information]] about what will go into your next commit. It’s sometimes referred to as the "index", but it’s also common to refer to it as the staging area.

The basic Git workflow goes something like this:

1. You modify files in your working tree.
2. You selectively stage just those changes you want to be part of your next commit, which adds only those changes to the staging area.
3. You do a commit, which takes the files as they are in the staging area and stores that snapshot permanently to your Git directory.

If a particular version of a file is in the Git directory, it’s considered committed. If it has been modified and was added to the staging area, it is staged. And if it was changed since it was checked out but has not been staged, it is modified.

Git uses the SHA-1 hash algorithm, which produces a 160-bit (20-byte) hash value known as a checksum. The formula for a [[Hash Function|hash function]] can be represented as:

$$
h: U \rightarrow Y
$$

where $U$ is the set of all possible inputs (in Git's case, this would be all possible file changes), and $Y$ is the set of all possible hash values.

> For more in-depth information, you can refer to the [Pro Git book](https://git-scm.com/book/en/v2), which is an excellent resource for understanding Git. For a deeper dive into the mathematics of [[Hash Function|hash functions]], you can check out this [Google search](https://www.google.com/search?q=mathematics+of+hash+functions).