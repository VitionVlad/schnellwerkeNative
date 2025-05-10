#ifndef EUCLID_H
#define EUCLID_H
#include <stdint.h>

float get_frametime(uint32_t eh);
uint32_t get_resx(uint32_t eh);
uint32_t get_resy(uint32_t eh);
void setresolution(uint32_t eh, uint32_t xs, uint32_t ys);
void setfullscreen(uint32_t eh);
void quitfullscreen(uint32_t eh);
uint8_t getKeyPressed(uint32_t eh, uint32_t index);
double get_mouse_posx(uint32_t eh);
double get_mouse_posy(uint32_t eh);
void req_mouse_lock(uint32_t eh);
void req_mouse_unlock(uint32_t eh);
void modifyshadowdata(uint32_t eh, uint32_t ncnt, uint32_t nres);
void modifydeffereddata(uint32_t eh, uint32_t ncnt, float nres);
void modifyshadowuniform(uint32_t eh, uint32_t pos, float value);
void modifydeffereduniform(uint32_t eh, uint32_t pos, float value);
uint32_t neweng(uint32_t shadowMapResolution);
void destroy(uint32_t eh);
uint32_t newmaterial(uint32_t eh, uint32_t *vert, uint32_t *frag, uint32_t *shadow, uint32_t svert, uint32_t sfrag, uint32_t sshadow, uint32_t cullmode);
uint32_t newmodel(uint32_t eh, float *vertices, float *uv, float *normals, uint32_t size);
void setmeshbuf(uint32_t eme, uint32_t i, float val);
uint32_t newmesh(uint32_t eh, uint32_t es, uint32_t em, uint32_t te, uint32_t usage);
uint32_t newtexture(uint32_t eh, uint32_t xsize, uint32_t ysize, uint32_t zsize, char *pixels);
uint32_t loopcont(uint32_t eh);

#endif