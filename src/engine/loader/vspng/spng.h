#ifndef SPNG_H
#define SPNG_H
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <png.h>

static void user_read_data(png_structp png_ptr, png_bytep out_buffer, png_size_t bytes_to_read);
void parsepng(png_structp png, png_infop info);
void read_png_file(const char *path);
void parse_png_buffer(const unsigned char* data, uint32_t size);
int getx();
int gety();
int8_t get_pixel(int x, int y, int c);
void clear();

#endif