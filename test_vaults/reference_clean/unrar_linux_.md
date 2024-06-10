---
bad_links: 
aliases: []
tags: [linux]
---
# Unrar (Linux)

Unrar is a utility that is used to extract files from RAR archives. RAR (Roshal Archive) is a proprietary archive file format that supports data compression, error recovery, and file spanning. It was developed by a Russian software engineer, Eugene Roshal (hence the name), but the copyright for the software is held by his brother Alexander.

The Unrar utility is available in two forms in Linux: freeware "unrar" and shareware "UnRAR". The freeware version is included with many Linux distributions and is typically used for uncompressing RAR files, while the shareware version must be purchased and includes additional features such as the ability to create RAR files.

The basic syntax for using unrar is as follows:

```
unrar x [option] file.rar
```

Where "x" is the command to extract files with full paths, "[option]" can be any additional options you want to use, and "file.rar" is the name of the RAR file you want to extract.

Some common options include:

- `-e`: Extract files without archived paths.
- `-p`: Print a file to stdout.
- `-v`: Verbosely list archive.
- `-x`: Extract files with full path.

For example, to extract a file named "archive.rar" with full paths, you would use the following command:

```
unrar x archive.rar
```

Unrar and RAR are related to the broader topic of data compression, which is a method of reducing the size of a file or data set. Data compression is used in many areas of computing, including file storage, data transmission, and memory allocation. There are two main types of data compression: lossless and lossy. RAR uses lossless compression, which means that the original data can be perfectly reconstructed from the compressed data.

Data compression algorithms often involve complex mathematical concepts and formulas. For example, Huffman coding, a common method of lossless data compression, involves the use of a binary tree to represent the most common characters in a data set with the shortest codes. However, the specific algorithms and formulas used in the RAR format are proprietary and not publicly disclosed.

> For more information on Unrar and related topics, you may want to check out the following resources:
> - [Unrar Manual](https://linux.die.net/man/1/unrar)
> - [Data Compression Explained](https://www.google.com/search?q=Data+Compression+Explained)
> - [Huffman Coding](https://www.google.com/search?q=Huffman+Coding)