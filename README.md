
* Rust

```text
$ cd ~/Scripts/rust/intspan
$ cargo build --release
$ command time -l target/release/intspan benchmark
["target/release/intspan", "benchmark"]
step 2
duration: 0.030005217 s
step 3
duration: 0.035659909 s
step 4
duration: 0.09449890700000001 s
step 5
duration: 0.371066978 s
step 6
duration: 1.430527595 s
        1.96 real         1.96 user         0.00 sys
   1036288  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
       262  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
       127  involuntary context switches

```

* Java

```text
$ cd ~/Scripts/java/jintspan
$ mvn clean verify
$ command time -l java -jar target/jintspan-*-jar-with-dependencies.jar benchmark
step 2
duration 0.021720
step 3
duration 0.034677
step 4
duration 0.052226
step 5
duration 0.286712
step 6
duration 0.725323
        1.27 real         1.56 user         0.10 sys
 176377856  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
     47402  page reclaims
         4  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         1  signals received
        21  voluntary context switches
      1402  involuntary context switches

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
