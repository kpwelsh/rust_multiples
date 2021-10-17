from sympy.polys.polytools import factor
from target.release.libmultiples import multiples
from sympy import primefactors as factors

def prod(A):
  r = 1
  for i in A:
    r *= i
  return r

rad_a = 1267
mults = multiples(factors(rad_a), 2 ** 60)


ns = [
1605289,
290557309,
52590872929,
9518948000149,
1722929588026969,
311850255432881389,
1104664012222876561,
]

for n in ns:
    print(2 ** 63 / n)


print(factors(rad_a))

for A in mults:
    print(A, factors(A), [A % a for a in factors(rad_a)])
    assert prod(factors(A)) == rad_a, f'A: {A}, f_A: {factors(A)}'