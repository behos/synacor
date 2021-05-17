
def check(h):
    a=4
    b=1

    def f6027():
        nonlocal a
        nonlocal b
        print(a, b)
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

    return b == 6

def f6027(a, b, h):
    # after a lot of trials I think the formula is b * (h + 1) + a * h
    return b * (h + 1) + a * h

import sys

mod = 32768

if __name__ == '__main__':
    for i in range(mod):
        if f6027(4, 1, i) % mod == 6:
            print(i)
