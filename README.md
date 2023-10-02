# Proof of Work Solver

This program solves such challenge:
```
sha256(MIa3OVQfKrrNBmpC + ???) == 00000000000000000000000000(26)
```

# Installtion

- Cargo crate publish: soon
- Prebuilt: check release

## Usage

Simply run:
```bash
# Example is given for the multi-threaded version
./solver-multi <prefix> <answer>

# Example
./solver-multi MIa3OVQfKrrNBmpC 26
```

You will get output like:

```
Answer is 90837933
Time elapsed in solve() is: 6.1497339s
```

## Performance
For our example problem given above:
```
CPU Intel(R) Core(TM) i5-10500H CPU @ 2.50GHz

Benchmark 1: .\solver-multi.exe MIa3OVQfKrrNBmpC 26
  Time (mean ± σ):      7.094 s ±  0.090 s    [User: 77.039 s, System: 0.110 s]
  Range (min … max):    7.033 s …  7.341 s    10 runs

Benchmark 2: .\solver-single.exe MIa3OVQfKrrNBmpC 26
  Time (mean ± σ):     29.534 s ±  0.131 s    [User: 25.044 s, System: 0.015 s]
  Range (min … max):   29.290 s … 29.744 s    10 runs
```
So just use the multi-threaded one, and allow your computer to be laggy for a short period of time.

## Avaliable targets
You can find prebuilt binaries for the following targets at [releases](https://github.com/66Leo66/PoW-solver-rs/releases):
- aarch64-linux-android
- aarch64-unknown-linux-gnu
- aarch64-unknown-linux-musl
- armv7-unknown-linux-musleabihf
- i686-pc-windows-gnu
- i686-unknown-linux-gnu
- x86_64-pc-windows-gnu
- x86_64-unknown-freebsd
- x86_64-unknown-linux-gnu
- x86_64-unknown-linux-musl

## Credits
I was going to use [RobinJadoul/proof-of-work](https://github.com/RobinJadoul/proof-of-work/) but it has no rust version, my PC has no golang environment, and it's single-threaded.

So I worked with ChatGPT and made a Rust version.
