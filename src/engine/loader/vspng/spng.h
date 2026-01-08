#ifndef SPNG_H
#define SPNG_H
#include <stdint.h>

void read_png_file(char *filename);
int getx();
int gety();
int8_t get_pixel(int x, int y, int c);
void clear();

#endif