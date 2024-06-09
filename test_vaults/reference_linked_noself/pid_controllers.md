---
bad_links:
aliases: []
date created: Monday, June 26th 2023, 3:32:29 pm
tags:
  - controlsystems
  - algorithms
title: PID Controllers
---

# PID Controllers

PID (Proportional-Integral-Derivative) controllers are a type of feedback controller widely used in control systems. They are used to correct the error between a measured process variable and a desired setpoint by calculating and then outputting a corrective action that can adjust the process accordingly.

The PID controller is named after its three correcting terms, whose sum constitutes the manipulated variable (MV):

1. **Proportional**: The proportional term produces an output value that is proportional to the current error value. The proportional response can be adjusted by multiplying the error by a constant Kp, known as the proportional gain constant.
    - The proportional term is given by $P_{out} = K_p e(t)$, where $e(t)$ is the error at time $t$.

2. **Integral**: The integral term is proportional to both the magnitude of the error and the duration of the error. The integral response will continually increase over time unless the error is zero, thus driving the [[Steady-State Error|Steady-State error]] towards zero. The integral term is given by $I_{out} = K_i \int e(t) dt$, where Ki is the integral gain.
    - The integral term is given by $I_{out} = K_i \int e(t) dt$, where $K_i$ is the integral gain.

3. **Derivative**: The derivative term is proportional to the rate of change of the error. This means the controller output is influenced by the rate at which the error is changing. The derivative response is given by $D_{out} = K_d \frac{de(t)}{dt}$, where Kd is the derivative gain.
    - The derivative term is given by $D_{out} = K_d \frac{de(t)}{dt}$, where $K_d$ is the derivative gain.

The PID controller formula is given by:

$$
MV(t) = K_p e(t) + K_i \int e(t) dt + K_d \frac{de(t)}{dt}
$$

Where:
- $MV(t)$ is the output of the PID controller.
- $e(t)$ is the error signal, which is the difference between the desired setpoint and the measured process variable.
- $K_p$, $K_i$, and $K_d$ are the proportional, integral, and derivative gains, respectively.

Tuning a PID controller involves adjusting the gain parameters $K_p$, $K_i$, and $K_d$ to achieve the desired system response. There are several methods for PID tuning, such as the Ziegler-Nichols method, Cohen-Coon method, and manual tuning.

Tangentially related items include the types of PID controllers (P, PI, PD, PID), the concept of [[LTI System Stability|system stability]], overshoot, undershoot, and [[Steady-State Error|steady-state error]], and the s-domain representation of PID controllers in [[Laplace Transform|Laplace transforms]].

> For more in-depth reading, you may refer to the following resources:
> - [PID controller (Wikipedia)](https://www.google.com/search?q=PID+controller+site:wikipedia.org)
> - [PID Controller Tuning: A Short Tutorial (Jinghua Zhong, McMaster University)](https://www.google.com/search?q=PID+Controller+Tuning%3A+A+Short+Tutorial+Jinghua+Zhong)
> - [PID Control (National Instruments)](https://www.google.com/search?q=PID+Control+site:ni.com)