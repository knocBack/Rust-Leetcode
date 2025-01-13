# Problem Title : 3223. Minimum Length of String After Operations

**Link to the problem:** [3223. Minimum Length of String After Operations](https://leetcode.com/problems/minimum-length-of-string-after-operations/)

## Problem Description

```
- Given string s
- Find minimum length of final string s after any number of operations allowed.
- operations allowed:
    - Choose and index i, assume c = s[i]
    - delete left closest c to i
    - delete right closest c to i
```

---

## Solution Explanation

### Approach
- Key point: No need to find the minimum length string, just need it's length
- Key observation: if a character c is repeating odd number of times, after n operations, only 1 will be left
- similarly if c is even number of times, only 2 will be left after m operations
- hence create cnt array for each letter
- if cnt is odd add 1 to ans
- else add 2
- which gives the minimum length of the string.

### Complexity Analysis
- **Assumptions:** n = s.len()
- **Time Complexity:** O(n + 26)
- **Space Complexity:** O(n)

---
