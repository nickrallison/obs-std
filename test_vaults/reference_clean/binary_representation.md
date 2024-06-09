---
bad_links: 
aliases: []
tags: [computerarchitecture]
---
# Binary Representation

Binary representation is a number system that uses only two digits, 0 and 1, to represent all its values. It is the fundamental language of computers and digital systems because it aligns well with the on/off nature of electrical signals.

The binary number system is a positional system, meaning the position of each digit in a binary number represents a specific power of the base (2). The rightmost position is the least significant bit (LSB), representing $2^0$, and each position to the left represents a higher power of 2.

For example, the binary number 1011 represents the decimal number 11, calculated as follows:

$$
(1 \times 2^3) + (0 \times 2^2) + (1 \times 2^1) + (1 \times 2^0) = 8 + 0 + 2 + 1 = 11
$$

To convert a decimal number to binary, you can use the method of successive division by 2. For example, to convert the decimal number 13 to binary:

1. Divide 13 by 2, quotient = 6, remainder = 1 (LSB)
2. Divide 6 by 2, quotient = 3, remainder = 0
3. Divide 3 by 2, quotient = 1, remainder = 1
4. Divide 1 by 2, quotient = 0, remainder = 1 (MSB)

So, 13 in decimal is 1101 in binary.

Binary representation is also used to represent non-integer numbers, using binary fractions. For example, the binary number 101.101 represents the decimal number 5.625, calculated as follows:

$$
(1 \times 2^2) + (0 \times 2^1) + (1 \times 2^0) + (1 \times 2^{-1}) + (0 \times 2^{-2}) + (1 \times 2^{-3}) = 4 + 0 + 1 + 0.5 + 0 + 0.125 = 5.625
$$

Binary representation is also used to represent text, images, and other types of data in computers, using various encoding schemes such as ASCII, Unicode, and JPEG.

> For more information, you may want to read about [Binary number](https://www.google.com/search?q=Binary+number), [Binary-coded decimal](https://www.google.com/search?q=Binary-coded+decimal), and [Binary file](https://www.google.com/search?q=Binary+file).