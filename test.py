# Feel free to put this wherever is convenient. It gets built 
# into the /target/release/ folder by default
from target.release.libmultiples import multiples
import time

# Once you have compiled the library with 'cargo build --release', you might need
# to rename the output library, depending on your OS.
# On windows, you can rename the libmultiples.dll -> libmultiples.pyd
# You might be able to directly import the .so, though on linux

primes = [2.,3.,7.,13.]
s = time.perf_counter()
vals = multiples(primes, 2**63)
e = time.perf_counter()
print('Number found: ', len(vals))
print('Time: ', e - s)

