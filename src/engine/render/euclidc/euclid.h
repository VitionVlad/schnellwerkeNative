#ifndef EUCLID_H
#define EUCLID_H
#include <stdint.h>

uint32_t neweng();
uint32_t loopcont(uint32_t eh);
void destroy(uint32_t eh);
uint32_t newmaterial(uint32_t eh, uint32_t *vert, uint32_t *frag, uint32_t svert, uint32_t sfrag, uint32_t cullmode);
uint32_t newmodel(uint32_t eh, float *vertices, float *uv, float *normals, uint32_t size);

#endif