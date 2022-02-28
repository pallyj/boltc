llc --filetype=asm hello-world.ll -o hello-world.s
llc --filetype=obj hello-world.ll -o hello-world.o
clang test.c hello-world.o -o test.o
./test.o