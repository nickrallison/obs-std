---
bad_links: 
aliases: []
tags: [linux, operatingsystems]
---
# Rsync

Rsync, which stands for "remote sync", is a powerful Linux command-line tool used for synchronizing files and directories between two locations. These locations can be on the same system, or they can be on different systems with communication over a network. Rsync is widely used for backups and mirroring data.

The primary advantage of Rsync is its efficiency. It uses an algorithm that minimizes data transfer by only moving the portions of files that have changed, rather than transferring entire files every time. This is particularly useful for large files or directories.

The basic syntax of the Rsync command is as follows:

```
rsync options source destination
```

Here, `options` are the parameters that control the behavior of Rsync, `source` is the file or directory that you want to copy, and `destination` is the location where you want to copy the file or directory.

Some commonly used options include:

- `-v` (verbose): This option increases the amount of [[Information Theory|information]] that Rsync displays during its operation.
- `-r` (recursive): This option tells Rsync to copy directories recursively.
- `-a` (archive): This option is a combination of several options (`-rlptgoD`) and preserves symbolic links, file permissions, user & group ownerships and timestamps during the copy.
- `-z` (compress): This option compresses the data before transferring it, which can speed up transfers over slow networks.
- `--delete`: This option deletes files in the destination directory that are not present in the source directory, effectively mirroring the source directory.

Rsync can also be used over SSH for secure data transfers. Here's an example of how to use Rsync with SSH:

```
rsync -avz -e ssh source user@remote_host:destination
```

In this command, `-e ssh` specifies that Rsync should use SSH for the data transfer, and `user@remote_host:destination` specifies the user and host for the SSH connection, as well as the destination directory on the remote host.

> For more [[Information Theory|information]] on Rsync, you can refer to the [Rsync man page](https://linux.die.net/man/1/rsync) or this [comprehensive guide on Rsync](https://www.tecmint.com/rsync-local-remote-file-synchronization-commands/).

As for formulas, derivations, and proofs, these are not typically applicable to Rsync as it is a practical tool rather than a theoretical concept. However, the efficiency of Rsync can be attributed to the rsync algorithm, which is based on the concept of rolling checksums and difference detection. The algorithm is designed to minimize network usage by only transferring the differences between the source and destination files.

> For a deep dive into the rsync algorithm, you can refer to the [original technical report](https://www.samba.org/~tridge/phd_thesis.pdf) by Andrew Tridgell, one of the creators of Rsync.