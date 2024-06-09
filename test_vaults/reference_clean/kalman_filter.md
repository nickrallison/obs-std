---
bad_links: 
aliases: []
date created: Monday, June 26th 2023, 3:32:29 pm
tags: [signalprocessing, controlsystems, robotics]
title: Kalman Filter
---
# Kalman Filter

The Kalman Filter is an algorithm that uses a series of measurements observed over time, containing statistical noise and other inaccuracies, and produces estimates of unknown variables that tend to be more accurate than those based on a single measurement alone. It is used in a wide range of engineering and econometric applications from radar and computer vision to stock market prediction.

The Kalman Filter operates recursively on streams of noisy input data to produce a statistically optimal estimate of the underlying system state. The filter is named after Rudolf E. Kálmán, one of the primary developers of its theory.

The Kalman Filter has two basic steps: Prediction and Update.

1. **Prediction**: In this step, the Kalman Filter produces estimates of the current state variables, along with their uncertainties. Once the outcome of a future (predicted) event has been observed, these estimates are updated using the new measurements.

2. **Update**: In this step, the Kalman Filter uses a weighted average, with more weight being given to estimates with higher certainty. The purpose of the weighting is that estimates with uncertainty are given less weight than estimates with high certainty.

The mathematical representation of the Kalman Filter involves matrices because the filter processes all of the measurements simultaneously in a batch.

The basic Kalman Filter structure for a linear system with additive Gaussian noise is as follows:

**Prediction:**

State estimate:
$$
\hat{x}_{k|k-1} = A\hat{x}_{k-1|k-1} + Bu_{k}
$$

Error covariance:
$$
P_{k|k-1} = AP_{k-1|k-1}A^T + Q
$$

**Update:**

Kalman gain:
$$
K_k = P_{k|k-1}H^T(HP_{k|k-1}H^T + R)^{-1}
$$

State estimate:
$$
\hat{x}_{k|k} = \hat{x}_{k|k-1} + K_k(z_k - H\hat{x}_{k|k-1})
$$

Error covariance:
$$
P_{k|k} = (I - K_kH)P_{k|k-1}
$$

Where:
- $x$ is the true state
- $\hat{x}$ is the estimated state
- $P$ is the state covariance
- $K$ is the Kalman gain
- $A$ is the state transition model
- $B$ is the control-input model
- $H$ is the observation model
- $Q$ is the process noise covariance
- $R$ is the observation noise covariance
- $z$ is the actual measurement
- $u$ is the control vector
- $I$ is the identity matrix

The Kalman Filter assumes that both the system dynamics and the measurement dynamics are linear and Gaussian. This means that the system can be modeled in terms of a matrix operation, and that all noise is Gaussian and additive. If these assumptions are not true, we may need to use an extension of the Kalman Filter, such as the Extended Kalman Filter (EKF) or the Unscented Kalman Filter (UKF).

> For more in-depth understanding, you can refer to the following resources:
> - [Understanding the Basis of the Kalman Filter Via a Simple and Intuitive Derivation](https://www.google.com/search?q=Understanding+the+Basis+of+the+Kalman+Filter+Via+a+Simple+and+Intuitive+Derivation)
> - [An Introduction to the Kalman Filter](https://www.google.com/search?q=An+Introduction+to+the+Kalman+Filter)
> - [Kalman Filter book](https://www.google.com/search?q=Kalman+Filter+book)