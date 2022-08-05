import math
 
def sieve_of_Sundaram(n):
    """The sieve of Sundaram is a simple deterministic algorithm for finding all the prime numbers up to a specified integer."""
    if n < 3:
        if n < 2:
            return 0
        else:
            return 1    
    k = (n - 3) // 2 + 1

    integers_list = [True for i in range(k)]

    ops = 0

    for i in range((int(math.sqrt(n)) - 3) // 2 + 1):
#        if integers_list[i]: # adding this condition turns it into a SoE!
            p = 2 * i + 3
            s = (p * p - 3) // 2 # compute cull start

            for j in range(s, k, p):
                integers_list[j] = False
                ops += 1

    print("Total operations:  ", ops, ";", sep='')

    count = 1
    for i in range(k):
        if integers_list[i]:
            print(i*2+1)
            count += 1

    print("Found ", count, " primes to ", n, ".", sep='')

sieve_of_Sundaram(10)
