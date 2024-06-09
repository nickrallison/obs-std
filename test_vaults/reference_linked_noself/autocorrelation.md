---
bad_links: 
aliases: []
tags: [probability]
---
# Autocorrelation

Autocorrelation, also known as serial correlation, is a statistical property where the correlation between the elements of a series of numbers is measured based on their time or spatial relationships. It is often used in time series analysis to detect non-randomness or to identify periodic patterns.

The autocorrelation function (ACF) is a tool for identifying repeating patterns, such as the presence of a periodic signal obscured by noise, or identifying the missing fundamental frequency in a signal implied by its harmonic frequencies. It is often used in signal processing for analyzing functions or series of values, such as time domain signals.

The formula for autocorrelation at lag $k$ for a time series $Y$ is:

$$
\rho_k = \frac{\sum_{t=k+1}^{T}(Y_t - \bar{Y})(Y_{t-k} - \bar{Y})}{\sum_{t=1}^{T}(Y_t - \bar{Y})^2}
$$

where:
- $\rho_k$ is the autocorrelation at lag $k$,
- $Y_t$ is the value of the time series $Y$ at time $t$,
- $\bar{Y}$ is the mean of the time series $Y$, and
- $T$ is the length of the time series.

The numerator of the formula is the [[Covariance|covariance]] of the time series with itself at lag $k$, and the denominator is the [[Variance|variance]] of the time series. Thus, autocorrelation is essentially a correlation coefficient, but instead of correlating $Y$ to another variable $X$, we are correlating $Y$ to itself at a different time.

Autocorrelation can range from -1 to 1. A positive autocorrelation indicates that a time series' elements increase or decrease together, while a negative autocorrelation indicates that a time series' elements show an alternating pattern.

Autocorrelation can be used to detect trends, cycles, or seasonal patterns in the data. For example, if the autocorrelation is high for a lag of 1, it suggests that the data have a strong trend. If the autocorrelation is high for a lag of 12, it suggests that the data have a yearly seasonal component.

> For more in-depth understanding, you can refer to the following resources:
> - [Autocorrelation — Wikipedia](https://www.google.com/search?q=Autocorrelation+Wikipedia)
> - [Autocorrelation: Definition, Formula, Example](https://www.google.com/search?q=Autocorrelation%3A+Definition%2C+Formula%2C+Example)
> - [Understanding Autocorrelation](https://www.google.com/search?q=Understanding+Autocorrelation)
> - [Autocorrelation in Time Series Data](https://www.google.com/search?q=Autocorrelation+in+Time+Series+Data)