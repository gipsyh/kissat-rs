#include <stdarg.h>
#include <stdlib.h>
#include <stdio.h>
void kissat_abort(void) { abort(); }
void kissat_fatal_message_start(void) {}
void kissat_fatal(const char *fmt, ...) {
  va_list ap;
  va_start(ap, fmt);
  vfprintf(stderr, fmt, ap);
  va_end(ap);
  abort();
}
