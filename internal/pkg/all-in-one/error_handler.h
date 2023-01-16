extern void goErrorHandler(int, char*);

typedef void (*ErrorHandler)(int, char*);

void allInOneErrorHandler(int size, char* string) {
  goErrorHandler(size, string);
}