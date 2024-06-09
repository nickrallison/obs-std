---
bad_links: 
aliases: []
tags: [computerarchitecture]
---
# SystemVerilog Sensitivity List

The sensitivity list in SystemVerilog, a hardware description language, is a fundamental concept used in the description of hardware behavior. It is used in always blocks to specify the signals that, when they change, will trigger the execution of the block.

In SystemVerilog, an always block is a procedural block that is executed whenever there is a change in any of the signals in its sensitivity list. The syntax for an always block with a sensitivity list is as follows:

```systemverilog
always @(sensitivity list)
begin
    // statements
end
```

The sensitivity list is a comma-separated list of signals. When any of these signals change, the statements inside the always block are executed. For example:

```systemverilog
always @(clk, reset)
begin
    // statements
end
```

In this example, the always block will be executed whenever there is a change in either the `clk` (clock) signal or the `reset` signal.

There are also some special constructs that can be used in the sensitivity list:

- `posedge signal`: The block is sensitive to the positive edge (0 to 1 transition) of the signal.
- `negedge signal`: The block is sensitive to the negative edge (1 to 0 transition) of the signal.

For example:

```systemverilog
always @(posedge clk or negedge reset)
begin
    // statements
end
```

In this example, the always block will be executed on the rising edge of the `clk` signal or the falling edge of the `reset` signal.

SystemVerilog also introduces the `always_comb`, `always_latch`, and `always_ff` constructs, which automatically infer the sensitivity list, making the code more readable and less prone to errors.

- `always_comb`: This is used for [[Combinational Logic|combinational logic]]. The sensitivity list is automatically inferred and includes all the nets and variables that are read within the block.
- `always_latch`: This is used for latch inference. The sensitivity list is automatically inferred and includes all the nets and variables that are read within the block.
- `always_ff`: This is used for flip-flop inference. The sensitivity list is automatically inferred and includes all the edge-sensitive nets and variables that are read within the block.

For example:

```systemverilog
always_comb
begin
    // [[Combinational Logic.md|combinational logic]]
end

always_ff @(posedge clk or negedge reset)
begin
    // sequential logic
end
```

In the first example, the `always_comb` block will be executed whenever there is a change in any of the signals used in the block. In the second example, the `always_ff` block will be executed on the rising edge of the `clk` signal or the falling edge of the `reset` signal.

> For more in-depth information, you may want to refer to the ["IEEE Standard for SystemVerilog—Unified Hardware Design, Specification, and Verification Language"](https://standards.ieee.org/standard/1800-2017.html) or the ["SystemVerilog for Verification: A Guide to Learning the Testbench Language Features"](https://www.google.com/search?q=SystemVerilog+for+Verification%3A+A+Guide+to+Learning+the+Testbench+Language+Features) book.