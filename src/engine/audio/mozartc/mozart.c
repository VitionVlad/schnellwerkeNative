#include "mozart.h"
#define MINIAUDIO_IMPLEMENTATION
#include <miniaudio.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct MozartHandle{
    ma_engine engine;
} MozartHandle;

typedef struct MozartSound{
    ma_sound sound;
    uint32_t sec;
} MozartSound;

struct Mozart{
    MozartHandle *mh;
    uint32_t mhn;
    MozartSound *ms;
    uint32_t msn;
} mz;

uint32_t newmozart(){
    uint32_t mhi = mz.mhn;
    if(mhi > 0){
        MozartHandle *tmp = malloc(sizeof(MozartHandle)*mhi);
        memcpy(tmp, mz.mh, sizeof(MozartHandle)*mhi);
        free(mz.mh);
        mz.mhn++;
        mz.mh = malloc(sizeof(MozartHandle)*mz.mhn);
        memcpy(mz.mh, tmp, sizeof(MozartHandle)*mhi);
        free(tmp);
    }else{
        mz.mhn++;
        mz.mh = malloc(sizeof(MozartHandle)*mz.mhn);
    }

    ma_result result;
    result = ma_engine_init(NULL, &mz.mh[mhi].engine);
    if (result == MA_SUCCESS) {
        printf("\e[1;36mMozart\e[0;37m: Miniaudio engine inited with success \n");
    }

    return mhi;
}

void mozartsetvolume(uint32_t mhi, float vol){
    ma_engine_set_volume(&mz.mh[mhi].engine, vol);
}

uint32_t newsound(uint32_t mhi, const char* path){
    uint32_t msn = mz.msn;
    if(msn > 0){
        MozartHandle *tmp = malloc(sizeof(MozartHandle)*msn);
        memcpy(tmp, mz.ms, sizeof(MozartHandle)*msn);
        free(mz.ms);
        mz.msn++;
        mz.ms = malloc(sizeof(MozartHandle)*mz.msn);
        memcpy(mz.ms, tmp, sizeof(MozartHandle)*msn);
        free(tmp);
    }else{
        mz.msn++;
        mz.ms = malloc(sizeof(MozartHandle)*mz.msn);
    }

    mz.ms[msn].sec = mhi;
    ma_result result = ma_sound_init_from_file(&mz.mh[mhi].engine, path, 0, NULL, NULL, &mz.ms[msn].sound);
    if (result == MA_SUCCESS) {
        printf("\e[1;36mMozartSound\e[0;37m: Sound created from file with success \n");
    }

    return msn;
}

void soundplay(uint32_t msn, float pan, float volume){
    ma_sound_set_pan(&mz.ms[msn].sound, pan);
    ma_sound_set_volume(&mz.ms[msn].sound, volume);
    ma_sound_start(&mz.ms[msn].sound);
}

void soudstop(uint32_t msn){
    ma_sound_stop(&mz.ms[msn].sound);
}

void destroymozart(uint32_t mhi){
    for(uint32_t i = 0; i != mz.msn; i++){
        if(mz.ms[i].sec == mhi){
            ma_sound_uninit(&mz.ms[i].sound);
        }
    }
    ma_engine_uninit(&mz.mh[mhi].engine);
}