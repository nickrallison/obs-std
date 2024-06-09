---
aliases:
tags:
  - coding
  - linux
bad_links:
---
# How to Use Perl Like SED

## Piped Input

Where sed might be used like this with piped input:

```
command | sed 's/pattern/replace/g'
```

 Perl can be used exchangibly

```
command | perl -p -e 's/pattern/replace/g'
```

## In Place

Where sed might be used like this:

```
sed -i 's/pattern/replace/g'
```

Perl can be used exchangibly

```
perl -pi -e 's/pattern/replace/g'
```