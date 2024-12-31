# Advent of Code 2024

I was on parental leave during AoC 2024, so I used the time away from my day job writing code to write code in my free time. 

At first, I was going to use a familiar language (golang) and [did the first few days](https://github.com/joshprzybyszewski/aoc2022/tree/2024). However, I saw CodSpeed was a sponsor for AoC, and they were [hosting a competition](https://codspeed.io/advent) for "who can write the solver that solves the puzzle the fastest". I was intrigued, but it meant I had to learn Rust. I'd never read or written Rust before, but I thought that sounded fun.

### Setup

To set up this repo, I followed the instructions [here](https://github.com/gobanos/cargo-aoc). 

[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/joshprzybyszewski/aocrust/benchmarks)

I set up CodSpeed to run benchmarks using [these instructions](https://gist.github.com/art049/a824a8607898241a3fe061488817099e) (and then [added a 5m timeout](https://github.com/joshprzybyszewski/aocrust/commit/a55c25aa02b6a78d0229a60c1940d429b3915673) after running benchmarks for [6 hours](https://github.com/joshprzybyszewski/aocrust/actions/runs/12508202699/job/34895943036) :oops:).

## How'd it go?

I finished 21st out of [144 total competitors](https://codspeed.io/advent/leaderboard/global). I'm very happy with that, considering 1) I've never written Rust before and 2) the top 3 to 10 submissions every day were basically assembly wrapped in Rust (like [simd](https://github.com/indiv0/aoc-fastest) and stuff I didn't want to learn this year).

Once the day's puzzle was released, competitors had 36 hours (noon EST, the next day) to publish a working solution. It must solve the "secret input" that they had with the correct answer, and then the benchmarks would run. (The main product offering of CodSpeed seems to be a consistent benchmarking environment in Github Actions for Rust and Python, so results are comparable across time.)

My highest scoring day was [Day 20](https://codspeed.io/advent/day/20): 5th out of 32 submissions. I liked Day 20 because I was introduced to [the idea](https://github.com/joshprzybyszewski/aocrust/blob/c47f48c7ae533b0d9cfaddc541a78fa2333b7813/src/day20.rs#L163-L164) of only checking half of the diamond of N-nearest spaces by a fellow competitor.

Perhaps my favorite algorithm from this year was on Day 23. I used [Bron-Kerbosch](https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm) to [find the largest clique in an undirected graph](https://github.com/joshprzybyszewski/aocrust/blob/c47f48c7ae533b0d9cfaddc541a78fa2333b7813/src/day23.rs#L219-L272). I was able to find and adapt an implementation from a 2018 AoC solver, which taught me better ways to share (vs. clone) data structures in Rustlang.

I had submissions for 20 of the 25 days. 

On the days I missed, I learned something each time:
-  Day 7 because I didn't understand the compiler error about `Result<usize>` vs. `usize` ([oops](https://github.com/joshprzybyszewski/aocrust/commit/a1aaf3d6b729f674f8500f1fc7c4259da0e4b324)). 
- Day 10 because the grid could be up to 64x64, but [I assumed it was always 57x57](https://github.com/joshprzybyszewski/aocrust/commit/f1c5d2204bc38f00afe2afcc171c80e91279beab).
- Day 14 because I assume that the correct output is the first where no two robots are on the same position -- which is true for my input, but [not the competition one](https://github.com/joshprzybyszewski/aocrust/commit/9be02107d5e604f21737e8faed706162e0fdf5cf) (I still don't know the "correct" way to solve this one:#).
- Day 21 because I knew it was basically pre-computable, so I was greedy and only wanted to submit a solver that did an array lookup. I wasn't able to grok the recursion using `const fn`s in time to submit (wasn't until the [26th](https://github.com/joshprzybyszewski/aocrust/commit/58a2aafbfcf6c89d4a09fbb1873fc852ece08804)).
- Day 24 because I was celebrating Christmas with family... Also recursing through an N-bit ripple adder to identify "output gates" that were _wrong_ was really throwing me off. I had to re-think the solver for this 3 or 4 times to be feel sort of confident in "[the right way to detect a bad gate](https://github.com/joshprzybyszewski/aocrust/commit/af9fe38a163d671d83469bdaf244c3e1175349bd)".
