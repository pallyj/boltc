cargo run --release -- test/test.bolt
clang test/test.c -c -o test/test.o
clang test.o test/test.o -e _2L4F4testmain
./a.out