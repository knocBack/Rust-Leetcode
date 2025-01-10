# Problem Title : 916. Word Subsets

**Link to the problem:** [916. Word Subsets](https://leetcode.com/problems/word-subsets/)

## Problem Description

```
- Given 2 string arrays words1 & words2
- Find all universal strings in words1
- String A is universal if all strings in words2 are subsets of A
- String B is subset of A if every letter in B is in string A (including multiplicity implies >= count of letter).
```

---

## Solution Explanation

### Approach
- Firstly, it's a keen observation that bruteforce doesn't work. And there's a repetition while doing bruteforce solution
- Max count of a letter in all of the strings combined for a single character is all we need to compute. It drastically reduces complexity from O(N) to O(26) for that run.
- How to do that? make a hashmap for each character and update all of these hashmap values into main words2 hash with max character count required to form all subsets
- Similarly, create a hashmap for every string in words1 while iterating and check for universal subsets. Universal subset => hash1[char] >= hash2[char].
- return the universal subsets

### Complexity Analysis
- **Constraints:** n = words1.size(), m = words2.size()
- **Constraints:** 0 <= words1[i].length(), words2[i].length() <= 10
- **Time Complexity:** O(n*(26+10) + m*(26+10))
- **Space Complexity:** O(n*10+26)

---
