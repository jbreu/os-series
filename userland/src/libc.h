#include <inttypes.h> 

uint64_t strlen( const char* str );
uint64_t getpid();
void draw_pixel(uint32_t x, uint32_t y, uint8_t color);
uint64_t malloc(uint64_t size);
uint64_t free(uint64_t address);
void fopen();
void fclose();
void fwrite();
void fseek(uint64_t offset, uint64_t origin);
uint64_t feof();
uint64_t ftell();
void fread(uint8_t* ptr, uint64_t size, uint64_t nmemb);

void write(int filedescriptor, const char* payload, int len);
