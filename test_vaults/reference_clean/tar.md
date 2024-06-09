---
bad_links: 
aliases: []
tags: [linux]
---
# Tar

The `tar` command, which stands for Tape Archive, is a widely used command in Unix and Linux systems. It is used to create, maintain, modify, and extract files that are archived in a `.tar` format. The `tar` command is often used in conjunction with compression utilities like `gzip` and `bzip2` to create compressed archive files.

The basic syntax of the `tar` command is as follows:

```
tar [options] [archive-file] [file or directory to be archived]
```

Here are some commonly used options with the `tar` command:

- `-c`: Create a new archive
- `-x`: Extract files from an archive
- `-v`: Verbosely list the files processed
- `-f`: Use archive file
- `-t`: List the contents of an archive
- `-z`: Filter the archive through `gzip`
- `-j`: Filter the archive through `bzip2`

For example, to create a tar archive of a directory, you would use:

```
tar -cvf archive_name.tar directory_name/
```

And to extract the same archive, you would use:

```
tar -xvf archive_name.tar
```

The `tar` command works by sequentially writing or reading files to or from the archive. The archive is essentially a concatenation of the files included in it, preceded by a header with metadata about the file. The format of this header is defined by the POSIX standard.

The `tar` command is closely related to file systems and file permissions in Unix and Linux systems. When creating an archive, `tar` preserves the file permissions and ownership information of the archived files. This makes it a useful tool for backing up and transferring files between systems.

As for formulas, derivations, or proofs, these are not typically associated with the `tar` command as it is a practical tool used for file archiving rather than a theoretical concept.

> For more information, you can refer to the [GNU Tar Manual](https://www.gnu.org/software/tar/manual/tar.html) or the [tar man page](https://man7.org/linux/man-pages/man1/tar.1.html).