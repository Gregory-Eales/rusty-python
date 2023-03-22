import time
import rusty_python as rp

def sieve_of_eratosthenes(n):
    primes = [True] * (n+1)
    primes[0] = False
    primes[1] = False

    for i in range(2, int(n**0.5)+1):
        if primes[i]:
            for j in range(i*i, n+1, i):
                primes[j] = False

    result = [i for i in range(2, n+1) if primes[i]]
    return result

n = 200000000

start_time = time.time()
primes = sieve_of_eratosthenes(n)
end_time = time.time()

#print("The first 10 prime numbers are:", primes)
print("Python execution time:", end_time - start_time, "seconds")


start_time = time.time()
primes = rp.get_n_primes(n)
end_time = time.time()

#print("The first 10 prime numbers are:", primes)
print("Rusty execution time:", end_time - start_time, "seconds")