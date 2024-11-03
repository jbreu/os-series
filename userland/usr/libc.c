#include "include/libc.h"

FILE _stdout = {
    .fd = 1,                // File descriptor 1 for stdout
    .buffer = 0,            // Some allocated buffer space
    .bufsize = 1024,        // Buffer size, line-buffered for terminal
    .pos = 0,               // Current position in buffer
    .flags = 0,             // Set to write-only
};

FILE *stdout = &_stdout;       // Point stdout to the _stdout instance


uint64_t strlen( const char* str ) {
    int len = 0;

    while (str[len] != '\0') {
        len++;
    }

    return len;
}

bool strcmp(const char* a, const char* b) {
    int i =0;

    while (a[i] != '\0') {
        if (a[i]==b[i]) {
            i++;
            continue;
        } else {
            return false;
        }
    }

    return true;
}

// Write function using syscall
int write(uint64_t filedescriptor, const char* payload, uint64_t len) {
    uint64_t result;
    DO_SYSCALL(1, result, filedescriptor, (uintptr_t)payload, len);
}

// Get process ID
uint64_t getpid() {
    uint64_t pid;
    DO_SYSCALL(2, pid, 0, 0, 0);  // No additional arguments needed
    return pid;
}

// Draw a pixel on screen
void draw_pixel(uint32_t x, uint32_t y, uint8_t color) {
    uint64_t result;
    DO_SYSCALL(3, result, x, y, color);
}

// Allocate memory
void* malloc(long unsigned int size) {
    uint64_t address;
    DO_SYSCALL(4, address, size, 0, 0);  // Only size is passed
    return (void*)address;
}

// Free memory (currently does nothing)
void free(void* address) {
    // TODO: Implement the free function
}

// Open a file
void* fopen(const char* filename, const char* options) {
    if (!strcmp(filename, "devdatadoom1.wad")) {
        return 0;
    }

    write(1, "fopen: ", strlen("fopen: "));
    write(1, filename, strlen(filename));

    uint64_t handle;
    DO_SYSCALL(5, handle, (uintptr_t)filename, (uintptr_t)options, 0);
    return (void*)handle;
}

// Close a file (currently does nothing)
void fclose(void* handle) {
    // TODO: Implement the fclose function
}

// Write to a file (currently does nothing)
long unsigned int fwrite(const void *, long unsigned int,  long unsigned int,  void *) {
    write(1, "fwrite: ", strlen("fwrite: "));
    return 0;  // TODO: Implement fwrite
}

// Seek within a file
int fseek ( FILE * stream, long int offset, int origin ) {
    uint64_t result;
    DO_SYSCALL(7, result, offset, origin, 0);
    return result;
}

// Check if end of file
int feof(void* handle) {
    uint64_t eof;
    DO_SYSCALL(9, eof, (uintptr_t)handle, 0, 0);
    return eof;
}

// Get the current position in a file
int ftell(void* handle) {
    uint64_t position;
    DO_SYSCALL(8, position, (uintptr_t)handle, 0, 0);
    return position;
}

// Read from a file
int fread(void* handle, void* ptr, int size) {
    uint64_t read_bytes;
    DO_SYSCALL(6, read_bytes, (uintptr_t)ptr, size, 0);
    return read_bytes;
}

// Draw the framebuffer
uint64_t draw_framebuffer(const uint8_t* framebuffer) {
    uint64_t result;
    DO_SYSCALL(10, result, (uintptr_t)framebuffer, 0, 0);
    return result;
}

// Switch VGA mode
uint64_t switch_vga_mode(bool vga_on) {
    uint64_t result;
    DO_SYSCALL(11, result, vga_on, 0, 0);
    return result;
}

// Get the state of a key
bool get_keystate(int key) {
    uint64_t state;
    DO_SYSCALL(12, state, key, 0, 0);
    return (bool)state;
}

void get_time(int* sec, int* usec) {
    uint64_t result;
    DO_SYSCALL(13, result, sec, usec, 0);
}

// A helper function to print an integer to the specified FILE stream.
void print_int(FILE *stream, int value) {
    if (value < 0) {
        fputc('-', stream);
        value = -value;
    }
    if (value / 10)
        print_int(stream, value / 10);
    fputc((value % 10) + '0', stream);
}

// A helper function to print a string to the specified FILE stream.
void print_str(FILE *stream, const char *str) {
    while (*str) {
        fputc(*str++, stream);
    }
}

// A simplified fprintf function
int fprintf(FILE *stream, const char *format, ...) {
    va_list args;
    va_start(args, format);
    int count = 0;

    while (*format) {
        if (*format == '%' && *(format + 1)) {
            format++;  // Skip '%'

            // Handle the format specifiers
            if (*format == 'd') {       // Integer
                int i = va_arg(args, int);
                print_int(stream, i);
            } else if (*format == 's') { // String
                const char *s = va_arg(args, const char *);
                print_str(stream, s);
            } else if (*format == 'c') { // Character
                char c = (char)va_arg(args, int);
                fputc(c, stream);
            } else { // Unknown format, print as-is
                fputc('%', stream);
                fputc(*format, stream);
            }
        } else { // Regular character, print as-is
            fputc(*format, stream);
        }
        format++;
        count++;
    }

    va_end(args);
    return count;
}

// TODO very inefficient, one syscall for each character - use buffered approach
int fputc(int c, FILE *stream) {
    unsigned char ch = (unsigned char)c;

    if (write(stream->fd, &ch, 1) != 1) {
        return EOF;
    }

    return (int)ch;
}
