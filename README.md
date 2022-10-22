# two_smallest_integers
This program was made to try to make a rust lang project for the first time.

### Usage

Type any number from `1` to `2^64-1` or `18446744073709551615`. Typing `0` will exit the program.

This program tries to find integers `a` and `b` such that `a*b=n`, where `n` is the number you have typed in the prgram. This program tries to find the minimum absolute value of `|a-b|`.

The program requires pressing enter when it finishes finding the factors for the integer `n`, and when it multiplies different combinations of the factors of `n` together to get the minimum of `|a-b|`. The results are printed after pressing enter.

Pressing enter if the program is still finding the factors/combinations will cancel the program to the start.