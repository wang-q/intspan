
* Rust

```text
$ cd ~/Scripts/rust/intspan
$ cargo build --release
$ command time -l target/release/intspan benchmark
["target/release/intspan", "benchmark"]
step 2
duration: 0.0291654 s
step 3
duration: 0.036023309999999996 s
step 4
duration: 0.08761089 s
step 5
duration: 0.36387602199999997 s
step 6
duration: 0.639986058 s
        1.16 real         1.15 user         0.00 sys
   1032192  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
       261  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
        65  involuntary context switches

```

* Java

```text
$ cd ~/Scripts/java/jintspan
$ mvn clean verify
$ command time -l java -jar target/jintspan-*-jar-with-dependencies.jar benchmark
step 2
duration 0.023358
step 3
duration 0.035295
step 4
duration 0.053588
step 5
duration 0.316216
step 6
duration 0.561500
        1.12 real         1.38 user         0.07 sys
 110718976  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
     30334  page reclaims
         4  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         1  signals received
         4  voluntary context switches
       926  involuntary context switches

```

* C

```text
$ cd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark
$ make
$ command time -l ./test_c benchmark
 test_add_range
step 2
duration 0.022878
step 3
duration 0.031702
step 4
duration 0.055751
step 5
duration 0.289292
step 6
duration 2.536171
        2.93 real         2.93 user         0.00 sys
   1081344  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
       273  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
       505  involuntary context switches

```

* Perl

```text
$ cd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark
$ command time -l perl test_ai.pl benchmark
step 2
duration 2.506869
step 3
duration 2.831008
step 4
duration 2.969270
step 5
duration 46.395918
step 6
duration 96.724945
      151.45 real       151.25 user         0.10 sys
   6377472  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
      1566  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
     14911  involuntary context switches

```
