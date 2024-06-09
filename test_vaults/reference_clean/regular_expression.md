---
bad_links: 
aliases: [regex]
date created: Monday, June 26th 2023, 3:32:29 pm
tags: [algorithms, coding]
title: RegEx
---

# Regular Expression

A system used to find and capture regular patterns in text.

an example Regex string in Python looks like:
```python
regex = r"[\w]+@[\w]+.[\w]+"
```
Which would match any email

## Characters

\\w - matches any word character, (equivalent to \[a-zA-Z0-9_\])  
\\W - matches any non word character, (equivalent to \[^a-zA-Z0-9_\]) \\d - matches any digit character, (equivalent to [0-9])  
\\D - matches any non-digit character, (equivalent to \[^0-9\])  
\\s - matches any whitespace character, including spaces, tabs, and line breaks.  
\\S - matches any non-whitespace character.  
\\b - matches a word boundary. This means it will match the position where a word character is not followed or preceded by another word-character.  
\\B - matches a non-word boundary. This means it will match the position where a word character is followed or preceded by another word-character.  
\\A - matches the start of the input.  
\\Z - matches the end of the input.  
\\G - matches the point where the last match finished.
