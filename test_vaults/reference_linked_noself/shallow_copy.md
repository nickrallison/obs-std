---
bad_links: 
aliases: []
tags: [coding]
---
# Shallow Copy

Shallow copy refers to the process of creating a new object and then copying the non-static fields of the current object to the new object. If a field is a value type, a bit-by-bit copy of the field is performed. If a field is a reference type, the reference is copied but the referred object is not; therefore, the original object and its clone refer to the same object. It's a method of copying where only one level [[Deep Copy|deep copy]] takes place and deeper levels are referenced rather than copied.