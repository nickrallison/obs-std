---
bad_links: 
aliases: []
tags: [computerarchitecture]
---
# SystemVerilog Number Format Notes

## Notes

Can typecast between signed and unsigned  
```
logic line = 10’sb0011;  
unsigned’ line;
```

—

Can make arrays signed

```
logic[7:0] signed array;
array = 1'b1;  // 8'b0000_0001
array = 8'b1;  // 8'b0000_0001
array = 1'sb1; // 8'b1111_1111
array = 8'sb1; // 8'b0000_0001
array = '1;    // 8'b1111_1111
```

—

Net data type  
wire keyword
```
wire logic[15:0] val;
```

Can’t be driven from an always block, must be driven by continuous assignment like module port  
can be bidirectional

—

var data type  
storage of local value inside module
```
var logic[15:0] val;
```

Always or continuous assignment

–  
Always defaults to unsigned. Can make it signed with a signed declaration
```
var logic[15:0] signed val;
```