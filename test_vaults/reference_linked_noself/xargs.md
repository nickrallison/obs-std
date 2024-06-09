---
aliases:
tags:
  - linux
bad_links:
---
# Xargs

Xargs is a powerful command-line utility in Linux that allows you to build and execute command lines from standard input. It is most commonly used in combination with other commands through the use of pipes. Xargs reads items from the standard input (stdin), delimited by blanks (which can be protected with double or single quotes or a backslash) or newlines, and executes a specified command on each item. The command that xargs executes is specified as an argument to xargs itself.

## How Xargs Works

At its core, xargs is about efficiency and flexibility. Instead of running a single command for each input item, xargs bundles as many inputs as possible into a single command execution, thereby reducing the overhead of starting new processes. This is particularly useful for commands that do not natively support taking input from a pipeline.

For instance, if you have a list of files and you want to perform an operation on each file, like compressing files or changing permissions, you might typically use a loop. However, xargs can handle this more efficiently by reducing the number of times the command is called by passing multiple files at a time to the command.

## Basic Usage

Here's a simple example of how to use xargs. Suppose you want to remove a list of files:

```bash
echo "file1.txt file2.txt file3.txt" | xargs rm
```

In this case, `echo` provides a list of filenames as input to xargs, which in turn passes them as arguments to the `rm` command, resulting in the files being removed.

## Run Once for Each Line

Normally xargs adds each line of input to the ran command, if you want to run each line as a separate command, run this:

```bash
find . -name "*.txt" | xargs -L 1 [command]
```

## Examples

Continuing from the basic usage, let's explore more examples of how xargs can be utilized in different scenarios.

### Finding Files and Removing Them

One common use case for xargs is in conjunction with the `find` command. For example, if you want to find all `.txt` files and remove them, you can use:

```bash
find . -name "*.txt" | xargs rm
```

Here, `find` outputs a list of `.txt` files in the current directory (and subdirectories), and `xargs` passes them to `rm` to be deleted.

### Using Xargs with [[Grep|Grep]]

Another powerful example is using xargs with `grep` to search within multiple files. Suppose you want to find occurrences of the string "example" in all `.txt` files:

```bash
find . -name "*.txt" | xargs grep "example"
```

This command first finds all `.txt` files, then `grep` searches within those files for the string "example".

### Handling Spaces in Filenames

By default, xargs treats spaces as delimiters, which can be problematic with filenames that contain spaces. To handle this, you can use the `-print0` option with `find` and the `-0` option with xargs:

```bash
find . -name "*.txt" -print0 | xargs -0 grep "example"
```

This combination ensures that filenames with spaces are correctly handled as single arguments.

### Customizing Delimiters

You can also customize the input delimiter for xargs using the `-d` option. For example, if you have a list of files separated by newlines, you might use:

```bash
echo -e "file1.txt\nfile2.txt\nfile3.txt" | xargs -d '\n' rm
```

### Running Commands in Parallel

Xargs can also run multiple instances of a command in parallel, which can be particularly useful for speeding up tasks across multiple files or inputs. The `-P` option specifies the number of parallel processes:

```bash
find . -name "*.png" | xargs -P 4 -I {} convert {} {}.jpg
```

This command finds all `.png` files and uses ImageMagick's `convert` command to parallelly convert them into `.jpg` format, running 4 conversions at a time.

## Further Tips

- **Use `-n` option** with xargs to limit the number of arguments passed to each command invocation. This can be useful for commands that have a limit on how many arguments they can accept.
- **Debugging:** If you're unsure what xargs will do, especially with complex commands or patterns, you can replace the command with `echo`. This will print the commands that would be executed without actually running them.

By mastering xargs and its options, you can significantly streamline and optimize your workflows in Linux, making your command-line usage more efficient and powerful.