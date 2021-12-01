from typing import List

def part1() -> int:
    increasing_count = 0
    prev = None
    for line in open("./input.txt").readlines():
        num = int(line)

        if prev:
            if num > prev:
                increasing_count += 1

        prev = num


    print(increasing_count)

def part2() -> int:
    increasing_count = 0
    prev_sum = None
    buffer: List[int] = []

    for i, line in enumerate(open("./input.txt").readlines()):
        num = int(line)
        buffer.append(num)
        if len(buffer) > 3:
            buffer = buffer[1:]
        
        if i >= 2:
            buffer_sum = sum(buffer)

            if prev_sum and buffer_sum > prev_sum:
                increasing_count += 1

            prev_sum = buffer_sum

    return increasing_count

part1()
part2()