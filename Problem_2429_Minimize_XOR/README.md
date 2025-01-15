# Problem Title : 2429_Minimize_XOR

**Link to the problem:** [2429_Minimize_XOR](https://leetcode.com/problems/minimize-xor/)

## Problem Description

```
- Given positive integers, num1 and num2
- find positive integer n such that
    - n has same number of set bits as num2
    - value of n XOR num1 is minimal
```

---

## Solution Explanation

### Approach
- to minimize the xor, we first need to add all the higher bits or most significant bits of num1 to n
- if we set all the num1 bits in n, xor is hence 0 and minimal
- but in some cases that is not enough as num2 might have more bits, then set all the not set least significant bits for n
- hence we get the lowest xor possible or minimal xor

### Complexity Analysis

- **Time Complexity:** O(1)
- **Space Complexity:** O(1)

---
