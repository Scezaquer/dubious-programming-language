stack = []
RAX = 0
R9 = 0
R10 = 0
R11 = 0
RSP = -1


def procedural_array(array_content, expr):
    global stack, RAX, R9, R10, R11, RSP

    R9 = expr
    R10 = expr

    while R10 > 0:
        if isinstance(array_content, int):
            stack.append(array_content)
            RSP += 1
            R11 += 1
        else:
            stack.append(R9)
            RSP += 1
            R11 += 1
            stack.append(R10)
            RSP += 1
            R11 += 2
            stack.append(R11)
            RSP += 1
            R11 = 0
            procedural_array(array_content[0], array_content[1])
            stack.append(RAX)
            RSP += 1
            R11 += 1

            R9 = stack[-R11-3]
            R10 = stack[-R11-2]
            R11 += stack[-R11-1]

        R10 -= 1

    if not isinstance(array_content, int):
        R8 = 0
        while R10 < R9:
            stack.append(RSP - R8 - 3 - R10)
            RSP += 1
            R11 += 1
            R10 += 1
            R8 += stack[RSP - R8 - 1 - R10]
            R8 += 5

    stack.append(R9)
    RSP += 1
    R11 += 1
    RAX = RSP - 1  # Simulate the stack pointer
    stack.append(R11)
    RSP += 1
    R11 += 1


def printstack(stack):
    print("Stack contents:")
    for i in range(len(stack)):
        print(f"  {i}: {stack[i]}")
    print(f"RSP: {RSP}, RAX: {RAX}, R8: {R9}, R9: {R10}, R10: {R11}")


procedural_array(((0, 5), 5), 5)
printstack(stack)
