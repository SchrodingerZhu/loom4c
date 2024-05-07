# Loom4c

I ported the loom model checker to C/C++ because I wanted to examine some futex based lock implementations. [CDSChecker](http://plrg.eecs.uci.edu/software_page/42-2/) is another option but it is rather old and less maintained. 

Currently, not all "futex" operations are supported. `loom4c` only provides a limited set of `wait/wake` operations via `wait(expected)` and `notify_one()/notify_all()`.

There are `Mutex/Condvar/...` primitives inside `loom`. They are not exposed via the C-FFI yet because this project is for now designed for experimenting lock implementations; but porting existing locks can be
easily done.

Some important parts are missing due to the limitation of `loom`:
- shared memory support
- timed waiting
- weak version of compare_and_exchange
- real `SeqCst` (which will always be treated as `AcqRel` inside loom)

## Compile and Run
```bash
make test.exe && ./test.exe
```