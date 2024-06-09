---
bad_links:
aliases: [resonance]
tags: [controlsystems, signalprocessing]
---
# Resonance Peak

A resonance peak typically refers to the highest point or maximum amplitude in a system's response to a varying frequency. It occurs when the frequency of an external force matches the natural frequency of the system, causing an increase in the amplitude of oscillations or vibrations. This concept is widely applied in various fields such as physics, engineering, and electronics.

A good example of resonance peaks are in control theory. A control system has a resonance peak when the system's output response reaches a maximum value for a particular input frequency. This peak is often a result of the system’s natural frequency coinciding with the input frequency, leading to an amplification of the output signal. The size and sharpness of the resonance peak can provide valuable [[Information Theory|information]] about the [[LTI System Stability|stability]] and performance of the control system. A larger and sharper peak typically indicates that the system is more sensitive to changes in input frequency and may be less stable or have slower response times.

Resonance peaks are a crucial consideration when designing control systems, particularly in industries such as electronics, aerospace, and mechanical engineering. Ignoring these peaks can lead to system instability or even catastrophic failure in certain circumstances. By using techniques such as dampening or tuning, engineers can manage resonance peaks to optimize system performance and ensure [[LTI System Stability|stability]]. However, it's essential that these measures be carefully calibrated to avoid compromising overall functionality.

If a system can be modelled as a [[Transfer Function|transfer function]] with dominant poles at $\alpha \pm  \beta$. The resonance peak frequency and magnitude can be found with the following formulas:
$$
\omega_r = \sqrt{\beta^2 - \alpha^2} \text{ if } \beta \ge \alpha \text{, else no resonance peak}
$$

$$
M_r = \frac{\alpha^2 + \beta^2}{2\alpha\beta}
$$
