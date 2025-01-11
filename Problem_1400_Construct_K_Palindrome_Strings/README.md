# Problem Title : <Problem_name>

**Link to the problem:** [1400. Construct K Palindrome Strings](https://leetcode.com/problems/construct-k-palindrome-strings/)

## Problem Description

```
- Given a string s and an integer k
- if by using all the characters of s we can make k palindorme strings
    - return true
- else return false
```

---

## Solution Explanation

### Approach
- Key observation is that maximum palindromic strings that can be formed are s.len()
- Minimum palindromic stirngs that must be split into, can be found by finding odd number of letters
- Because if a letter is repeated odd number of times, it must be in a separate string! And even number of letters don't matter, they always make the split lie in between the min and max.
- To find the minimum limit, simply hash the string with a vector 26 size (26 letters), get the count, then count the odd times repeating character.

### Complexity Analysis
- **Assumptions:** n = s.len()
- **Time Complexity:** O(n + 26)
- **Space Complexity:** O(26)

---
