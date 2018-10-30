ffi:
	rm -f src/sigar/ffi.rs
	mkdir -p src/sigar
	bindgen --with-derive-default --distrust-clang-mangling include/wrapper.h -o src/sigar/ffi.rs -- -I libsigar/include
