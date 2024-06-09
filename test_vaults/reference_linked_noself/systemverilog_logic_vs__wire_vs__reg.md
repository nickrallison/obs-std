---
aliases: []
tags: [computerarchitecture]
bad_links:
---
# SystemVerilog Logic vs. Wire vs. Reg

SystemVerilog is a hardware description language (HDL) that is widely used in the field of computer architecture and digital design. It enhances and extends the capabilities of its predecessor, Verilog, by providing a richer set of data types and more flexible modeling constructs. Among the most commonly used data types in SystemVerilog are `logic`, `wire`, and `reg`. Understanding the differences between these types is crucial for designing efficient and accurate digital systems. This discussion will clarify these differences in the context of their usage within digital design and simulation.

## Wire

- **Definition**: A `wire` in SystemVerilog is used to represent connections between different hardware elements. Wires are used for combinatorial connections and cannot store a value (i.e., they cannot hold a state). Their value is determined by the logic-driver attached to them, such as a continuous assignment or the output of a module.

- **Usage**: Use `wire` when you want to connect outputs of modules, gates, or for continuous assignments. Wires are purely used for signal interconnections.

- **Example**: Connecting the output of a decoder to several multiplexers.

## Reg

- **Definition**: The term `reg` does not literally relate to a hardware register. In SystemVerilog, a `reg` can be used to store a value across simulation time steps. `Reg` data types are synthesizable and can be mapped to actual hardware registers in an [[FPGA|FPGA]] or ASIC design, but within simulation, they represent variables that can hold a value over periods of time.

- **Usage**: Use `reg` when you need to store values or when modeling sequential logic. Although its name suggests register-based storage, within simulation, it also serves to represent variables that can retain values across procedural assignments.

- **Example**: Storing the state of a [[Finite State Machines.md|finite state machine]].

- **Definition**: The `logic` data type was introduced in SystemVerilog to combine the capabilities of both `wire` and `reg`. A `logic` type variable can be driven by a continuous assignment or assigned to within an always block, making it versatile for both combinatorial and sequential logic.

- **Usage**: Use `logic` when you need a single variable that can be driven both procedurally (like `reg`) and by continuous assignments (like `wire`). It is the preferred type for signal declaration in modern SystemVerilog code because of its flexibility.

- **Example**: A signal in a module that can be assigned a value both through procedural blocks and through continuous assignment statements.

## Key Takeaways

1. **Wire**: Used for signal interconnections, cannot hold a state, driven by continuous assignments or outputs of modules.
2. **Reg**: Used for modeling variables that can hold values across time steps, useful in sequential logic.
3. **Logic**: A versatile type that can act as both `wire` and `reg`, recommended for use in most situations due to its flexibility.

The choice between `wire`, `reg`, and `logic` fundamentally depends on the nature of the signal being modeled - whether it needs to hold a value, how it is driven, and the level of flexibility required in its use. With the introduction of `logic`, SystemVerilog has greatly simplified the decision-making process for digital designers, promoting a more unified approach to HDL coding.