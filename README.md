# Find Prime

Generates prime numbers using Miller-Rabin probabilistic primality test.
Written in Rust.

### Usage

`rusty-prime 17 5` Would generate:
```rust
Starting with 3 and finding 20 more primes.
5
7
11
13
17
19
23
29
31
37
41
43
47
53
59
61
67
71
73
79
Found 20 primes in 0 seconds.
```

### Resources
* Heavily borrowed from https://stackoverflow.com/questions/35423277/is-the-big-integer-implementation-in-the-num-crate-slow.