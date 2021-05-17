
def check(a, b, h):
    def f6027():
        nonlocal a
        nonlocal b
        if a != 0:
            if b != 0:
                tmp = a # keep value of 'a' to not be affected by the recursion
                b -= 1
                f6027()
                b = a
                a = tmp
                a -= 1
                f6027()
                return
            else:
                a -= 1
                b = h
                f6027()
                return
        else:
            a = b + 1
            return

    if a != 0:
        f6027()
    return a, b

# Based on the ackermann table, the solution for 4 1 1 would be 65533
# how does h affect this result?

def smpl(a, b, h):
    if a == 0:
        return b + 1
    else:
        if b == 0:
            return smpl(a - 1, h, h)
        else:
            return smpl(a - 1, smpl(a, b - 1, h), h)

import sys

mod = 32768

if __name__ == '__main__':

    a = int(sys.argv[1])
    b = int(sys.argv[2])
    h = int(sys.argv[3])

    print("ctrl", check(a, b, h)[0])
    print("smpl", smpl(a, b, h))
