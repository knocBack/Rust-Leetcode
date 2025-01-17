# Problem Title : 2683. Neighboring Bitwise XOR

**Link to the problem:** [2683. Neighboring Bitwise XOR](https://leetcode.com/problems/neighboring-bitwise-xor/)

## Problem Description

```
- Given an array of 0's and 1's named "derived" of length n
- it is derived by computing the bitwise XOR of adjacent values of another binary array original of length n,  following cyclic rotation
    - namely, if i == n-1, adjacent = 0; else adjacent = i+1; in original array
- return if the derived array has any valid binary array original from which it got derived
```

---

## Solution Explanation

### Approach
- on index i, derived\[i] = original\[i] ^ original\[i+1]
- on index i+1, derived\[i+1] = original\[i+1] ^ original\[i+2]
- derived\[i]^derived\[i+1] = original\[i] ^ original\[i+2]
- since it's cyclic, if you continue xoring all the elements from 0 till n-1, every element in original will be repeated twice, hence xor will become zero!
- hence, if all the element's xor in derived is zero, implies it had a valid original array, if not, it wasn't derived properly!


### Complexity Analysis
- **Time Complexity:** O(n)
- **Space Complexity:** O(1)

---
