---
aliases:
  - change env vars
tags:
  - linux
bad_links:
---
# How to Change Environment Vars

## Bash

In Bash, you can change [[Environment Variables.md|environment variables]] for the current session by using the `export` command. The variable will be available for any sub-process started from this shell.

```bash
export VARIABLE_NAME="value"
```

To make the change permanent, you can add the `export` command to your `~/.bashrc` or `~/.bash_profile` file.

## CSH

In the Csh shell, you can set environment variables using the setenv command.

```bash
setenv VARIABLE_NAME value
```

To make the change persist across sessions, add the `setenv` command to your `~/.cshrc`.

## ZSH

Zsh, being somewhat similar to Bash in handling environment variables, allows you to use the `export` command as well.

```zsh
export VARIABLE_NAME="value"
```

For those changes to be permanent, place them in your `~/.zshrc` file.

## Fish

Fish shell uses a different syntax for setting environment variables. To set a variable for the current session, use the `set` command with the `-x` option.

```bash
set -x VARIABLE_NAME value
```

To make a variable persist across sessions, add the `set` command to your `~/.config/fish/config.fish` file.
