int i;
unsigned int i;
unsigned long long i;
struct foo i;
const int i;
const struct foo i;
const int* i;
const struct foo* i;
int * const i;
int i[500];
int * i[500];
int (*f_ptr)(int);
// TODO: [100] shoudl be after f_name
const int * const (*f_name)(const int * i[100])[100];
struct {} *i[500];
