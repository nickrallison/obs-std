---
bad_links: 
aliases: []
tags: [probability, algorithms]
title: Monte Carlo Methods
date created: Saturday, July 15th 2023, 4:03:26 pm
---
# Monte Carlo Methods

Monte Carlo methods are a broad class of computational algorithms that rely on repeated random sampling to obtain numerical results. The underlying concept is to use randomness to solve problems that might be deterministic in principle. They are often used when the system being modeled is complex with an intractable mathematical formula, or when it's just more convenient to perform simulations.

The name "Monte Carlo" was coined by scientists working on nuclear weapon projects in the U.S., in reference to the Monte Carlo Casino in Monaco where games of chance (i.e., involving random outcomes) are played.

The basic concept behind Monte Carlo methods is to generate a large number of "experiments" (i.e., random paths), calculate some function of the result of each experiment, and then take an average over these results. The Law of Large Numbers guarantees that this average will converge to the expected value of the function as the number of experiments goes to infinity.

Here's a simple example of a Monte Carlo method: estimating the value of $\pi$. We can do this by randomly throwing darts at a square dartboard with a circular target inscribed in it. If we throw $N$ darts and $M$ of them land inside the circle, then the ratio $M/N$ approximates the ratio of the areas of the circle and the square, which is $\pi/4$. Therefore, an estimate for $\pi$ is $4M/N$.

In mathematical notation, this can be written as:

$$
\pi \approx 4 \frac{M}{N}
$$

Monte Carlo methods are used in a wide variety of fields, including physics, engineering, statistics, finance, and computer graphics. They can be used to simulate the propagation of light in digital images, to model financial markets, to optimize complex systems, and to analyze large data sets, among many other things.

One important aspect of Monte Carlo methods is the generation of random numbers, or more generally, random variables. There are many algorithms for generating random numbers on a computer, but they all ultimately rely on a deterministic process, so they are called pseudorandom number generators. The quality of a Monte Carlo simulation can depend critically on the quality of the pseudorandom number generator.

Another important aspect is the estimation of error, or how far off we expect the estimate to be. This is typically done using statistical methods, and it's important to remember that the error decreases as the square root of the number of experiments, not linearly. So to get twice the precision, you need to do four times as many experiments.

> For more in-depth understanding, you may want to read the following resources:
> - ["Monte Carlo method"](https://www.google.com/search?q=Monte+Carlo+method) on Wikipedia
> - ["Monte Carlo Methods"](https://www.google.com/search?q=Monte+Carlo+Methods) on Stanford Encyclopedia of Philosophy
> - ["Understanding the Monte Carlo Simulation: Examples and Applications"](https://www.google.com/search?q=Understanding+the+Monte+Carlo+Simulation%3A+Examples+and+Applications) on Investopedia
> - ["Monte Carlo methods in statistical physics"](https://scholar.google.com/scholar?q=Monte+Carlo+methods+in+statistical+physics) by Newman and Barkema, a comprehensive textbook on the subject.