# Advent of Code 2022 - Day 3

## Task 1

Parse the input file and identify duplicates.\
This exercise is case-sensitive.

* Lowercase and uppercase characters compose lines of text.
* Each line is divided in two: their first and second halves.
* A character is considered a duplicate if it is found in both the first and the
second half of the line.
* A character is not considered a duplicate if it is found multiple times in
a single half of the line.
* Lowercase characters `a` to `z` have values from `1` through `26`.
* Uppercase characters `A` to `Z` have values from `27` through `52`.

Find the duplicate for each line, retrieve its value, and calculate the sum
of all the duplicates' values in the input file.

## Example

```
vJrwpWtwJgWrhcsFMMfFFhFp
PmmdzqPrVvPwwTWBwg
```
Lowercase `p` is found in both the first and second half of line 1.\
It has a value of `16`.\
Uppercase `P` is found in both the first and second half of line 2.\
It has a value of `42`.\
The sum is `16 + 42 = 58`.
