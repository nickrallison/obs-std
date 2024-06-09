---
bad_links: 
aliases: [hi-z, high impedance]
tags: [computerarchitecture]
---
# High Impedance Logic Level

High Impedance Logic Level, also known as Hi-Z or tri-state logic, is a state in digital circuits where the output is neither high (logic 1) nor low (logic 0), but instead presents a high impedance to the connected circuitry. In this state, the output effectively disconnects from the circuit, allowing other devices to drive the signal line without interference.

The high impedance state is achieved using a tri-state buffer, which is a type of electronic circuit that can be enabled or disabled. When the tri-state buffer is enabled, it behaves like a regular buffer, passing the input signal to the output. However, when it is disabled, it enters the high impedance state, effectively disconnecting the output from the circuit.

The high impedance state is useful in situations where multiple devices need to share a common bus or signal line. By placing the output in a high impedance state, only one device can actively drive the line at a time, preventing conflicts and ensuring proper communication.

The high impedance state is often represented by the letter "Z" in digital logic diagrams. It is important to note that the high impedance state is not a logic level itself, but rather a state that allows for multiple logic levels to coexist on the same line.

The high impedance state can be mathematically represented using the following formula:

$$
V_{\text{out}} = Z_{\text{out}} \cdot I_{\text{out}}
$$

where:
- \($V_{\text{out}}$\) is the output voltage
- \($Z_{\text{out}}$\) is the output impedance
- \($I_{\text{out}}$\) is the output current

In practical circuits, the high impedance state is achieved by using a transistor-based tri-state buffer. When the buffer is enabled, the transistor acts as a low impedance path, allowing the signal to pass through. When the buffer is disabled, the transistor is turned off, creating a high impedance state.

Tangentially related items to consider when studying high impedance logic level include:
- Tri-state buffers and their operation
- Bus contention and arbitration
- Timing considerations when using high impedance logic

For more in-depth information and examples, you can refer to the following resources:
- [Tri-state logic on Wikipedia](https://en.wikipedia.org/wiki/Three-state_logic)
- [Tri-state buffer on All About Circuits](https://www.allaboutcircuits.com/technical-articles/tri-state-buffers-gates-electronics-tutorial/)
- [Bus contention on Electronics Notes](https://www.electronics-notes.com/articles/electronic_components/logic/bus-contention.php)

> I hope this explanation helps! If you have any further questions or need additional information, feel free to ask.