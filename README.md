# Advent of Code 2023

Fifth year participating, using Rust this time.

Not in it to win it, but I'll try to comment my code to explain my approach.

Plain Rust, zero dependencies, no parallel processing.

```plain
Usage: cargo run --release <task> <input>
 <task>     Day number (two digits) plus part ('a' or 'b')
 <input>    Input file base name, e.g. 'input' or 'sample'
 --profile  Run solution multiple times and compute average duration
Example: cargo run --release 01a sample
```


# Results

The table below shows the average core runtime of each solution, recorded over an average of 20 runs. These times were recorded on a 2021 MacBook Pro using `rustc 1.74.0`. The core runtime does not include the time it takes to read the input file and split it into lines, but does include any additional input parsing.

| Day  | Part A (μs) | Part B (μs) |
| :--: | ----------: | ----------: |
|  01  |         24  |        325  |
|  02  |         85  |         85  |
|  03  |        288  |        157  |
|  04  |        384  |        404  |
|  05  |         23  |         37  |
|  06  |          1  |          1  |
|  07  |        339  |        329  |
|  08  |        111  |        769  |
|  09  |        212  |        208  |
|  10  |        180  |        252  |
|  11  |        753  |        760  |
|  12  |        773  |      7,818  |
|  13  |        136  |        124  |
|  14  |         32  |     37,825  |
|  15  |         86  |        537  |
|  16  |        437  |     94,776  |
|  17  |             |             |
|  18  |      7,175  |        314  |
|  19  |        403  |        656  |
|  20  |      2,948  |     12,108  |
|  21  |             |             |
|  22  |        624  |    133,114  |
|  23  |             |             |
|  24  |             |             |
|  25  |             |             |
