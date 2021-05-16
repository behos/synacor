
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

# post execution b should match 6, so what value should h start with

def refactored_check(h):

    def f6027(depth, a, b):
        print(a, b, f"d:{depth}")
        if a == 0:
            print(b + 1, b, f"ret:{depth}")
            return b + 1, b
        else:
            if b == 0:
                a, b = f6027(depth + 1, a - 1, h)
                print(a, b, f"ret:{depth}")
                return a, b
            else:
                b, _ = f6027(depth + 1, a, b - 1)
                a, b = f6027(depth + 1, a - 1, b)
                print(a, b, f"ret:{depth}")
                return a, b

    a, b = f6027(0, 4,1)
    return b == 6
