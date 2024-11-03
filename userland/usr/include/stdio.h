#include "ctype.h"

typedef struct {
    int fd;                 // File descriptor for system calls
    unsigned char *buffer;  // Pointer to buffer
    size_t bufsize;         // Buffer size
    size_t pos;             // Current position in buffer
    int flags;              // Flags to indicate mode, errors, EOF, etc.
} FILE;

FILE *stdout;
FILE *stderr;

#define EOF (-1)
#define NULL ((void *)0)


int fprintf(FILE *stream, const char *format, ...);
int sprintf(char *str, const char *format, ...);
int fputs ( const char * str, FILE * stream );
int putc( int c, FILE * stream );
char * strrchr (char * str, int character );
int getopt(int argc, char * const argv[], const char *optstring);
void* fopen(const char* filename, const char* options);
void fclose(void* handle);
char *strerror(int errnum);