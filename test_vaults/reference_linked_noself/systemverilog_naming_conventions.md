---
aliases: 
tags:
  - computerarchitecture
bad_links:
---
# SystemVerilog Naming Conventions

SystemVerilog naming conventions are a set of guidelines that help in writing code that is more readable, maintainable, and consistent across large projects and teams. Following a consistent naming convention helps in understanding the purpose of variables, modules, and other elements in the codebase without needing to deeply analyze their implementations. Here we will expand on various aspects of naming conventions in SystemVerilog, building upon the foundational introduction.

## Types of Identifiers

SystemVerilog identifiers typically fall into several categories, each serving a different purpose in the design and verification environment:

- **Modules and Interfaces:** Represent hardware blocks or interfaces between blocks.
- **Variables and Constants:** Hold values that can change over time (variables) or remain constant.
- **Functions and Tasks:** Encapsulate reusable logic.
- **Parameters:** Define constants that parameterize modules and interfaces.
- **Types:** Define new data types or type aliases.

## Naming Guidelines

### Modules and Interfaces

It is common to start the names of modules and interfaces with a capital letter and use CamelCase for additional words. For instance, `EthernetController` or `UsbInterface`. This makes it clear when a particular identifier refers to a significant component.

```systemverilog
module EthernetController;
    // Module definition
endmodule

interface UsbInterface;
    // Interface definition
endinterface
```

### Variables and Constants

Variables often use `lowerCamelCase`, starting with a lowercase letter and capitalizing subsequent words. Constants are usually written in `UPPER_CASE`, separated by underscores, making them distinct from variables.

```systemverilog
int readCount;
localparam int MAX_TIMEOUT = 1000;
```

### Functions and Tasks

Functions and tasks, like modules, often start with an uppercase letter to denote their importance and utility within the code, using CamelCase thereafter.

```systemverilog
function void CalculateChecksum;
    // Function body
endfunction

task InitializeHardware;
    // Task body
endtask
```

### Parameters and Localparams

Parameters and local parameters are generally written in `UPPER_CASE`, similar to constants, as they define fixed values used to parameterize modules. This convention helps differentiate them from regular variables or instance-specific constants.

```systemverilog
parameter int DEFAULT_TIMEOUT = 50;
localparam int MAX_WIDTH = 1024;
```

### Types

New types or type aliases often follow the CamelCase convention, starting with a capital letter, especially for complex types such as `struct`, `union`, and `typedef`.

```systemverilog
typedef struct packed {
    logic [7:0] address;
    logic [31:0] data;
} MemoryRequest;
```

## General Recommendations

- **Prefixes/Suffixes:** To enhance readability, some teams use specific prefixes or suffixes. For example, using `_n` for active-low signals (`reset_n`) or `i_`/`o_` for input/output signals in modules (`i_data`, `o_ack`).
- **Avoid Ambiguities:** Names should be descriptive yet concise to avoid ambiguity but also to prevent overly long identifiers that are hard to read.
- **Consistency:** Perhaps the most crucial aspect of adopting a naming convention is consistency. The entire team should adhere to the agreed-upon conventions to ensure the codebase remains coherent and maintainable.

By adhering to these naming conventions, teams can significantly improve the readability and maintainability of their SystemVerilog codebases, making it easier for new developers to understand and contribute to the project.