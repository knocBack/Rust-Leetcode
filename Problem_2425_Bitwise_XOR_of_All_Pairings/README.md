# Problem Title : 2425. Bitwise XOR of All Pairings

**Link to the problem:** [2425. Bitwise XOR of All Pairings](https://leetcode.com/problems/bitwise-xor-of-all-pairings/)

## Problem Description

```
- Given two 0-indexed non-negative integer arrays -> nums1, nums2
- there exists another array nums3, which contains XOR of all pairings between nums1 and nums2
- return bitwise XOR of all integers in nums3
```

---

## Solution Explanation

### Approach
- nums3 is nothing but a matrix multiplication of nums1 and nums2 transpose
- simpler ways it can be represented this way
nums1 = \[1 2 3], size = n1 = 3
nums2 = \[4 5], size = n2 = 2
nums2T = \[ 4
            5 ]
nums3 = nums2T X nums1 = \[4^1 4^2 4^3
                           5^1 5^2 5^3], size = n3 = 2*3 = 6
- flattening above matrix is our actual nums3 matrix
- key observations to note:
    - all the numbers in nums2 gets repeated n1 number of times, similarly nums1 are repeated n2 times
    - using the logic if a number is xor'ed even times, resultant is zero, xor'ed odd times, resultant is number itself, we can find that if n1 == even, n2 numbers won't contribute in nums3 xor, and vice versa
    - so if both n1 and n2 are odd, then xor of nums1 and nums2 will be the answer! if both are even, it's zero!


### Complexity Analysis
- **Assumptions:** n1 = nums1.size(), n2 = nums2.size()
- **Time Complexity:** O(n1+n2)
- **Space Complexity:** O(1)

---
