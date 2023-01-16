#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Represents a all in one instance
 */
typedef struct AllInOneInstance AllInOneInstance;

void set_error_handler(void (*func)(int, char*));

/**
 * Loads the config json using the nova shared config loader
 */
char *load_config(void);

void stop_instance(struct AllInOneInstance *instance);

/**
 * # Panics
 * Panics if an incorrect `RUST_LOG` variables is specified.
 */
struct AllInOneInstance *create_instance(char *config);
