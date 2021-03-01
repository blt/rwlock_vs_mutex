A simplistic demonstration of the differences between a RwLock and Mutex in
Rust.

Results when run on my system:

```
> cargo run --release
BASELINE || high: 151ns, low: 20ns, mid: 20ns
MUTEX SINGLE R/W || high: 1.179978ms, low: 27.452µs, mid: 41.288µs
MUTEX 100 R / SINGLE W || high: 4.217299ms, low: 1.851735ms, mid: 2.460652ms
^C
```

Final test ran for 5 minutes and I killed it.
