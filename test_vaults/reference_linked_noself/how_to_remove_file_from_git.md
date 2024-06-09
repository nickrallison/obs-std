---
aliases:
  - Remove File from Git
  - git remove file
tags:
  - coding
  - algorithms
bad_links:
---
# How to Remove File from [[Git.md|Git]]

Use `git rm`

If you want to remove the file from the [[Git.md|Git]] repository **and the filesystem**, use:

```bash
git rm file1.txt
git commit -m "remove file1.txt"
```

But if you want to remove the file only from the [[Git.md|Git]] repository and not remove it from the filesystem, use:

```bash
git rm --cached file1.txt
git commit -m "remove file1.txt"
```

And to push changes to remote repo

```bash
git push origin branch_name
```