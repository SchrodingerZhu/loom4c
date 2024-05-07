CXX := c++

target/release/libloom4c.so: src/lib.rs Cargo.toml
	cargo build --release

test.exe: target/release/libloom4c.so test/test.cpp
	$(CXX) -o test.exe test/test.cpp -Ltarget/release -lloom4c -Wl,-rpath=target/release -O3 -g3

.PHONY: clean
clean:
	cargo clean
	rm -rf test.exe