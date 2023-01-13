extern void goErrorHandler(int, char*);

typedef void (*ErrorHandler)(int, char*);

__attribute__((weak))
void allInOneErrorHandler(int size, char* string) {
  goErrorHandler(size, string);
}