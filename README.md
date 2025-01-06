# Advent of Code 2024

I was on parental leave during AoC 2024, so I used the time away from my day job writing code to write code in my free time. 

At first, I was going to use a familiar language (golang) and [did the first few days](https://github.com/joshprzybyszewski/aoc2022/tree/2024). However, I saw CodSpeed was a sponsor for AoC, and they were [hosting a competition](https://codspeed.io/advent) for "who can write the solver that solves the puzzle the fastest". I was intrigued, but it meant I had to learn Rust. I'd never read or written Rust before, but I thought that sounded fun.

Once the day's puzzle was released, competitors had 36 hours (noon EST, the next day) to publish a working solution. It must solve the "secret input" that they had with the correct answer, and then the benchmarks would run. (The main product offering of CodSpeed seems to be a consistent benchmarking environment in Github Actions for Rust and Python, so results are comparable across time.)

### Setup

To set up this repo, I followed the instructions [here](https://github.com/gobanos/cargo-aoc). 

[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/joshprzybyszewski/aocrust/benchmarks)

I set up CodSpeed to run benchmarks using [these instructions](https://gist.github.com/art049/a824a8607898241a3fe061488817099e) (and then [added a 5m timeout](https://github.com/joshprzybyszewski/aocrust/commit/a55c25aa02b6a78d0229a60c1940d429b3915673) after running benchmarks for [6 hours](https://github.com/joshprzybyszewski/aocrust/actions/runs/12508202699/job/34895943036) :oops:).

## How'd it go?

I finished 21st out of [144 total competitors](https://codspeed.io/advent/leaderboard/global). I'm very happy with that, considering 1) I've never written Rust before and 2) the top 3 to 10 submissions every day were basically assembly wrapped in Rust (like [simd](https://github.com/indiv0/aoc-fastest) and stuff I didn't want to learn this year).

I'm certainly no expert in Rust, but I've gotten to dabble in some of what it has to offer. Particularly, I love that the author must declare `mut` for mutable pieces of memory.

My highest scoring day was [Day 20](https://codspeed.io/advent/day/20): 5th out of 32 submissions. I liked Day 20 because I was introduced to [the idea](https://github.com/joshprzybyszewski/aocrust/blob/c47f48c7ae533b0d9cfaddc541a78fa2333b7813/src/day20.rs#L163-L164) of only checking half of the diamond of N-nearest spaces by a fellow competitor.

Perhaps my favorite algorithm from this year was on Day 23. I used [Bron-Kerbosch](https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm) to [find the largest clique in an undirected graph](https://github.com/joshprzybyszewski/aocrust/blob/c47f48c7ae533b0d9cfaddc541a78fa2333b7813/src/day23.rs#L219-L272). I was able to find and adapt an implementation from a 2018 AoC solver, which taught me better ways to share (vs. clone) data structures in Rustlang.

I had submissions for 20 of the 25 days. 

On the days I missed, I learned something each time:
-  Day 7 because I didn't understand the compiler error about `Result<usize>` vs. `usize` ([oops](https://github.com/joshprzybyszewski/aocrust/commit/a1aaf3d6b729f674f8500f1fc7c4259da0e4b324)). 
- Day 10 because the grid could be up to 64x64, but [I assumed it was always 57x57](https://github.com/joshprzybyszewski/aocrust/commit/f1c5d2204bc38f00afe2afcc171c80e91279beab).
- Day 14 because I assumed, at first, that the correct output is the first where no two robots are on the same position -- which is true for my input, but [not the competition one](https://github.com/joshprzybyszewski/aocrust/commit/9be02107d5e604f21737e8faed706162e0fdf5cf). I'm guessing a second check (i.e. for a border around the tree) is necessary to get the "correct" answer. On third pass, I learned about the Chinese remainder theorem.
- Day 21 because I knew it was basically pre-computable, so I was greedy and only wanted to submit a solver that did an array lookup. I wasn't able to grok the recursion using `const fn`s in time to submit (wasn't until the [26th](https://github.com/joshprzybyszewski/aocrust/commit/58a2aafbfcf6c89d4a09fbb1873fc852ece08804)).
- Day 24 because I was celebrating Christmas with family... Also recursing through an N-bit ripple adder to identify "output gates" that were _wrong_ was really throwing me off. I had to re-think the solver for this 3 or 4 times to be feel sort of confident in "[the right way to detect a bad gate](https://github.com/joshprzybyszewski/aocrust/commit/af9fe38a163d671d83469bdaf244c3e1175349bd)".

## Results

Here's the results on my machine (`Intel(R) Core(TM) i5-3570 CPU @ 3.40GHz`):

| Benchmark | Duration |
| - | -: |
| day1_part1 | 33.941 µs |
| day1_part2 | 35.384 µs |
| day2_part1 | 40.731 µs |
| day2_part2 | 47.323 µs |
| day3_part1 | 19.317 µs |
| day3_part2 | 28.392 µs |
| day4_part1 | 218.32 µs |
| day4_part2 | 162.13 µs |
| day5_part1 | 521.99 µs |
| day5_part2 | 562.33 µs |
| day6_part1 | 36.642 µs |
| day6_part2 | 73.531 ms |
| day7_part1 | 913.17 µs |
| day7_part2 | 5.6330 ms |
| day8_part1 | 5.6517 µs |
| day8_part2 | 6.9268 µs |
| day9_part1 | 42.611 µs |
| day9_part2 | 2.1772 ms |
| day10_part1 | 128.31 µs |
| day10_part2 | 53.904 µs |
| day11_part1 | 84.086 µs |
| day11_part2 | 1.7363 ms |
| day12_part1 | 398.77 µs |
| day12_part2 | 539.74 µs |
| day13_part1 | 9.3828 µs |
| day13_part2 | 11.760 µs |
| day14_part1 | 6.5758 µs |
| day14_part2 | 379.72 µs |
| day15_part1 | 325.98 µs |
| day15_part2 | 329.60 µs |
| day16_part1 | 3.1873 ms |
| day16_part2 | 3.5521 ms |
| day17_part1 | 226.34 ns |
| day17_part2 | 97.860 µs |
| day18_part1 | 53.137 µs |
| day18_part2 | 102.14 µs |
| day19_part1 | 494.81 µs |
| day19_part2 | 487.61 µs |
| day20_part1 | 150.57 µs |
| day20_part2 | 10.683 ms |
| day21_part1 | 41.736 ns |
| day21_part2 | 41.426 ns |
| day22_part1 | 8.9992 ms |
| day22_part2 | 27.416 ms |
| day23_part1 | 1.3607 ms |
| day23_part2 | 2.0715 ms |
| day24_part1 | 162.73 µs |
| day24_part2 | 234.03 µs |
| day25_part1 | 16.400 µs |
| day25_part2 | 272.06 ps |

Here's the results from [CodSpeed](https://codspeed.io/joshprzybyszewski/aocrust/benchmarks) (as of f0d57b52ca0d48805e7ebc859df0f6380015909d):

| Benchmark | Duration |
| - | -: |
|day1_part1|142.1 µs|
|day1_part2|147.8 µs|
|day2_part1|118.3 µs|
|day2_part2|148.4 µs|
|day3_part1|61.8 µs|
|day3_part2|92.1 µs|
|day4_part1|544.7 µs|
|day4_part2|187.5 µs|
|day5_part1|15.5 ms|
|day5_part2|16.4 ms|
|day6_part1|142.2 µs|
|day6_part2|247.9 ms|
|day7_part1|4.9 ms|
|day7_part2|79.6 ms|
|day8_part1|32.2 µs|
|day8_part2|37.4 µs|
|day9_part1|117.2 µs|
|day9_part2|8.2 ms|
|day10_part1|680.3 µs|
|day10_part2|229.7 µs|
|day11_part1|1.4 ms|
|day11_part2|5.2 ms|
|day12_part1|1.4 ms|
|day12_part2|1.6 ms|
|day13_part1|38 µs|
|day13_part2|37.7 µs|
|day14_part1|27.3 µs|
|day14_part2|749.1 µs|
|day15_part1|630.2 µs|
|day15_part2|668.4 µs|
|day16_part1|14.6 ms|
|day16_part2|15.2 ms|
|day17_part1|3.2 µs|
|day17_part2|271 µs|
|day18_part1|249.2 µs|
|day18_part2|426.5 µs|
|day19_part1|830.1 µs|
|day19_part2|829.9 µs|
|day20_part1|724.9 µs|
|day20_part2|19.7 ms|
|day21_part1|985 ns|
|day21_part2|926.7 ns|
|day22_part1|14.5 ms|
|day22_part2|102.2 ms|
|day23_part1|4.5 ms|
|day23_part2|6.8 ms|
|day24_part1|357.6 µs|
|day24_part2|547.5 µs|
|day25_part1|89.5 µs|
|day25_part2|0 s|
