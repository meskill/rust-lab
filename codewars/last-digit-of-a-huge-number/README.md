# Last digit of a huge number

[codewars link](https://www.codewars.com/kata/5518a860a73e708c0a000027/train/rust)

For a given list [x1, x2, x3, ..., xn] compute the last (decimal) digit of x1 ^ (x2 ^ (x3 ^ (... ^ xn))).


Beware: powers grow incredibly fast. For example, 9 ^ (9 ^ 9) has more than 369 millions of digits. lastDigit has to deal with such numbers efficiently.

Corner cases: we assume that 0 ^ 0 = 1 and that lastDigit of an empty list equals to 1

## Example

```rust
last_digit({3, 4, 2}, 3) == 1
because 3 ^ (4 ^ 2) = 3 ^ 16 = 43046721.
```
