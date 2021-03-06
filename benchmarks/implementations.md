# Benchmarks of different implementations

## IntSpan 1

* Rust

```text
$ pushd ~/Scripts/rust/intspan && 
    cargo build --release --examples && 
    command time -l target/release/examples/benchmark && 
    popd
["target/release/examples/benchmark"]
step 2
duration: 0.024120771000000003 s
step 3
duration: 0.028843299 s
step 4
duration: 0.067436723 s
step 5
duration: 0.26832116899999997 s
step 6
duration: 0.470739197 s
        0.86 real         0.85 user         0.00 sys
   1073152  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
       271  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
        42  involuntary context switches

```

* Java

```text
$ pushd ~/Scripts/java/jintspan && 
    mvn clean verify && 
    command time -l java -jar target/jintspan-*-jar-with-dependencies.jar benchmark && 
    popd
step 2
duration 0.015188
step 3
duration 0.023388
step 4
duration 0.042441
step 5
duration 0.212403
step 6
duration 0.365264
        0.78 real         0.93 user         0.05 sys
 182984704  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
     46445  page reclaims
         1  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         1  signals received
         5  voluntary context switches
       654  involuntary context switches

```

* C

```text
$ pushd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark &&
    make &&
    command time -l ./test_c benchmark &&
    popd
step 2
duration 0.022875
step 3
duration 0.032172
step 4
duration 0.057164
step 5
duration 0.294729
step 6
duration 0.525069
        0.93 real         0.93 user         0.00 sys
   1085440  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
       274  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
       176  involuntary context switches

```

* Perl

```text
$ pushd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark &&
    command time -l perl test_ai.pl benchmark &&
    popd
step 2
duration 1.749132
step 3
duration 1.951063
step 4
duration 2.060910
step 5
duration 32.967968
step 6
duration 68.138299
      106.89 real       106.77 user         0.05 sys
   6500352  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
      1596  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
      6303  involuntary context switches

```

* Perl XS

```text
$ pushd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark &&
    command time -l perl test_ai.pl benchmark xs &&
    popd
step 2
step 2
duration 0.195549
step 3
duration 0.205843
step 4
duration 0.271525
step 5
duration 1.440861
step 6
duration 6.663916
        8.79 real         8.78 user         0.00 sys
   6586368  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
      1617  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
       609  involuntary context switches

```

## IntSpan 2

* Rust

```text
$ pushd ~/Scripts/rust/intspan && 
    cargo build --release --examples && 
    command time -l target/release/examples/file && 
    popd
["target/release/examples/file"]
step 1 create
duration: 0.014380602999999999 s
step 2 intersect
duration: 0.523088785 s
step 3 intersect runlist
duration: 0.611574268 s
        1.15 real         1.14 user         0.00 sys
   2699264  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
       668  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
       243  involuntary context switches

```

* Java

```text
$ pushd ~/Scripts/java/jintspan && 
    mvn clean verify && 
    command time -l java -jar target/jintspan-*-jar-with-dependencies.jar file && 
    popd
step 1 create
duration 0.053408
step 2 intersect
duration 0.347398
step 3 intersect runlist
duration 0.555934
        1.06 real         1.17 user         0.12 sys
 582258688  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
    143954  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         1  signals received
         0  voluntary context switches
      1194  involuntary context switches

```

* C

```text
$ pushd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark &&
    make &&
    command time -l ./test_c file &&
    popd
step 1 create
duration 0.118375
step 2 intersect
duration 2.174462
step 3 intersect runlist
duration 18.218233
       20.51 real        20.42 user         0.05 sys
   2121728  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
       521  page reclaims
         6  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         1  voluntary context switches
      6360  involuntary context switches

```

* Perl

```text
$ pushd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark &&
    command time -l perl test_ai.pl file &&
    popd
==> test against large sets
step 1 create
duration 4.548069
step 2 intersect
duration 61.313397
step 3 intersect runlist
duration 61.335031
      127.25 real       126.56 user         0.38 sys
  11943936  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
      2924  page reclaims
         1  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         6  voluntary context switches
     45869  involuntary context switches

```

* Perl XS

```text
$ pushd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark &&
    command time -l perl test_ai.pl file xs &&
    popd
==> test against large sets
step 1 create
duration 0.116019
step 2 intersect
duration 8.530752
step 3 intersect runlist
duration 8.677303
       17.37 real        17.26 user         0.05 sys
   9822208  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
      2407  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
      7011  involuntary context switches

```
