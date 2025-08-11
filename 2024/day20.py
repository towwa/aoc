import fileinput
from collections import deque, defaultdict, Counter
from functools import cache
from heapq import heappop, heappush

lines = [l.strip() for l in fileinput.input()]

walls = set()
start = -1, -1
end = -1, -1
TARGET = 100
distance = -1

for i in range(len(lines)):
    for j in range(len(lines[0])):
        if lines[i][j] == '#':
            walls.add((i,j))
        if lines[i][j] == 'S':
            start = i,j
        if lines[i][j] == 'E':
            end = i,j
assert start[0] != -1 and end[0] != -1
def bfs(beg=[0, start, 1], search=False): #search means we're finding initial distance
    q = deque() # dist, (x,y), cheats
    q.append(beg)
    visited = set() # state
    visited.add(beg)
    results = Counter()
    while q:
        dist, pos, cheats = q.popleft()
        if pos == end:
            results[dist] +=1
            if search:
                break
            elif dist == distance:
                break
        visited.add((dist, pos, cheats))
        for xdiff, ydiff in [[0, 1], [0, -1], [1,0], [-1, 0]]:
            nx, ny = pos[0] + xdiff, pos[1] + ydiff
            if nx>=0 and nx < len(lines) and ny >=0 and ny < len(lines[0]):
                if (nx,ny) not in walls:
                    np = (dist+1, (nx,ny), cheats)
                    if np in visited:
                        continue
                    visited.add(np)
                    q.append(np)
                elif cheats: # try to jump , intermediate in wall state 
                    np = (dist+1, (nx,ny), 0)
                    if np in visited:
                        continue
                    visited.add(np)
                    q.append(np)
    return results
    

fdist = bfs((0,start,0), True)
distance = fdist.popitem()[0]
print(fdist)
print('regular bfs distance is ', distance)

allfound = bfs((0,start,1), False)
print(allfound)