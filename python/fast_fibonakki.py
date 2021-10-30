def fib(n):
    a, b = 0, 1
    for _ in range(n):
        a, b = b, a + b
    return a


if __name__ == "__main__":
    n = int(input("Input to fibonacci: "))
    print("Fibonacci(", n, ") =", fib(n), sep="")
