from typing import List
from itertools import permutations


class ALU:
    def __init__(self, _input: List[int]) -> None:
        self.w = 0
        self.x = 0
        self.y = 0
        self.z = 0
        self.input = _input
        self.pos = 0

    def run(self, file_name):
        with open(file_name) as f:
            for line in f.readlines():
                instruction, *res = line.strip().split(" ")
                if len(res) == 1:
                    n = self.input[self.pos]
                    self.pos += 1
                    if res[0] == "w":
                        self.w = n
                    elif res[0] == "x":
                        self.x = n
                    elif res[0] == "y":
                        self.y = n
                    elif res[0] == "z":
                        self.z = n
                else:
                    if res[0] == "w":
                        a = self.w
                    elif res[0] == "x":
                        a = self.x
                    elif res[0] == "y":
                        a = self.y
                    elif res[0] == "z":
                        a = self.z

                    try:
                        b = int(res[1])
                    except:
                        if res[1] == "w":
                            b = self.w
                        elif res[1] == "x":
                            b = self.x
                        elif res[1] == "y":
                            b = self.y
                        elif res[1] == "z":
                            b = self.z

                    if instruction == "add":
                        a = a + b
                    elif instruction == "mul":
                        a = a * b
                    elif instruction == "div":
                        if b == 0:
                            print("AJJAWKEJFEKJF")
                        a = a // b
                    elif instruction == "mod":
                        if a < 0 or b <= 0:
                            print("EJJEEJEJJEEJJEJE")
                        a = a % b
                    elif instruction == "eql":
                        a = 1 if a == b else 0

                    if res[0] == "w":
                        self.w = a
                    elif res[0] == "x":
                        self.x = a
                    elif res[0] == "y":
                        self.y = a
                    elif res[0] == "z":
                        self.z = a

        return self.z

    def __str__(self) -> str:
        return f"{self.z=}\n{self.y=}\n{self.x=}\n{self.w=}\n{self.pos=}\n{self.input=}"


alu = ALU([int(k) for k in str(54986732675439)])
alu.run("./input.txt")
print(alu)