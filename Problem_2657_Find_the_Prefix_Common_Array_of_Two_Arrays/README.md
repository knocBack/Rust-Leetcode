# Problem Title : 2657. Find the Prefix Common Array of Two Arrays

**Link to the problem:** [2657. Find the Prefix Common Array of Two Arrays](https://leetcode.com/problems/find-the-prefix-common-array-of-two-arrays)

## Problem Description

```
- Given two permutations A and B of length 'n'
- Find a prefix common array C which satisfies following conditions
    - C[i] = count of numbers which appeared in both A and B in [0,i] indices

- permutation of n implies all [1,n] numbers in any order occuring exactly once.
```

---

## Solution Explanation

### Approach
- C\[i] = count of numbers which appeared in both A and B in \[0,i] indices
- simply put, all the numbers appearing twice between \[0,i] indices are counted
- key observation: any number can appear atmost twice till ith index, because both A and B are permutations of n
- initializing a frequency counter for each integer and iterating through [0,i]
    - updating frequency while iterating it through all indices
    - since frequency hits '2' only once, we can thus increment count and update that count to the resultant vector in the end of each iteration

### Complexity Analysis

- **Time Complexity:** O(n)
- **Space Complexity:** O(2n+1)

---
