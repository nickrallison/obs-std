---
bad_links: 
aliases: []
tags: [operatingsystems, linux]
---
# Shebang

The Shebang (`#!`) is a character sequence consisting of a hash and an exclamation mark that is used in scripts to indicate an interpreter for execution under UNIX and UNIX-like operating systems. It is also known as a hashbang, pound-bang, or shabang. 

The Shebang is the first line of a script and looks something like this:

```bash
#!/bin/bash
```

In this example, `#!/bin/bash` tells the system that this script should be run using the Bash shell. The path after the `#!` is the path to the interpreter that should be used to interpret the rest of the lines in the text file. 

The Shebang is not a part of the scripting languages themselves, but it's a directive to the operating system. It's a special kind of comment. When the operating system sees this, it treats the rest of the file as a script to be interpreted by the interpreter specified by the Shebang.

The Shebang must be the very first thing in the file, with no blank lines before it. Also, it doesn't work if there are spaces between `#!` and the path.

The Shebang is used in various scripting languages that are interpreted such as Python, Perl, Ruby, PHP, and others. For example, a Python script might start with:

```python
#!/usr/bin/python3
```

This tells the system to execute the script using Python 3.

> For more information, you can refer to the following resources:
> - [Shebang (Unix)](https://www.google.com/search?q=Shebang+(Unix))
> - [Why do people write #!/usr/bin/env python on the first line of a Python script?](https://www.google.com/search?q=Why+do+people+write+#!/usr/bin/env+python+on+the+first+line+of+a+Python+script?)
> - [The #! magic, details about the shebang/hash-bang mechanism on various Unix flavours](https://www.google.com/search?q=The+#!+magic,+details+about+the+shebang/hash-bang+mechanism+on+various+Unix+flavours)