---
aliases:
tags:
  - "computerarchitecture"
bad_links:
---
# SystemVerilog DFF

SystemVerilog DFF, or D flip flop, is a fundamental element in digital design used to store state information. It serves as a memory element that captures the value of the data input (D) at the moment of the rising or falling edge of the clock input (CLK) and then maintains that value until the next clock edge. D flip-flops are pivotal in the creation of sequential logic circuits, where the output and the function of the circuit depend on the sequence of data inputs.

The behavior of D flip-flops can be described precisely using SystemVerilog, a hardware description and verification language that extends Verilog with a richer set of constructs and capabilities for modeling, simulation, and validation of complex digital systems.

## Basic D Flip-FlopImplementation

In SystemVerilog, a D flip flop can be modeled as sensitive to either the rising edge (posedge) or the falling edge (negedge) of the clock signal. Here's a basic implementation of a DFF:

```systemverilog
module dff (
    input logic clk,    // Clock signal
    input logic rstn,   // Active low reset
    input logic d,      // Data input
    output logic q      // Output
);

always_ff @(posedge clk or posedge ares_reset) begin
    if (rstn)
        q <= 1'b0; // Reset the output to 0 on async reset
    else
        q <= d;    // On clock edge, capture the input
end

endmodule
```

## Important Implementation Note

Make sure the async reset has priority over any other symbols, if some other condition is met first in the if statement chain, a reset may not occur when it should.
## Enhancing the D Flip-Flop

The basic DFF can be extended or modified in numerous ways to fit specific requirements. For example, one might add functionality to:

- **Enable Input:** Helps control when the DFF captures data, offering an additional layer of control beyond just the clock and reset.
- **Asynchronous/Synchronous Set or Reset:** Allows for setting or resetting the flip-flop independently of the clock signal, either immediately (asynchronously) or at the next clock edge (synchronously).
- **Data Locking:** Prevents the DFF from updating its output until certain conditions are met, even if the clock and data inputs change.

### Adding an Enable Control

To introduce an enable signal `en`, the always block can be modified as follows:

```systemverilog
module dff_with_enable (
    input logic clk,
    input logic rstn,
    input logic d,
    input logic en,    // Enable control
    output logic q
);

always_ff @(posedge clk or negedge rstn) begin
    if (!rstn)
        q <= 1'b0;
    else if (en)
        q <= d; // Capture the input only when enabled
end

endmodule
```
