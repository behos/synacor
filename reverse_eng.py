
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

def s(a, b, h):
    if a == 0:
        return b + 1, b
    else:
        if b == 0:
            a, b = s(a - 1, h, h)
            return a, b
        else:
            b, _ = s(a, b - 1, h)
            a, b = s(a - 1, b, h)
    return a, b

def f6027(a, b, h):
    # after a lot of trials I think the formula is b * (h + 1) + a * h
    b = a ** (b * h)
    return b + 1, b

import sys

mod = 32768

if __name__ == '__main__':

    a = int(sys.argv[1])
    b = int(sys.argv[2])
    h = int(sys.argv[3])

    print("ctrl", check(a, b, h))
    print("smpl", s(a, b, h))
    print("frml", f6027(a, b, h))

    # for i in range(mod):
    #     if f6027(4, 1, i) % mod == 5:
    #         print(i)
