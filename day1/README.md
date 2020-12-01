# Day 1

Implemented in Haskell, Rust and C for speed comparison.

```
[neil@bawbags haskell]$ ghc -dynamic -O day1.hs
[1 of 1] Compiling Main             ( day1.hs, day1.o ) [Optimisation flags changed]
Linking day1 ...
[neil@bawbags haskell]$ time ./day1 
840324
170098110

real	0m0.033s
user	0m0.023s
sys	0m0.010s
```

```
[neil@bawbags rust-day1]$ cargo build --release
   Compiling rust-day1 v0.1.0 (/home/neil/Projects/adventofcode2020/day1/rust-day1)
    Finished release [optimized] target(s) in 0.53s
[neil@bawbags rust-day1]$ time ./target/release/rust-day1 
Part1 840324
Part2 170098110

real	0m0.006s
user	0m0.006s
sys	0m0.000s
```

```
[neil@bawbags c]$ gcc -O3 -o day1 day1.c
[neil@bawbags c]$ time ./day1 
part1: 840324
part2: 170098110

real	0m0.007s
user	0m0.003s
sys	0m0.003s
```


