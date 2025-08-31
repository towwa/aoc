import fileinput
from collections import deque, defaultdict
from functools import cache
from heapq import heappop, heappush

lines = [l.strip() for l in fileinput.input()]

registers = [0]*3
registers[0] = int(lines[0].split(':')[1])
registers[1] = int(lines[1].split(':')[1])
registers[2] = int(lines[2].split(':')[1])
# day 17 part 1
def run(registers) -> list[int]:
    registers = registers[:]
    def combo(v):
        if v <=3:
            return v
        return registers[v-4]
    program = [int(i) for i in lines[4].split(':')[1].split(',')]
    out = []
    ptr = 0
    while True:
        if ptr + 1 >= len(program):
            break
        opcode, operand = program[ptr], program[ptr + 1]
        increment = True
        match opcode:
            case 0:
                num = registers[0]
                dem = 2** combo(operand)
                registers[0] = int(num / dem) 
            case 1:
                registers[1] = registers[1] ^ operand
            case 2:
                registers[1] = combo(operand)%8
            case 3:
                if registers[0] != 0:
                    increment = False
                    ptr = operand
            case 4:
                registers[1] = registers[1] ^ registers[2]
            case 5:
                out.append(combo(operand)%8)
            case 6:
                num = registers[0]
                dem = 2** combo(operand)
                registers[1] = int(num / dem) 
            case 7:
                num = registers[0]
                dem = 2** combo(operand)
                registers[2] = int(num / dem) 
            case _:
                assert False, f"Unknown opcode {opcode}"
        if increment:
            ptr +=2
    return out
print(registers)
print('silver', ','.join([str(i) for i in run(registers)]), registers)

def func(value):
    registers = [0]*3
    registers[0] = value
    return run(registers)
def arrtoint(arr):
    return int(''.join([str(i) for i in arr]))
def functoint(value):
    return arrtoint(func(value))
program = [int(i) for i in lines[4].split(':')[1].split(',')]
ptarg = program[::-1]
print(program)
for i in [8, 64, 512, 4096, 32768, 262144]:
    print(i, func(i))
# reversing my program:


# set b to a%8
# set b = b^1
# c = int(a / 2 ** b)
# b = b^5
# b = b ^ c
# output b % 8
# a = int(a / 8)
# restart program if a != 0


