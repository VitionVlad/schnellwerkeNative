#include "spng.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <png.h>
#include <string.h>

int width, height;
png_byte color_type;
png_byte bit_depth;
png_bytep *row_pointers = NULL;

typedef struct {
    const unsigned char *buffer;
    size_t size;
    size_t position;
} MemoryBufferState;

static void user_read_data(png_structp png_ptr, png_bytep out_buffer, png_size_t bytes_to_read){
  MemoryBufferState *state = (MemoryBufferState *) png_get_io_ptr(png_ptr);

  if (state->position + bytes_to_read > state->size){
    png_error(png_ptr, "Attempt to read beyond end of memory buffer");
  }

  memcpy(out_buffer, state->buffer + state->position, bytes_to_read);
  state->position += bytes_to_read;
}

void parsepng(png_structp png, png_infop info){
  png_read_info(png, info);

  width      = png_get_image_width(png, info);
  height     = png_get_image_height(png, info);
  color_type = png_get_color_type(png, info);
  bit_depth  = png_get_bit_depth(png, info);

  if(bit_depth == 16)
    png_set_strip_16(png);

  if(color_type == PNG_COLOR_TYPE_PALETTE)
    png_set_palette_to_rgb(png);

  if(color_type == PNG_COLOR_TYPE_GRAY && bit_depth < 8)
    png_set_expand_gray_1_2_4_to_8(png);

  if(png_get_valid(png, info, PNG_INFO_tRNS))
    png_set_tRNS_to_alpha(png);

  if(color_type == PNG_COLOR_TYPE_RGB ||
     color_type == PNG_COLOR_TYPE_GRAY ||
     color_type == PNG_COLOR_TYPE_PALETTE)
    png_set_filler(png, 0xFF, PNG_FILLER_AFTER);

  if(color_type == PNG_COLOR_TYPE_GRAY ||
     color_type == PNG_COLOR_TYPE_GRAY_ALPHA)
    png_set_gray_to_rgb(png);

  png_read_update_info(png, info);

  if (row_pointers) abort();

  row_pointers = (png_bytep*)malloc(sizeof(png_bytep) * height);
  for(int y = 0; y < height; y++) {
    row_pointers[y] = (png_byte*)malloc(png_get_rowbytes(png,info));
  }

  png_read_image(png, row_pointers);
}

void read_png_file(const char *path){
  FILE *fp = fopen(path, "rb");
  png_structp png = png_create_read_struct(PNG_LIBPNG_VER_STRING, NULL, NULL, NULL);
  if(!png) abort();

  png_infop info = png_create_info_struct(png);
  if(!info) abort();

  if(setjmp(png_jmpbuf(png))) abort();

  png_init_io(png, fp);

  parsepng(png, info);

  fclose(fp);

  png_destroy_read_struct(&png, &info, NULL);
}

void parse_png_buffer(const unsigned char* data, uint32_t size){
  png_structp png = png_create_read_struct(PNG_LIBPNG_VER_STRING, NULL, NULL, NULL);
  if(!png) abort();

  png_infop info = png_create_info_struct(png);
  if(!info) abort();

  if(setjmp(png_jmpbuf(png))) abort();

  MemoryBufferState state = {
    .buffer   = data,
    .size     = size,
    .position = 0
  };

  png_set_read_fn(png, &state, (png_rw_ptr)user_read_data);

  parsepng(png, info);

  png_destroy_read_struct(&png, &info, NULL);
}

int getx(){
  return width;
}

int gety(){
  return height;
}

int8_t get_pixel(int x, int y, int c){
  png_bytep row = row_pointers[y];
  png_bytep px = &(row[x * 4]);
  return (int8_t) px[c];
}

void clear(){
  free(row_pointers);
  *row_pointers = NULL;
}