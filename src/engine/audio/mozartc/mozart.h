#ifndef MOZART_H
#define MOZART_H
#include <stdint.h>

uint32_t newmozart();
void mozartsetvolume(uint32_t mhi, float vol);
void destroymozart(uint32_t mhi);
uint32_t newsound(uint32_t mhi, const char* path);
void soundplay(uint32_t msn, float pan, float volume);
void soudstop(uint32_t msn);
void destroymozart(uint32_t mhi);

#endif