import fileinput
from collections import deque, defaultdict
from functools import cache
from heapq import heappop, heappush

lines = [l.strip() for l in fileinput.input()]

flags = lines[0].split(', ')
silver, gold = 0,0
@cache
def possible(pattern, i=0):
    if i>=len(pattern):
        return 1
    res = 0
    for f in flags:
        end = len(f)
        if pattern[i:i+end] == f:
            res += possible(pattern, i+end)
    return res

for pattern in lines[2:]:
    rs = possible(pattern,0)
    if rs:
        silver +=1
    gold += rs

print('silver', silver)
print('gold', gold)