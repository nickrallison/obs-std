---
bad_links: 
date created: Monday, June 26th 2023, 3:32:28 pm
tags: [finance, calculus]
title: Annuity
aliases: []
---
# Annuity

An annuity is a regular payment with compound interest

$$
PV = P\frac{1-(1+r)^{-n}}{r}
$$
$$
FV = P\frac{(1+r)^{n}-1}{r}
$$

It can be derived via the following differential equation in latex formatting:

$$
\frac{dV}{dt} = rV - C
$$

where V(t) is the value of the annuity at time t, C is the payment made per time period, and r is the interest rate. 
## Annuity Due

the difference with an annuity due is that the initial payment is due immediately as opposed to one payment period after

$$
PV = P\frac{1-(1+r)^{-n}}{r}(1-r)
$$
$$
FV = P\frac{(1+r)^{n}-1}{r}(1-r)
$$
