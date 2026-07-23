#include "euclid.h"
#include <pthread.h>
#include <stdatomic.h>
#include <time.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <vulkan/vulkan.h>
#include <GLFW/glfw3.h>
#include <GLFW/glfw3native.h>

const int MAX_FRAMES_IN_FLIGHT = 2;

enum {
    EUCLID_QUEUE_TYPE_MATERIAL = 0,
    EUCLID_QUEUE_TYPE_MODEL = 1,
    EUCLID_QUEUE_TYPE_MESH = 2,
    EUCLID_QUEUE_TYPE_TEXTURE = 3
};

typedef struct euclidCreationQueue{
    uint8_t type;
    uint32_t *vert;
    uint32_t *frag;
    uint32_t *shadow;
    uint32_t svert;
    uint32_t sfrag;
    uint32_t sshadow;
    uint32_t cullmode;
    uint32_t scullmode;
    float *vertices;
    float *uv;
    float *normals;
    uint32_t size;
    uint32_t xsize;
    uint32_t ysize;
    uint32_t zsize;
    uint32_t byteperpixel;
    char *pixels;
    uint8_t is3d;
    uint32_t imageformat;
    uint8_t genmips;
    uint32_t es;
    uint32_t em;
    uint32_t meshid;
    uint32_t *te;
    uint32_t tn;
    uint32_t usage;
} euclidCreationQueue;

typedef struct euclidh{
    VkInstance instance;
    VkPhysicalDevice *physicalDevices;
    uint32_t chosenDevice;
    uint32_t queueFamilyCount;
    uint32_t chosenqueuefam;
    uint32_t chosenpresentqueue;
    GLFWwindow *window;
    VkSurfaceKHR surface;
    VkDevice device;
    VkQueue graphicsQueue;
    VkQueue presentQueue;
    VkSwapchainKHR swapChain;
    VkImage *swapChainImages;
    uint32_t swapChainImageCount;
    VkExtent2D swapChainExtent;
    VkFormat swapChainImageFormat;
    uint32_t usedPresentMode;
    _Atomic uint32_t resolutionX[2];
    _Atomic uint32_t resolutionY[2];
    uint32_t sresolutionX;
    uint32_t sresolutionY;
    uint32_t oldx;
    uint32_t oldy;
    VkImageView *swapChainImageViews;
    VkRenderPass renderPass;
    VkImage depthImage;
    VkImageView depthImageView;
    VkDeviceMemory depthImageMemory;
    VkFramebuffer *swapChainFramebuffers;
    VkCommandBuffer *commandBuffers;
    VkCommandPool commandPool[5];
    VkSemaphore *imageAvailableSemaphores;
    VkSemaphore *renderFinishedSemaphores;
    VkFence *inFlightFences;
    uint32_t currentFrame;
    uint32_t imageIndex;
    uint32_t totalFrames;
    _Atomic uint32_t shadowMapResolution[2];
    uint32_t oldshadowMapResolution;
    float resolutionScale[2];
    float oldResolutionScale;
    uint32_t renderResolutionX;
    uint32_t renderResolutionY;
    _Atomic uint32_t shadowMapsCount[2];
    uint8_t enableShadowMaps;
    uint32_t lightsCount;
    uint32_t oldshadowMapsCount;
    _Atomic uint32_t defferedCount[2];
    uint32_t oldDefferedCount;
    VkImage shadowImage;
    VkImageView shadowImageView;
    VkImageView shadowRenderImageViews[100];
    VkDeviceMemory shadowImageMemory;
    VkFramebuffer shadowFramebuffers[100];
    VkRenderPass shadowRenderPass;
    float shadowMatrices[2][2400];
    uint8_t shadowslot;
    uint8_t usedshadowlot;
    VkBuffer shadowUniformBuffer;
    VkDeviceMemory shadowUniformBuffersMemory;
    void** shadowUniformBuffersMapped;
    VkRenderPass defferedRenderPass;
    float defferedMatrices[2][400];
    uint8_t defferedslot;
    uint8_t useddefferedlot;
    VkBuffer defferedUniformBuffer;
    VkDeviceMemory defferedUniformBuffersMemory;
    void** defferedUniformBuffersMapped;
    VkImage defferedImage;
    VkImage defferedDepthImage;
    VkImageView defferedImageView;
    VkImageView defferedDepthImageView;
    VkImageView defferedRenderImageViews[30];
    VkDeviceMemory defferedImageMemory;
    VkDeviceMemory defferedDepthImageMemory;
    VkImageView defferedDepthRenderImageViews[10];
    VkFramebuffer defferedFramebuffers[10];
    VkRenderPass lightingRenderPass;
    VkImage lightingImage;
    VkImage lightingDepthImage;
    VkImageView lightingImageViews[2];
    VkImageView lightingDepthImageViews[2];
    VkImageView lightingImageViewl;
    VkImageView lightingDepthImageViewl;
    VkDeviceMemory lightingImageMemory;
    VkDeviceMemory lightingDepthImageMemory;
    VkFramebuffer lightingFramebuffers[2];
    VkSampler attachmentSampler;
    uint8_t key_state[58];
    double xpos;
    double ypos;
    uint32_t mrec;
    float frametime;
    uint8_t right;
    uint8_t left;
    uint8_t middle;
    int acnt;
    float *axes;
    int bcnt;
    unsigned char *btnstats;
    uint8_t gamepaden;
    uint8_t debug;
    _Atomic uint8_t lightattn;
    pthread_t threads[4];
    _Atomic uint32_t lpcont;
    _Atomic uint8_t halt_render;
    euclidCreationQueue *eq;
    _Atomic uint32_t queuesize;
} euclidh;

typedef struct euclidmaterial{
    VkShaderModule vertModule;
    VkShaderModule fragModule;
    VkShaderModule shadowModule;
    uint32_t cullMode;
    uint32_t shcullMode;
    uint32_t polygonMode;
    float lineWidth;
} euclidmaterial;

typedef struct euclidmodel{
    VkDeviceMemory vertexBufferMemory;
    VkBuffer vertexBuffer;
    uint32_t vertnum;
} euclidmodel;

typedef struct euclidtexture{
    VkImage texture;
    VkDeviceMemory textureImageMemory;
    VkImageView textureImageView;
    VkSampler sampler;
    uint32_t mipLevels;
} euclidtexture;

typedef struct euclidmesh{
    uint32_t euclidid;
    VkDescriptorSetLayout descriptorSetLayout;
    VkPipelineLayout pipelineLayout;
    //VkPipelineLayout lightingPipelineLayout;
    VkPipelineLayout shadowPipelineLayout;
    VkPipelineLayout defferedPipelineLayout;
    VkPipeline graphicsPipeline;
    //VkPipeline lightingPipeline;
    VkPipeline shadowPipeline;
    VkPipeline defferedPipeline;
    uint32_t modelId;
    VkBuffer *uniformBuffers;
    VkDeviceMemory *uniformBuffersMemory;
    void** uniformBuffersMapped;
    VkDescriptorPool descriptorPool;
    VkDescriptorSet *descriptorSets;
    VkDescriptorPool shadowDescriptorPool;
    VkDescriptorSet shadowDescriptorSets[100];
    VkDescriptorSetLayout shadowDescriptorSetLayout;
    VkDescriptorPool defferedDescriptorPool;
    VkDescriptorPool lightingDescriptorPool;
    VkDescriptorSet defferedDescriptorSets[10];
    VkDescriptorSet lightingDescriptorSets;
    VkDescriptorSetLayout defferedDescriptorSetLayout;
    VkDescriptorSetLayout lightingDescriptorSetLayout;
    float lub[2][64];
    _Atomic uint8_t uniformslot;
    _Atomic uint8_t drawable;
    uint32_t usage;
    uint32_t mrec;
    _Atomic int8_t camrend;
    uint8_t paramslot;
    uint32_t savpapparam[2];
    uint32_t *savedtex;
    uint32_t texnm;
} euclidmesh;

struct euclidVK{
    euclidh *handle;
    uint32_t size;
    euclidmaterial *materials;
    uint32_t msize;
    uint32_t vmsize;
    euclidmodel *models;
    uint32_t mosize;
    uint32_t vmosize;
    euclidmesh *meshes;
    uint32_t mesize;
    uint32_t vmesize;
    euclidtexture *textures;
    uint32_t tsize;
    uint32_t vtsize;
} euclid;

float get_frametime(uint32_t eh){
    return euclid.handle[eh].frametime * 1000;
}

uint32_t get_resx(uint32_t eh){
    return euclid.handle[eh].resolutionX[0];
}

uint32_t get_resy(uint32_t eh){
    return euclid.handle[eh].resolutionY[0];
}

static int mini(int x, int y)
{
    return x < y ? x : y;
}

static int maxi(int x, int y)
{
    return x > y ? x : y;
}

GLFWmonitor* get_current_monitor(GLFWwindow *window)
{
    int nmonitors, i;
    int wx, wy, ww, wh;
    int mx, my, mw, mh;
    int overlap, bestoverlap;
    GLFWmonitor *bestmonitor;
    GLFWmonitor **monitors;
    const GLFWvidmode *mode;

    bestoverlap = 0;
    bestmonitor = NULL;

    glfwGetWindowPos(window, &wx, &wy);
    glfwGetWindowSize(window, &ww, &wh);
    monitors = glfwGetMonitors(&nmonitors);

    for (i = 0; i < nmonitors; i++) {
        mode = glfwGetVideoMode(monitors[i]);
        glfwGetMonitorPos(monitors[i], &mx, &my);
        mw = mode->width;
        mh = mode->height;

        overlap =
            maxi(0, mini(wx + ww, mx + mw) - maxi(wx, mx)) *
            maxi(0, mini(wy + wh, my + mh) - maxi(wy, my));

        if (bestoverlap < overlap) {
            bestoverlap = overlap;
            bestmonitor = monitors[i];
        }
    }

    return bestmonitor;
}

void setresolution(uint32_t eh, uint32_t xs, uint32_t ys){
    glfwSetWindowSize(euclid.handle[eh].window, xs, ys);
}

void settitle(uint32_t eh, const char* title){
    glfwSetWindowTitle(euclid.handle[eh].window, title);
}

void seticon(uint32_t eh, uint32_t xs, uint32_t ys, char *pixels){
    GLFWimage img;
    img.height = ys;
    img.width = xs;
    img.pixels = pixels;

    glfwSetWindowIcon(euclid.handle[eh].window, 1, &img);
}

void setfullscreen(uint32_t eh){
    const GLFWvidmode* mode = glfwGetVideoMode(get_current_monitor(euclid.handle[eh].window));
    euclid.handle[eh].sresolutionX = euclid.handle[eh].resolutionX[0];
    euclid.handle[eh].sresolutionY = euclid.handle[eh].resolutionY[0];
    glfwSetWindowAttrib(euclid.handle[eh].window, GLFW_DECORATED, GLFW_FALSE);
    glfwSetWindowAttrib(euclid.handle[eh].window, GLFW_RESIZABLE, GLFW_FALSE);
    glfwSetWindowSize(euclid.handle[eh].window, mode->width, mode->height+1);
    glfwSetWindowPos(euclid.handle[eh].window, 0, 0);
}

void quitfullscreen(uint32_t eh){
    euclid.handle[eh].resolutionX[1] = euclid.handle[eh].sresolutionX;
    euclid.handle[eh].resolutionY[1] = euclid.handle[eh].sresolutionY;
    glfwSetWindowAttrib(euclid.handle[eh].window, GLFW_DECORATED, GLFW_TRUE);
    glfwSetWindowAttrib(euclid.handle[eh].window, GLFW_RESIZABLE, GLFW_TRUE);
    glfwSetWindowSize(euclid.handle[eh].window, euclid.handle[eh].resolutionX[1], euclid.handle[eh].resolutionY[1]);
    glfwSetWindowPos(euclid.handle[eh].window, 50, 50);
}

uint8_t getKeyPressed(uint32_t eh, uint32_t index){
    return euclid.handle[eh].key_state[index];
}

uint8_t getmr(uint32_t eh){
    return euclid.handle[eh].right;
}

uint8_t getml(uint32_t eh){
    return euclid.handle[eh].left;
}

uint8_t getmm(uint32_t eh){
    return euclid.handle[eh].middle;
}

double get_mouse_posx(uint32_t eh){
    return euclid.handle[eh].xpos;
}

double get_mouse_posy(uint32_t eh){
    return euclid.handle[eh].ypos;
}

void req_mouse_lock(uint32_t eh){
    glfwSetInputMode(euclid.handle[eh].window, GLFW_CURSOR, GLFW_CURSOR_DISABLED);
}

void req_mouse_unlock(uint32_t eh){
    glfwSetInputMode(euclid.handle[eh].window, GLFW_CURSOR, GLFW_CURSOR_NORMAL);
}

uint32_t getKey(uint32_t glfwkey){
    if(glfwkey >= 290 && glfwkey <= 301){
        return glfwkey - 290;
    }else if(glfwkey >= 48 && glfwkey <= 57){
        return glfwkey - 36;
    }else if(glfwkey >= 65 && glfwkey <= 90){
        return glfwkey - 43;
    }else{
        switch(glfwkey){
            case GLFW_KEY_SPACE:
                return 48;
                break;
            case GLFW_KEY_ESCAPE:
                return 49;
                break;
            case GLFW_KEY_LEFT_SHIFT:
                return 50;
                break;
            case GLFW_KEY_RIGHT_SHIFT:
                return 50;
                break;
            case GLFW_KEY_LEFT_CONTROL:
                return 51;
                break;
            case GLFW_KEY_RIGHT_CONTROL:
                return 51;
                break;
            case GLFW_KEY_UP:
                return 52;
                break;
            case GLFW_KEY_LEFT:
                return 53;
                break;
            case GLFW_KEY_DOWN:
                return 54;
                break;
            case GLFW_KEY_RIGHT:
                return 55;
                break;
            case GLFW_KEY_ENTER:
                return 56;
                break;
            case GLFW_KEY_BACKSPACE:
                return 57;
                break;
        }
    }
    return 0;
}

void setk(uint32_t eh, uint32_t key, uint32_t state){
    switch(state){
        case GLFW_PRESS:
            euclid.handle[eh].key_state[getKey(key)] = 1;
            break;
        default:
            euclid.handle[eh].key_state[getKey(key)] = 0;
            break;
    }
}

void keywork(uint32_t eh){
    uint32_t btstate = 0;
    for(uint32_t i = GLFW_KEY_F1; i != GLFW_KEY_F13; i++){
        btstate = glfwGetKey(euclid.handle[eh].window, i);
        setk(eh, i, btstate);
    }
    for(uint32_t i = GLFW_KEY_0; i != GLFW_KEY_9; i++){
        btstate = glfwGetKey(euclid.handle[eh].window, i);
        setk(eh, i, btstate);
    }
    for(uint32_t i = GLFW_KEY_A; i != GLFW_KEY_Z; i++){
        btstate = glfwGetKey(euclid.handle[eh].window, i);
        setk(eh, i, btstate);
    }
    btstate = glfwGetKey(euclid.handle[eh].window, GLFW_KEY_SPACE);
    setk(eh, GLFW_KEY_SPACE, btstate);
    btstate = glfwGetKey(euclid.handle[eh].window, GLFW_KEY_ESCAPE);
    setk(eh, GLFW_KEY_ESCAPE, btstate);
    btstate = glfwGetKey(euclid.handle[eh].window, GLFW_KEY_LEFT_SHIFT);
    setk(eh, GLFW_KEY_LEFT_SHIFT, btstate);
    btstate = glfwGetKey(euclid.handle[eh].window, GLFW_KEY_RIGHT_SHIFT);
    setk(eh, GLFW_KEY_RIGHT_SHIFT, btstate);
    btstate = glfwGetKey(euclid.handle[eh].window, GLFW_KEY_LEFT_CONTROL);
    setk(eh, GLFW_KEY_LEFT_CONTROL, btstate);
    btstate = glfwGetKey(euclid.handle[eh].window, GLFW_KEY_RIGHT_CONTROL);
    setk(eh, GLFW_KEY_RIGHT_CONTROL, btstate);
    btstate = glfwGetKey(euclid.handle[eh].window, GLFW_KEY_UP);
    setk(eh, GLFW_KEY_UP, btstate);
    btstate = glfwGetKey(euclid.handle[eh].window, GLFW_KEY_LEFT);
    setk(eh, GLFW_KEY_LEFT, btstate);
    btstate = glfwGetKey(euclid.handle[eh].window, GLFW_KEY_DOWN);
    setk(eh, GLFW_KEY_DOWN, btstate);
    btstate = glfwGetKey(euclid.handle[eh].window, GLFW_KEY_RIGHT);
    setk(eh, GLFW_KEY_RIGHT, btstate);
    btstate = glfwGetKey(euclid.handle[eh].window, GLFW_KEY_ENTER);
    setk(eh, GLFW_KEY_ENTER, btstate);
    btstate = glfwGetKey(euclid.handle[eh].window, GLFW_KEY_BACKSPACE);
    setk(eh, GLFW_KEY_BACKSPACE, btstate);
    btstate = glfwGetMouseButton(euclid.handle[eh].window, GLFW_MOUSE_BUTTON_RIGHT);
    switch(btstate){
        case GLFW_PRESS:
            euclid.handle[eh].right = 1;
            break;
        default:
            euclid.handle[eh].right = 0;
            break;
    }
    btstate = glfwGetMouseButton(euclid.handle[eh].window, GLFW_MOUSE_BUTTON_MIDDLE);
    switch(btstate){
        case GLFW_PRESS:
            euclid.handle[eh].middle = 1;
            break;
        default:
            euclid.handle[eh].middle = 0;
            break;
    }
    btstate = glfwGetMouseButton(euclid.handle[eh].window, GLFW_MOUSE_BUTTON_LEFT);
    switch(btstate){
        case GLFW_PRESS:
            euclid.handle[eh].left = 1;
            break;
        default:
            euclid.handle[eh].left = 0;
            break;
    }
    glfwGetCursorPos(euclid.handle[eh].window, &euclid.handle[eh].xpos, &euclid.handle[eh].ypos);
    euclid.handle[eh].gamepaden = 0;
    if(glfwJoystickPresent(GLFW_JOYSTICK_1)){
        euclid.handle[eh].gamepaden = 1;
        euclid.handle[eh].axes = glfwGetJoystickAxes(GLFW_JOYSTICK_1, &euclid.handle[eh].acnt);
        euclid.handle[eh].btnstats = glfwGetJoystickButtons(GLFW_JOYSTICK_1, &euclid.handle[eh].bcnt);
        //if(euclid.handle[eh].acnt > 0){
        //    for(uint8_t i = 0; i != euclid.handle[eh].acnt; i++){
        //        printf("%f, ", euclid.handle[eh].axes[i]);
        //    }
        //    printf(" | ");
        //    for(uint8_t i = 0; i != euclid.handle[eh].bcnt; i++){
        //        printf("%d, ", euclid.handle[eh].btnstats[i]);
        //    }
        //    printf("\n");
        //}
    }
}

float get_axis(uint32_t eh, uint8_t n){
    if(euclid.handle[eh].acnt < n){
        return 0;
    }
    return euclid.handle[eh].axes[n];
}

unsigned char get_button(uint32_t eh, uint8_t n){
    if(euclid.handle[eh].bcnt < n){
        return 0;
    }
    return euclid.handle[eh].btnstats[n];
}

uint8_t gamepad_con(uint32_t eh){
    return euclid.handle[eh].gamepaden;
}

uint8_t gamepad_axisnm(uint32_t eh){
    return euclid.handle[eh].acnt;
}

uint8_t gamepad_buttonnm(uint32_t eh){
    return euclid.handle[eh].bcnt;
}

uint32_t findMemoryType(uint32_t typeFilter, VkMemoryPropertyFlags properties, uint32_t eh) {
    VkPhysicalDeviceMemoryProperties memProperties;
    vkGetPhysicalDeviceMemoryProperties(euclid.handle[eh].physicalDevices[euclid.handle[eh].chosenDevice], &memProperties);
    for (uint32_t i = 0; i < memProperties.memoryTypeCount; i++) {
        if ((typeFilter & (1 << i)) && (memProperties.memoryTypes[i].propertyFlags & properties) == properties) {
            return i;
        }
    }
    if (euclid.handle[eh].debug == 1) printf("\e[1;31mError\e[0;37m: Cant find suitable memory");
    exit(-1);
}

void createInstance(uint32_t eh){
    VkApplicationInfo appinfo = {0};
    appinfo.apiVersion = VK_API_VERSION_1_4;
    appinfo.applicationVersion = VK_MAKE_VERSION(0, 1, 0);
    appinfo.engineVersion = VK_MAKE_VERSION(0, 1, 0);
    appinfo.pApplicationName = "Schnellwerke3n";
    appinfo.pEngineName = "euclidRender";
    appinfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
    appinfo.pNext = NULL;

    uint32_t layerCount;
    vkEnumerateInstanceLayerProperties(&layerCount, NULL);
    VkLayerProperties *layers = malloc(sizeof(VkLayerProperties)*layerCount);
    vkEnumerateInstanceLayerProperties(&layerCount, layers);

    char **nms = malloc(sizeof(char*)*layerCount);
    for(uint32_t i = 0; i != layerCount; i++){
        nms[i] = layers[i].layerName;
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Enabled validation layer %s\n", layers[i].layerName);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Validation layer description %s\n", layers[i].description);
    }
    
    VkInstanceCreateInfo createInfo = {0};
    createInfo.enabledExtensionCount = 0;
    createInfo.ppEnabledExtensionNames = NULL;
    createInfo.enabledLayerCount = 0;
    createInfo.ppEnabledLayerNames = (const char *const *) nms;
    createInfo.pApplicationInfo = &appinfo;
    createInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
    createInfo.pNext = NULL;
    createInfo.flags = 0;

    uint32_t lrnm = 0;
    vkEnumerateInstanceExtensionProperties(NULL, &lrnm, NULL);
    VkExtensionProperties *extprop = malloc(sizeof(VkExtensionProperties)*lrnm);
    vkEnumerateInstanceExtensionProperties(NULL, &lrnm, extprop);
    createInfo.enabledExtensionCount = lrnm;
    char** extnms = malloc(sizeof(char*)*lrnm);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Enabled extensions count = %d\n", lrnm);
    for(int i = 0; i != lrnm; i++){
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Enabled extension %s\n", extprop[i].extensionName);
        extnms[i] = extprop[i].extensionName;
    }
    createInfo.ppEnabledExtensionNames = (const char *const *) extnms;

    VkResult result = vkCreateInstance(&createInfo, NULL, &euclid.handle[eh].instance);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Vulkan instance created with result %d \n", result);
    free(extprop);
    free(extnms);
}

void getDevice(uint32_t eh){
    uint32_t dn = 0;
    vkEnumeratePhysicalDevices(euclid.handle[eh].instance, &dn, NULL);
    euclid.handle[eh].physicalDevices = malloc(sizeof(VkPhysicalDevice)*dn);
    vkEnumeratePhysicalDevices(euclid.handle[eh].instance, &dn, euclid.handle[eh].physicalDevices);
    if(euclid.handle[eh].chosenDevice == -1){
        int dt = 0;
        for(int i = 0; i != dn; i++){
            VkPhysicalDeviceProperties deviceProperties;
            vkGetPhysicalDeviceProperties(euclid.handle[eh].physicalDevices[i], &deviceProperties);
            if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Device id = %d\n", i);
            if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Device name = %s\n", deviceProperties.deviceName);
            if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Device api version = %d\n", deviceProperties.apiVersion);
            if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Device type = %d\n", deviceProperties.deviceType);
            if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Device maxUniformBufferRange = %d\n", deviceProperties.limits.maxUniformBufferRange);
            vkGetPhysicalDeviceQueueFamilyProperties(euclid.handle[eh].physicalDevices[i], &euclid.handle[eh].queueFamilyCount, NULL);

            VkQueueFamilyProperties *queueFamilies = malloc(sizeof(VkQueueFamilyProperties)*euclid.handle[eh].queueFamilyCount);
            vkGetPhysicalDeviceQueueFamilyProperties(euclid.handle[eh].physicalDevices[i], &euclid.handle[eh].queueFamilyCount, queueFamilies);

            for(int j = 0; j != euclid.handle[eh].queueFamilyCount; j++){
                if(queueFamilies[j].queueFlags & VK_QUEUE_GRAPHICS_BIT){ 
                    euclid.handle[eh].chosenqueuefam = j;
                    if(euclid.handle[eh].chosenDevice == -1 || (deviceProperties.deviceType == 1 && dt != 2) || deviceProperties.deviceType == 2){
                        euclid.handle[eh].chosenDevice = i;
                        dt = deviceProperties.deviceType;
                    }
                }
            }
            free(queueFamilies);
        }
        if(euclid.handle[eh].chosenDevice == -1){
            if (euclid.handle[eh].debug == 1) printf("\e[1;31mError\e[0;37m: Can not find a suitable device");
            exit(-1);
        }
    }
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Chosen physical device id = %d\n", euclid.handle[eh].chosenDevice);
}

void getPresentFamily(uint32_t eh){
    VkBool32 presentSupport = VK_FALSE;
    for(int i = 0; i != euclid.handle[eh].queueFamilyCount; i++){
        vkGetPhysicalDeviceSurfaceSupportKHR(euclid.handle[eh].physicalDevices[euclid.handle[eh].chosenDevice], i, euclid.handle[eh].surface, &presentSupport);
        if(presentSupport == VK_TRUE){
            euclid.handle[eh].chosenpresentqueue = i;
            break;
        }
    }
}

void createDevice(uint32_t eh){
    VkDeviceQueueCreateInfo queueCreateInfo[2];
    queueCreateInfo[0].sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
    queueCreateInfo[0].queueFamilyIndex = euclid.handle[eh].chosenqueuefam;
    queueCreateInfo[0].queueCount = 1;
    float queuePriority = 1.0f;
    queueCreateInfo[0].pQueuePriorities = &queuePriority;
    queueCreateInfo[0].flags = 0;
    queueCreateInfo[0].pNext = NULL;

    queueCreateInfo[1].sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
    queueCreateInfo[1].queueFamilyIndex = euclid.handle[eh].chosenpresentqueue;
    queueCreateInfo[1].queueCount = 1;
    queueCreateInfo[1].pQueuePriorities = &queuePriority;
    queueCreateInfo[1].flags = 0;
    queueCreateInfo[1].pNext = NULL;

    VkPhysicalDeviceFeatures deviceFeatures = {0};

    uint32_t extensionCount = 0;
    vkEnumerateDeviceExtensionProperties(euclid.handle[eh].physicalDevices[euclid.handle[eh].chosenDevice], NULL, &extensionCount, NULL);

    VkExtensionProperties *extprop = malloc(sizeof(VkExtensionProperties)*extensionCount);
    vkEnumerateDeviceExtensionProperties(euclid.handle[eh].physicalDevices[euclid.handle[eh].chosenDevice], NULL, &extensionCount, extprop);

    char** extnms = malloc(sizeof(char*)*extensionCount);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Enabled device extensions count = %d\n", extensionCount);
    for(int i = 0; i != extensionCount; i++){
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Enabled device extension(%d) %s\n", i, extprop[i].extensionName);
        extnms[i] = extprop[i].extensionName;
    }

    VkDeviceCreateInfo createInfo = {0};
    createInfo.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
    createInfo.queueCreateInfoCount = 2;
    if(euclid.handle[eh].chosenqueuefam == euclid.handle[eh].chosenpresentqueue){
        createInfo.queueCreateInfoCount = 1;
    }
    createInfo.pQueueCreateInfos = queueCreateInfo;
    createInfo.pEnabledFeatures = &deviceFeatures;
    createInfo.enabledExtensionCount = extensionCount;
    createInfo.ppEnabledExtensionNames = (const char *const *) extnms;
    createInfo.enabledLayerCount = 0;
    createInfo.ppEnabledLayerNames = NULL;
    createInfo.pNext = NULL;
    createInfo.flags = 0;
    VkResult result = vkCreateDevice(euclid.handle[eh].physicalDevices[euclid.handle[eh].chosenDevice], &createInfo, NULL, &euclid.handle[eh].device);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Device created with result = %d\n", result);
    vkGetDeviceQueue(euclid.handle[eh].device, euclid.handle[eh].chosenqueuefam, 0, &euclid.handle[eh].graphicsQueue);
    vkGetDeviceQueue(euclid.handle[eh].device, euclid.handle[eh].chosenpresentqueue, 0, &euclid.handle[eh].presentQueue);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Chosen present queue = %d\n", euclid.handle[eh].chosenpresentqueue);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Chosen queue family with result = %d\n", euclid.handle[eh].chosenqueuefam);
    free(extnms);
    free(extprop);
}

void createSwapChain(uint32_t eh){
    VkSurfaceCapabilitiesKHR capabilities;
    vkGetPhysicalDeviceSurfaceCapabilitiesKHR(euclid.handle[eh].physicalDevices[euclid.handle[eh].chosenDevice], euclid.handle[eh].surface, &capabilities);

    uint32_t formatCount = 0;
    uint32_t chosenFormat = 0;
    VkSurfaceFormatKHR *formats;
    vkGetPhysicalDeviceSurfaceFormatsKHR(euclid.handle[eh].physicalDevices[euclid.handle[eh].chosenDevice], euclid.handle[eh].surface, &formatCount, NULL);
    if (formatCount != 0) {
        formats = malloc(sizeof(VkSurfaceFormatKHR)*formatCount);
        vkGetPhysicalDeviceSurfaceFormatsKHR(euclid.handle[eh].physicalDevices[euclid.handle[eh].chosenDevice], euclid.handle[eh].surface, &formatCount, formats);
        for(int i = 0; i != formatCount; i++){
            if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Avaible format = %d avaible color space = %d\n", formats[i].format, formats[i].colorSpace);
            if (formats[i].format == VK_FORMAT_B8G8R8A8_SRGB && formats[i].colorSpace == VK_COLOR_SPACE_SRGB_NONLINEAR_KHR) {
                chosenFormat = i;
                break;
            }
        }
    }else{
        if (euclid.handle[eh].debug == 1) printf("\e[1;31mError\e[0;37m: No formats avaible");
        exit(-1);
    }

    uint32_t presentModeCount;
    VkPresentModeKHR *modes;
    vkGetPhysicalDeviceSurfacePresentModesKHR(euclid.handle[eh].physicalDevices[euclid.handle[eh].chosenDevice], euclid.handle[eh].surface, &presentModeCount, NULL);
    if (presentModeCount != 0) {
        modes = malloc(sizeof(VkPresentModeKHR)*presentModeCount);
        vkGetPhysicalDeviceSurfacePresentModesKHR(euclid.handle[eh].physicalDevices[euclid.handle[eh].chosenDevice], euclid.handle[eh].surface, &presentModeCount, modes);
        for(int i = 0; i != presentModeCount; i++){
            if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Present mode avaible = %d\n", modes[i]);
        }
    }else{
        if (euclid.handle[eh].debug == 1) printf("\e[1;31mError\e[0;37m: No present mode avaible");
        exit(-1);
    }

    uint32_t imageCount = capabilities.minImageCount+1;
    if (capabilities.maxImageCount > 0 && imageCount > capabilities.maxImageCount) {
        imageCount = capabilities.maxImageCount;
    }

    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: SwapChain image count = %d\n", imageCount);

    VkSwapchainCreateInfoKHR createInfo = {0};
    createInfo.sType = VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;
    createInfo.surface = euclid.handle[eh].surface;
    createInfo.minImageCount = imageCount;
    createInfo.imageFormat = formats[chosenFormat].format;
    createInfo.imageColorSpace = formats[chosenFormat].colorSpace;
    createInfo.imageExtent.width = euclid.handle[eh].resolutionX[0];
    createInfo.imageExtent.height = euclid.handle[eh].resolutionY[0];
    createInfo.imageArrayLayers = 1;
    createInfo.imageUsage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;

    uint32_t queueFamilyIndices[] = {euclid.handle[eh].chosenqueuefam, euclid.handle[eh].chosenpresentqueue};

    if (euclid.handle[eh].chosenqueuefam != euclid.handle[eh].chosenpresentqueue) {
        createInfo.imageSharingMode = VK_SHARING_MODE_CONCURRENT;
        createInfo.queueFamilyIndexCount = 2;
        createInfo.pQueueFamilyIndices = queueFamilyIndices;
    } else {
        createInfo.imageSharingMode = VK_SHARING_MODE_EXCLUSIVE;
        createInfo.queueFamilyIndexCount = 0;
        createInfo.pQueueFamilyIndices = NULL;
    }

    createInfo.preTransform = capabilities.currentTransform;
    createInfo.compositeAlpha = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
    createInfo.presentMode = modes[euclid.handle[eh].usedPresentMode];
    createInfo.clipped = VK_TRUE;
    createInfo.oldSwapchain = VK_NULL_HANDLE;
    createInfo.pNext = NULL;
    createInfo.flags = 0;

    VkResult result = vkCreateSwapchainKHR(euclid.handle[eh].device, &createInfo, NULL, &euclid.handle[eh].swapChain);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: SwapChain created with result = %d\n", result);
    vkGetSwapchainImagesKHR(euclid.handle[eh].device, euclid.handle[eh].swapChain, &imageCount, NULL);
    euclid.handle[eh].swapChainImages = malloc(sizeof(VkImage)*imageCount);
    vkGetSwapchainImagesKHR(euclid.handle[eh].device, euclid.handle[eh].swapChain, &imageCount, euclid.handle[eh].swapChainImages);
    euclid.handle[eh].swapChainImageFormat = formats[chosenFormat].format;
    euclid.handle[eh].swapChainExtent = capabilities.currentExtent;
    euclid.handle[eh].swapChainImageCount = imageCount;
    euclid.handle[eh].oldx = euclid.handle[eh].resolutionX[0];
    euclid.handle[eh].oldy = euclid.handle[eh].resolutionY[0];
    free(modes);
}

void createSwapChainImageViews(uint32_t eh){
    euclid.handle[eh].swapChainImageViews = malloc(sizeof(VkImageView)*euclid.handle[eh].swapChainImageCount);
    for (int i = 0; i < euclid.handle[eh].swapChainImageCount; i++) {
        VkImageViewCreateInfo createInfo = {0};
        createInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
        createInfo.image = euclid.handle[eh].swapChainImages[i];
        createInfo.viewType = VK_IMAGE_VIEW_TYPE_2D;
        createInfo.format = euclid.handle[eh].swapChainImageFormat;
        createInfo.components.r = VK_COMPONENT_SWIZZLE_IDENTITY;
        createInfo.components.g = VK_COMPONENT_SWIZZLE_IDENTITY;
        createInfo.components.b = VK_COMPONENT_SWIZZLE_IDENTITY;
        createInfo.components.a = VK_COMPONENT_SWIZZLE_IDENTITY;
        createInfo.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
        createInfo.subresourceRange.baseMipLevel = 0;
        createInfo.subresourceRange.levelCount = 1;
        createInfo.subresourceRange.baseArrayLayer = 0;
        createInfo.subresourceRange.layerCount = 1;
        createInfo.flags = 0;
        createInfo.pNext = NULL;
        VkResult result = vkCreateImageView(euclid.handle[eh].device, &createInfo, NULL, &euclid.handle[eh].swapChainImageViews[i]);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: SwapChain Image View %d created with result = %d\n", i, result);
    }
}

void createRenderPass(uint32_t eh){
    VkAttachmentDescription attachments[2];
    attachments[0].format = euclid.handle[eh].swapChainImageFormat;
    attachments[0].samples = VK_SAMPLE_COUNT_1_BIT;
    attachments[0].loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR;
    attachments[0].storeOp = VK_ATTACHMENT_STORE_OP_STORE;
    attachments[0].stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE;
    attachments[0].stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE;
    attachments[0].initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    attachments[0].finalLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
    attachments[0].flags = 0;

    attachments[1].format = VK_FORMAT_D32_SFLOAT;
    attachments[1].samples = VK_SAMPLE_COUNT_1_BIT;
    attachments[1].loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR;
    attachments[1].storeOp = VK_ATTACHMENT_STORE_OP_DONT_CARE;
    attachments[1].stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE;
    attachments[1].stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE;
    attachments[1].initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    attachments[1].finalLayout = VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
    attachments[1].flags = 0;

    VkAttachmentReference depthAttachmentRef = {0};
    depthAttachmentRef.attachment = 1;
    depthAttachmentRef.layout = VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL;

    VkAttachmentReference colorAttachmentRef = {0};
    colorAttachmentRef.attachment = 0;
    colorAttachmentRef.layout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;

    VkSubpassDescription subpass = {0};
    subpass.pipelineBindPoint = VK_PIPELINE_BIND_POINT_GRAPHICS;
    subpass.colorAttachmentCount = 1;
    subpass.pColorAttachments = &colorAttachmentRef;
    subpass.pDepthStencilAttachment = &depthAttachmentRef;
    subpass.flags = 0;
    subpass.pInputAttachments = NULL;
    subpass.pPreserveAttachments = NULL;
    subpass.preserveAttachmentCount = 0;
    subpass.pResolveAttachments = NULL;

    VkSubpassDependency dependency = {0};
    dependency.srcSubpass = VK_SUBPASS_EXTERNAL;
    dependency.dstSubpass = 0;
    dependency.srcAccessMask = 0;
    dependency.srcStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT | VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT;
    dependency.dstStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT | VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT;
    dependency.dstAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT | VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT;
    dependency.dependencyFlags = 0;

    VkRenderPassCreateInfo renderPassInfo = {0};
    renderPassInfo.sType = VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO;
    renderPassInfo.attachmentCount = 2;
    renderPassInfo.pAttachments = attachments;
    renderPassInfo.subpassCount = 1;
    renderPassInfo.pSubpasses = &subpass;
    renderPassInfo.dependencyCount = 1;
    renderPassInfo.pDependencies = &dependency;
    renderPassInfo.flags = 0;
    renderPassInfo.pNext = NULL;

    VkResult result = vkCreateRenderPass(euclid.handle[eh].device, &renderPassInfo, NULL, &euclid.handle[eh].renderPass);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Renderpass created with result = %d\n", result);
}

void createShadowRenderPass(uint32_t eh){
    VkAttachmentDescription attachments;
    attachments.format = VK_FORMAT_D32_SFLOAT;
    attachments.samples = VK_SAMPLE_COUNT_1_BIT;
    attachments.loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR;
    attachments.storeOp = VK_ATTACHMENT_STORE_OP_STORE;
    attachments.stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE;
    attachments.stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE;
    attachments.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    attachments.finalLayout = VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
    attachments.flags = 0;

    VkAttachmentReference depthAttachmentRef = {0};
    depthAttachmentRef.attachment = 0;
    depthAttachmentRef.layout = VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL;

    VkSubpassDescription subpass = {0};
    subpass.pipelineBindPoint = VK_PIPELINE_BIND_POINT_GRAPHICS;
    subpass.colorAttachmentCount = 0;
    subpass.pColorAttachments = NULL;
    subpass.pDepthStencilAttachment = &depthAttachmentRef;
    subpass.flags = 0;
    subpass.pInputAttachments = NULL;
    subpass.pPreserveAttachments = NULL;
    subpass.preserveAttachmentCount = 0;
    subpass.pResolveAttachments = NULL;

    VkSubpassDependency dependency = {0};
    dependency.srcSubpass = VK_SUBPASS_EXTERNAL;
    dependency.dstSubpass = 0;
    dependency.srcAccessMask = 0;
    dependency.srcStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT | VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT;
    dependency.dstStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT | VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT;
    dependency.dstAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT | VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT;
    dependency.dependencyFlags = 0;

    VkRenderPassCreateInfo renderPassInfo = {0};
    renderPassInfo.sType = VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO;
    renderPassInfo.attachmentCount = 1;
    renderPassInfo.pAttachments = &attachments;
    renderPassInfo.subpassCount = 1;
    renderPassInfo.pSubpasses = &subpass;
    renderPassInfo.dependencyCount = 1;
    renderPassInfo.pDependencies = &dependency;
    renderPassInfo.flags = 0;
    renderPassInfo.pNext = NULL;

    VkResult result = vkCreateRenderPass(euclid.handle[eh].device, &renderPassInfo, NULL, &euclid.handle[eh].shadowRenderPass);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Shadow renderpass created with result = %d\n", result);
}

void createDefferedRenderPass(uint32_t eh){
    VkAttachmentDescription attachments[5];
    for(uint32_t i = 0; i != 3; i++){
        attachments[i].format = VK_FORMAT_R16G16B16A16_SFLOAT;
        attachments[i].samples = VK_SAMPLE_COUNT_1_BIT;
        attachments[i].loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR;
        attachments[i].storeOp = VK_ATTACHMENT_STORE_OP_STORE;
        attachments[i].stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE;
        attachments[i].stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE;
        attachments[i].initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
        attachments[i].finalLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
        attachments[i].flags = 0;
    }

    attachments[3].format = VK_FORMAT_D32_SFLOAT;
    attachments[3].samples = VK_SAMPLE_COUNT_1_BIT;
    attachments[3].loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR;
    attachments[3].storeOp = VK_ATTACHMENT_STORE_OP_STORE;
    attachments[3].stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE;
    attachments[3].stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE;
    attachments[3].initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    attachments[3].finalLayout = VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
    attachments[3].flags = 0;

    VkAttachmentReference attref[4] = {0};
    for(uint32_t i = 0; i != 3; i++){
        attref[i].attachment = i;
        attref[i].layout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
    }

    VkAttachmentReference depthAttachmentRef = {0};
    depthAttachmentRef.attachment = 3;
    depthAttachmentRef.layout = VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL;

    VkSubpassDescription subpass = {0};
    subpass.pipelineBindPoint = VK_PIPELINE_BIND_POINT_GRAPHICS;
    subpass.colorAttachmentCount = 3;
    subpass.pColorAttachments = attref;
    subpass.pDepthStencilAttachment = &depthAttachmentRef;
    subpass.flags = 0;
    subpass.pInputAttachments = NULL;
    subpass.pPreserveAttachments = NULL;
    subpass.preserveAttachmentCount = 0;
    subpass.pResolveAttachments = NULL;

    VkSubpassDependency dependency = {0};
    dependency.srcSubpass = VK_SUBPASS_EXTERNAL;
    dependency.dstSubpass = 0;
    dependency.srcAccessMask = 0;
    dependency.srcStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT | VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT;
    dependency.dstStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT | VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT;
    dependency.dstAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT | VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT;
    dependency.dependencyFlags = 0;

    VkRenderPassCreateInfo renderPassInfo = {0};
    renderPassInfo.sType = VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO;
    renderPassInfo.attachmentCount = 4;
    renderPassInfo.pAttachments = attachments;
    renderPassInfo.subpassCount = 1;
    renderPassInfo.pSubpasses = &subpass;
    renderPassInfo.dependencyCount = 1;
    renderPassInfo.pDependencies = &dependency;
    renderPassInfo.flags = 0;
    renderPassInfo.pNext = NULL;

    VkResult result = vkCreateRenderPass(euclid.handle[eh].device, &renderPassInfo, NULL, &euclid.handle[eh].defferedRenderPass);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Deffered renderpass created with result = %d\n", result);
}

void createLightingRenderPass(uint32_t eh){
    VkAttachmentDescription attachments[2] = {0};

    attachments[0].format = VK_FORMAT_R16G16B16A16_SFLOAT;
    attachments[0].samples = VK_SAMPLE_COUNT_1_BIT;
    attachments[0].loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR;
    attachments[0].storeOp = VK_ATTACHMENT_STORE_OP_STORE;
    attachments[0].stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE;
    attachments[0].stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE;
    attachments[0].initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    attachments[0].finalLayout = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
    attachments[0].flags = 0;

    attachments[1].format = VK_FORMAT_D32_SFLOAT;
    attachments[1].samples = VK_SAMPLE_COUNT_1_BIT;
    attachments[1].loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR;
    attachments[1].storeOp = VK_ATTACHMENT_STORE_OP_STORE;
    attachments[1].stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE;
    attachments[1].stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE;
    attachments[1].initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    attachments[1].finalLayout = VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
    attachments[1].flags = 0;

    VkAttachmentReference colorAttachmentRef = {0};
    colorAttachmentRef.attachment = 0;
    colorAttachmentRef.layout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;

    VkAttachmentReference depthAttachmentRef = {0};
    depthAttachmentRef.attachment = 1;
    depthAttachmentRef.layout = VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL;

    VkSubpassDescription subpass = {0};
    subpass.pipelineBindPoint = VK_PIPELINE_BIND_POINT_GRAPHICS;
    subpass.colorAttachmentCount = 1;
    subpass.pColorAttachments = &colorAttachmentRef;
    subpass.pDepthStencilAttachment = &depthAttachmentRef;
    subpass.flags = 0;
    subpass.pInputAttachments = NULL;
    subpass.pPreserveAttachments = NULL;
    subpass.preserveAttachmentCount = 0;
    subpass.pResolveAttachments = NULL;

    VkSubpassDependency dependency = {0};
    dependency.srcSubpass = VK_SUBPASS_EXTERNAL;
    dependency.dstSubpass = 0;
    dependency.srcAccessMask = 0;
    dependency.srcStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT | VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT;
    dependency.dstStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT | VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT;
    dependency.dstAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT | VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT;
    dependency.dependencyFlags = 0;

    VkRenderPassCreateInfo renderPassInfo = {0};
    renderPassInfo.sType = VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO;
    renderPassInfo.attachmentCount = 2;
    renderPassInfo.pAttachments = attachments;
    renderPassInfo.subpassCount = 1;
    renderPassInfo.pSubpasses = &subpass;
    renderPassInfo.dependencyCount = 1;
    renderPassInfo.pDependencies = &dependency;
    renderPassInfo.flags = 0;
    renderPassInfo.pNext = NULL;

    VkResult result = vkCreateRenderPass(euclid.handle[eh].device, &renderPassInfo, NULL, &euclid.handle[eh].lightingRenderPass);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Lighting renderpass created with result = %d\n", result);
}

void createShadowData(uint32_t eh){
    uint32_t queueFamilyIndices[] = {euclid.handle[eh].chosenqueuefam, euclid.handle[eh].chosenpresentqueue};
    VkImageCreateInfo depthCreateInfo = {0};
    depthCreateInfo.sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
    depthCreateInfo.arrayLayers = euclid.handle[eh].shadowMapsCount[0];
    depthCreateInfo.format = VK_FORMAT_D32_SFLOAT;
    depthCreateInfo.tiling = VK_IMAGE_TILING_OPTIMAL;
    depthCreateInfo.usage = VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT | VK_IMAGE_USAGE_SAMPLED_BIT;
    depthCreateInfo.mipLevels = 1;
    depthCreateInfo.extent.depth = 1;
    depthCreateInfo.imageType = VK_IMAGE_TYPE_2D;
    depthCreateInfo.extent.width = euclid.handle[eh].shadowMapResolution[0];
    depthCreateInfo.extent.height = euclid.handle[eh].shadowMapResolution[0];
    depthCreateInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
    depthCreateInfo.samples = VK_SAMPLE_COUNT_1_BIT;
    depthCreateInfo.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    VkResult result = vkCreateImage(euclid.handle[eh].device, &depthCreateInfo, NULL, &euclid.handle[eh].shadowImage);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Shadow image created with result = %d\n", result);

    VkMemoryRequirements memRequirements;
    vkGetImageMemoryRequirements(euclid.handle[eh].device, euclid.handle[eh].shadowImage, &memRequirements);

    VkMemoryAllocateInfo allocInfo = {0};
    allocInfo.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    allocInfo.allocationSize = memRequirements.size;
    allocInfo.memoryTypeIndex = findMemoryType(memRequirements.memoryTypeBits, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT, eh);

    result = vkAllocateMemory(euclid.handle[eh].device, &allocInfo, NULL, &euclid.handle[eh].shadowImageMemory);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Shadow image memory allocated with result = %d\n", result);

    vkBindImageMemory(euclid.handle[eh].device, euclid.handle[eh].shadowImage, euclid.handle[eh].shadowImageMemory, 0);

    VkImageViewCreateInfo dicreateInfo = {0};
    dicreateInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
    dicreateInfo.image = euclid.handle[eh].shadowImage;
    dicreateInfo.viewType = VK_IMAGE_VIEW_TYPE_2D_ARRAY;
    dicreateInfo.format = VK_FORMAT_D32_SFLOAT;
    dicreateInfo.components.r = VK_COMPONENT_SWIZZLE_IDENTITY;
    dicreateInfo.components.g = VK_COMPONENT_SWIZZLE_IDENTITY;
    dicreateInfo.components.b = VK_COMPONENT_SWIZZLE_IDENTITY;
    dicreateInfo.components.a = VK_COMPONENT_SWIZZLE_IDENTITY;
    dicreateInfo.subresourceRange.aspectMask = VK_IMAGE_ASPECT_DEPTH_BIT;
    dicreateInfo.subresourceRange.baseMipLevel = 0;
    dicreateInfo.subresourceRange.levelCount = 1;
    dicreateInfo.subresourceRange.baseArrayLayer = 0;
    dicreateInfo.subresourceRange.layerCount = euclid.handle[eh].shadowMapsCount[0];
    result = vkCreateImageView(euclid.handle[eh].device, &dicreateInfo, NULL, &euclid.handle[eh].shadowImageView);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Shadow imageview created with result = %d\n", result);

    for (int i = 0; i != euclid.handle[eh].shadowMapsCount[0]; i++) {
        dicreateInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
        dicreateInfo.image = euclid.handle[eh].shadowImage;
        dicreateInfo.viewType = VK_IMAGE_VIEW_TYPE_2D;
        dicreateInfo.format = VK_FORMAT_D32_SFLOAT;
        dicreateInfo.components.r = VK_COMPONENT_SWIZZLE_IDENTITY;
        dicreateInfo.components.g = VK_COMPONENT_SWIZZLE_IDENTITY;
        dicreateInfo.components.b = VK_COMPONENT_SWIZZLE_IDENTITY;
        dicreateInfo.components.a = VK_COMPONENT_SWIZZLE_IDENTITY;
        dicreateInfo.subresourceRange.aspectMask = VK_IMAGE_ASPECT_DEPTH_BIT;
        dicreateInfo.subresourceRange.baseMipLevel = 0;
        dicreateInfo.subresourceRange.levelCount = 1;
        dicreateInfo.subresourceRange.baseArrayLayer = i;
        dicreateInfo.subresourceRange.layerCount = 1;
        result = vkCreateImageView(euclid.handle[eh].device, &dicreateInfo, NULL, &euclid.handle[eh].shadowRenderImageViews[i]);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Shadow render imageview created with result = %d\n", result);

        VkImageView attachments[] = {
            euclid.handle[eh].shadowRenderImageViews[i],
        };
    
        VkFramebufferCreateInfo framebufferInfo = {0};
        framebufferInfo.sType = VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO;
        framebufferInfo.renderPass = euclid.handle[eh].shadowRenderPass;
        framebufferInfo.attachmentCount = 1;
        framebufferInfo.pAttachments = attachments;
        framebufferInfo.width = euclid.handle[eh].shadowMapResolution[0];
        framebufferInfo.height = euclid.handle[eh].shadowMapResolution[0];
        framebufferInfo.layers = 1;
    
        result = vkCreateFramebuffer(euclid.handle[eh].device, &framebufferInfo, NULL, &euclid.handle[eh].shadowFramebuffers[i]);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Shadow framebuffer created with result = %d\n", result);
    }
}

void createDefferedData(uint32_t eh){
    euclid.handle[eh].renderResolutionX = euclid.handle[eh].resolutionX[0] * euclid.handle[eh].resolutionScale[0];
    euclid.handle[eh].renderResolutionY = euclid.handle[eh].resolutionY[0] * euclid.handle[eh].resolutionScale[0];
    euclid.handle[eh].oldDefferedCount = euclid.handle[eh].defferedCount[0];
    euclid.handle[eh].oldResolutionScale = euclid.handle[eh].resolutionScale[0];
    if(euclid.handle[eh].renderResolutionX <= 0){
        euclid.handle[eh].renderResolutionX = 1;
    }
    if(euclid.handle[eh].renderResolutionY <= 0){
        euclid.handle[eh].renderResolutionY = 1;
    }
    VkResult result;
    {
        uint32_t queueFamilyIndices[] = {euclid.handle[eh].chosenqueuefam, euclid.handle[eh].chosenpresentqueue};
        VkImageCreateInfo depthCreateInfo = {0};
        depthCreateInfo.sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
        depthCreateInfo.arrayLayers = euclid.handle[eh].defferedCount[0];
        depthCreateInfo.format = VK_FORMAT_D32_SFLOAT;
        depthCreateInfo.tiling = VK_IMAGE_TILING_OPTIMAL;
        depthCreateInfo.usage = VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT | VK_IMAGE_USAGE_SAMPLED_BIT;
        depthCreateInfo.mipLevels = 1;
        depthCreateInfo.extent.depth = 1;
        depthCreateInfo.imageType = VK_IMAGE_TYPE_2D;
        depthCreateInfo.extent.width = euclid.handle[eh].renderResolutionX;
        depthCreateInfo.extent.height = euclid.handle[eh].renderResolutionY;
        depthCreateInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
        depthCreateInfo.samples = VK_SAMPLE_COUNT_1_BIT;
        depthCreateInfo.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
        VkResult result = vkCreateImage(euclid.handle[eh].device, &depthCreateInfo, NULL, &euclid.handle[eh].defferedDepthImage);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Deffered depth image created with result = %d\n", result);

        VkMemoryRequirements memRequirements;
        vkGetImageMemoryRequirements(euclid.handle[eh].device, euclid.handle[eh].defferedDepthImage, &memRequirements);

        VkMemoryAllocateInfo allocInfo = {0};
        allocInfo.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        allocInfo.allocationSize = memRequirements.size;
        allocInfo.memoryTypeIndex = findMemoryType(memRequirements.memoryTypeBits, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT, eh);

        result = vkAllocateMemory(euclid.handle[eh].device, &allocInfo, NULL, &euclid.handle[eh].defferedDepthImageMemory);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Deffered depth image memory allocated with result = %d\n", result);

        vkBindImageMemory(euclid.handle[eh].device, euclid.handle[eh].defferedDepthImage, euclid.handle[eh].defferedDepthImageMemory, 0);
    }

    {
        VkImageCreateInfo defferedCreateInfo = {0};
        defferedCreateInfo.sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
        defferedCreateInfo.arrayLayers = euclid.handle[eh].defferedCount[0]*3;
        defferedCreateInfo.format = VK_FORMAT_R16G16B16A16_SFLOAT;
        defferedCreateInfo.tiling = VK_IMAGE_TILING_OPTIMAL;
        defferedCreateInfo.usage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_SAMPLED_BIT;
        defferedCreateInfo.mipLevels = 1;
        defferedCreateInfo.extent.depth = 1;
        defferedCreateInfo.imageType = VK_IMAGE_TYPE_2D;
        defferedCreateInfo.extent.width = euclid.handle[eh].renderResolutionX;
        defferedCreateInfo.extent.height = euclid.handle[eh].renderResolutionY;
        defferedCreateInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
        defferedCreateInfo.samples = VK_SAMPLE_COUNT_1_BIT;
        defferedCreateInfo.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
        VkResult result = vkCreateImage(euclid.handle[eh].device, &defferedCreateInfo, NULL, &euclid.handle[eh].defferedImage);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Deffered image created with result = %d\n", result);

        VkMemoryRequirements memdefRequirements;
        vkGetImageMemoryRequirements(euclid.handle[eh].device, euclid.handle[eh].defferedImage, &memdefRequirements);

        VkMemoryAllocateInfo allodefcInfo = {0};
        allodefcInfo.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        allodefcInfo.allocationSize = memdefRequirements.size;
        allodefcInfo.memoryTypeIndex = findMemoryType(memdefRequirements.memoryTypeBits, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT, eh);

        result = vkAllocateMemory(euclid.handle[eh].device, &allodefcInfo, NULL, &euclid.handle[eh].defferedImageMemory);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Deffered image memory allocated with result = %d\n", result);

        vkBindImageMemory(euclid.handle[eh].device, euclid.handle[eh].defferedImage, euclid.handle[eh].defferedImageMemory, 0);
    }

    VkImageViewCreateInfo dicreateInfo = {0};
    dicreateInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
    dicreateInfo.image = euclid.handle[eh].defferedDepthImage;
    dicreateInfo.viewType = VK_IMAGE_VIEW_TYPE_2D_ARRAY;
    dicreateInfo.format = VK_FORMAT_D32_SFLOAT;
    dicreateInfo.components.r = VK_COMPONENT_SWIZZLE_IDENTITY;
    dicreateInfo.components.g = VK_COMPONENT_SWIZZLE_IDENTITY;
    dicreateInfo.components.b = VK_COMPONENT_SWIZZLE_IDENTITY;
    dicreateInfo.components.a = VK_COMPONENT_SWIZZLE_IDENTITY;
    dicreateInfo.subresourceRange.aspectMask = VK_IMAGE_ASPECT_DEPTH_BIT;
    dicreateInfo.subresourceRange.baseMipLevel = 0;
    dicreateInfo.subresourceRange.levelCount = 1;
    dicreateInfo.subresourceRange.baseArrayLayer = 0;
    dicreateInfo.subresourceRange.layerCount = euclid.handle[eh].defferedCount[0];
    result = vkCreateImageView(euclid.handle[eh].device, &dicreateInfo, NULL, &euclid.handle[eh].defferedDepthImageView);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Deffered depth imageview created with result = %d\n", result);

    VkImageViewCreateInfo defcreateInfo = {0};
    defcreateInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
    defcreateInfo.image = euclid.handle[eh].defferedImage;
    defcreateInfo.viewType = VK_IMAGE_VIEW_TYPE_2D_ARRAY;
    defcreateInfo.format = VK_FORMAT_R16G16B16A16_SFLOAT;
    defcreateInfo.components.r = VK_COMPONENT_SWIZZLE_IDENTITY;
    defcreateInfo.components.g = VK_COMPONENT_SWIZZLE_IDENTITY;
    defcreateInfo.components.b = VK_COMPONENT_SWIZZLE_IDENTITY;
    defcreateInfo.components.a = VK_COMPONENT_SWIZZLE_IDENTITY;
    defcreateInfo.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
    defcreateInfo.subresourceRange.baseMipLevel = 0;
    defcreateInfo.subresourceRange.levelCount = 1;
    defcreateInfo.subresourceRange.baseArrayLayer = 0;
    defcreateInfo.subresourceRange.layerCount = euclid.handle[eh].defferedCount[0]*3;
    result = vkCreateImageView(euclid.handle[eh].device, &defcreateInfo, NULL, &euclid.handle[eh].defferedImageView);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Deffered depth imageview created with result = %d\n", result);

    for (uint32_t i = 0, b = 0; i != euclid.handle[eh].defferedCount[0]; i++, b+=3) {
        dicreateInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
        dicreateInfo.image = euclid.handle[eh].defferedDepthImage;
        dicreateInfo.viewType = VK_IMAGE_VIEW_TYPE_2D;
        dicreateInfo.format = VK_FORMAT_D32_SFLOAT;
        dicreateInfo.components.r = VK_COMPONENT_SWIZZLE_IDENTITY;
        dicreateInfo.components.g = VK_COMPONENT_SWIZZLE_IDENTITY;
        dicreateInfo.components.b = VK_COMPONENT_SWIZZLE_IDENTITY;
        dicreateInfo.components.a = VK_COMPONENT_SWIZZLE_IDENTITY;
        dicreateInfo.subresourceRange.aspectMask = VK_IMAGE_ASPECT_DEPTH_BIT;
        dicreateInfo.subresourceRange.baseMipLevel = 0;
        dicreateInfo.subresourceRange.levelCount = 1;
        dicreateInfo.subresourceRange.baseArrayLayer = i;
        dicreateInfo.subresourceRange.layerCount = 1;
        result = vkCreateImageView(euclid.handle[eh].device, &dicreateInfo, NULL, &euclid.handle[eh].defferedDepthRenderImageViews[i]);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Deffered depth render imageview (%d) created with result = %d\n", i, result);

        for(uint32_t j = 0; j != 3; j++){
            defcreateInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
            defcreateInfo.image = euclid.handle[eh].defferedImage;
            defcreateInfo.viewType = VK_IMAGE_VIEW_TYPE_2D;
            defcreateInfo.format = VK_FORMAT_R16G16B16A16_SFLOAT;
            defcreateInfo.components.r = VK_COMPONENT_SWIZZLE_IDENTITY;
            defcreateInfo.components.g = VK_COMPONENT_SWIZZLE_IDENTITY;
            defcreateInfo.components.b = VK_COMPONENT_SWIZZLE_IDENTITY;
            defcreateInfo.components.a = VK_COMPONENT_SWIZZLE_IDENTITY;
            defcreateInfo.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
            defcreateInfo.subresourceRange.baseMipLevel = 0;
            defcreateInfo.subresourceRange.levelCount = 1;
            defcreateInfo.subresourceRange.baseArrayLayer = b+j;
            defcreateInfo.subresourceRange.layerCount = 1;
            result = vkCreateImageView(euclid.handle[eh].device, &defcreateInfo, NULL, &euclid.handle[eh].defferedRenderImageViews[b+j]);
            if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Deffered render imageview (%d) created with result = %d\n", b+j, result);
        }

        VkImageView attachments[] = {
            euclid.handle[eh].defferedRenderImageViews[b],
            euclid.handle[eh].defferedRenderImageViews[b+1],
            euclid.handle[eh].defferedRenderImageViews[b+2],
            euclid.handle[eh].defferedDepthRenderImageViews[i],
        };
    
        VkFramebufferCreateInfo framebufferInfo = {0};
        framebufferInfo.sType = VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO;
        framebufferInfo.renderPass = euclid.handle[eh].defferedRenderPass;
        framebufferInfo.attachmentCount = 4;
        framebufferInfo.pAttachments = attachments;
        framebufferInfo.width = euclid.handle[eh].renderResolutionX;
        framebufferInfo.height = euclid.handle[eh].renderResolutionY;
        framebufferInfo.layers = 1;
    
        result = vkCreateFramebuffer(euclid.handle[eh].device, &framebufferInfo, NULL, &euclid.handle[eh].defferedFramebuffers[i]);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Deffered framebuffer (%d) created with result = %d\n", i, result);
    }
}

void createLightingData(uint32_t eh){
    VkResult result;

    {
        VkImageCreateInfo depthCreateInfo = {0};
        depthCreateInfo.sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
        depthCreateInfo.arrayLayers = 2;
        depthCreateInfo.format = VK_FORMAT_D32_SFLOAT;
        depthCreateInfo.tiling = VK_IMAGE_TILING_OPTIMAL;
        depthCreateInfo.usage = VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT | VK_IMAGE_USAGE_SAMPLED_BIT;
        depthCreateInfo.mipLevels = 1;
        depthCreateInfo.extent.depth = 1;
        depthCreateInfo.imageType = VK_IMAGE_TYPE_2D;
        depthCreateInfo.extent.width = euclid.handle[eh].renderResolutionX;
        depthCreateInfo.extent.height = euclid.handle[eh].renderResolutionY;
        depthCreateInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
        depthCreateInfo.samples = VK_SAMPLE_COUNT_1_BIT;
        depthCreateInfo.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
        result = vkCreateImage(euclid.handle[eh].device, &depthCreateInfo, NULL, &euclid.handle[eh].lightingDepthImage);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Lighting depth image created with result = %d\n", result);

        VkMemoryRequirements memRequirements;
        vkGetImageMemoryRequirements(euclid.handle[eh].device, euclid.handle[eh].lightingDepthImage, &memRequirements);

        VkMemoryAllocateInfo allocInfo = {0};
        allocInfo.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        allocInfo.allocationSize = memRequirements.size;
        allocInfo.memoryTypeIndex = findMemoryType(memRequirements.memoryTypeBits, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT, eh);

        result = vkAllocateMemory(euclid.handle[eh].device, &allocInfo, NULL, &euclid.handle[eh].lightingDepthImageMemory);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Lighting depth image memory allocated with result = %d\n", result);

        vkBindImageMemory(euclid.handle[eh].device, euclid.handle[eh].lightingDepthImage, euclid.handle[eh].lightingDepthImageMemory, 0);
    }

    {
        VkImageCreateInfo lightingCreateInfo = {0};
        lightingCreateInfo.sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
        lightingCreateInfo.arrayLayers = 2;
        lightingCreateInfo.format = VK_FORMAT_R16G16B16A16_SFLOAT;
        lightingCreateInfo.tiling = VK_IMAGE_TILING_OPTIMAL;
        lightingCreateInfo.usage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_SAMPLED_BIT;
        lightingCreateInfo.mipLevels = 1;
        lightingCreateInfo.extent.depth = 1;
        lightingCreateInfo.imageType = VK_IMAGE_TYPE_2D;
        lightingCreateInfo.extent.width = euclid.handle[eh].renderResolutionX;
        lightingCreateInfo.extent.height = euclid.handle[eh].renderResolutionY;
        lightingCreateInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
        lightingCreateInfo.samples = VK_SAMPLE_COUNT_1_BIT;
        lightingCreateInfo.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
        result = vkCreateImage(euclid.handle[eh].device, &lightingCreateInfo, NULL, &euclid.handle[eh].lightingImage);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Lighting image created with result = %d\n", result);

        VkMemoryRequirements memRequirements;
        vkGetImageMemoryRequirements(euclid.handle[eh].device, euclid.handle[eh].lightingImage, &memRequirements);

        VkMemoryAllocateInfo allocInfo = {0};
        allocInfo.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        allocInfo.allocationSize = memRequirements.size;
        allocInfo.memoryTypeIndex = findMemoryType(memRequirements.memoryTypeBits, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT, eh);

        result = vkAllocateMemory(euclid.handle[eh].device, &allocInfo, NULL, &euclid.handle[eh].lightingImageMemory);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Lighting image memory allocated with result = %d\n", result);

        vkBindImageMemory(euclid.handle[eh].device, euclid.handle[eh].lightingImage, euclid.handle[eh].lightingImageMemory, 0);
    }

    VkImageViewCreateInfo depthCreateInfo = {0};
    depthCreateInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
    depthCreateInfo.image = euclid.handle[eh].lightingDepthImage;
    depthCreateInfo.viewType = VK_IMAGE_VIEW_TYPE_2D;
    depthCreateInfo.format = VK_FORMAT_D32_SFLOAT;
    depthCreateInfo.components.r = VK_COMPONENT_SWIZZLE_IDENTITY;
    depthCreateInfo.components.g = VK_COMPONENT_SWIZZLE_IDENTITY;
    depthCreateInfo.components.b = VK_COMPONENT_SWIZZLE_IDENTITY;
    depthCreateInfo.components.a = VK_COMPONENT_SWIZZLE_IDENTITY;
    depthCreateInfo.subresourceRange.aspectMask = VK_IMAGE_ASPECT_DEPTH_BIT;
    depthCreateInfo.subresourceRange.baseMipLevel = 0;
    depthCreateInfo.subresourceRange.levelCount = 1;
    depthCreateInfo.subresourceRange.baseArrayLayer = 0;
    depthCreateInfo.subresourceRange.layerCount = 2;
    result = vkCreateImageView(euclid.handle[eh].device, &depthCreateInfo, NULL, &euclid.handle[eh].lightingDepthImageViewl);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Lighting depth imageview created with result = %d\n", result);

    VkImageViewCreateInfo lightingCreateInfo = {0};
    lightingCreateInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
    lightingCreateInfo.image = euclid.handle[eh].lightingImage;
    lightingCreateInfo.viewType = VK_IMAGE_VIEW_TYPE_2D;
    lightingCreateInfo.format = VK_FORMAT_R16G16B16A16_SFLOAT;
    lightingCreateInfo.components.r = VK_COMPONENT_SWIZZLE_IDENTITY;
    lightingCreateInfo.components.g = VK_COMPONENT_SWIZZLE_IDENTITY;
    lightingCreateInfo.components.b = VK_COMPONENT_SWIZZLE_IDENTITY;
    lightingCreateInfo.components.a = VK_COMPONENT_SWIZZLE_IDENTITY;
    lightingCreateInfo.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
    lightingCreateInfo.subresourceRange.baseMipLevel = 0;
    lightingCreateInfo.subresourceRange.levelCount = 1;
    lightingCreateInfo.subresourceRange.baseArrayLayer = 0;
    lightingCreateInfo.subresourceRange.layerCount = 2;
    result = vkCreateImageView(euclid.handle[eh].device, &lightingCreateInfo, NULL, &euclid.handle[eh].lightingImageViewl);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Lighting imageview created with result = %d\n", result);

    for(uint32_t i = 0; i < 2; i++){
        VkImageViewCreateInfo depthCreateInfo = {0};
        depthCreateInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
        depthCreateInfo.image = euclid.handle[eh].lightingDepthImage;
        depthCreateInfo.viewType = VK_IMAGE_VIEW_TYPE_2D;
        depthCreateInfo.format = VK_FORMAT_D32_SFLOAT;
        depthCreateInfo.components.r = VK_COMPONENT_SWIZZLE_IDENTITY;
        depthCreateInfo.components.g = VK_COMPONENT_SWIZZLE_IDENTITY;
        depthCreateInfo.components.b = VK_COMPONENT_SWIZZLE_IDENTITY;
        depthCreateInfo.components.a = VK_COMPONENT_SWIZZLE_IDENTITY;
        depthCreateInfo.subresourceRange.aspectMask = VK_IMAGE_ASPECT_DEPTH_BIT;
        depthCreateInfo.subresourceRange.baseMipLevel = 0;
        depthCreateInfo.subresourceRange.levelCount = 1;
        depthCreateInfo.subresourceRange.baseArrayLayer = i;
        depthCreateInfo.subresourceRange.layerCount = 1;
        result = vkCreateImageView(euclid.handle[eh].device, &depthCreateInfo, NULL, &euclid.handle[eh].lightingDepthImageViews[i]);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Lighting depth imageview created with result = %d\n", result);

        VkImageViewCreateInfo lightingCreateInfo = {0};
        lightingCreateInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
        lightingCreateInfo.image = euclid.handle[eh].lightingImage;
        lightingCreateInfo.viewType = VK_IMAGE_VIEW_TYPE_2D;
        lightingCreateInfo.format = VK_FORMAT_R16G16B16A16_SFLOAT;
        lightingCreateInfo.components.r = VK_COMPONENT_SWIZZLE_IDENTITY;
        lightingCreateInfo.components.g = VK_COMPONENT_SWIZZLE_IDENTITY;
        lightingCreateInfo.components.b = VK_COMPONENT_SWIZZLE_IDENTITY;
        lightingCreateInfo.components.a = VK_COMPONENT_SWIZZLE_IDENTITY;
        lightingCreateInfo.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
        lightingCreateInfo.subresourceRange.baseMipLevel = 0;
        lightingCreateInfo.subresourceRange.levelCount = 1;
        lightingCreateInfo.subresourceRange.baseArrayLayer = i;
        lightingCreateInfo.subresourceRange.layerCount = 1;
        result = vkCreateImageView(euclid.handle[eh].device, &lightingCreateInfo, NULL, &euclid.handle[eh].lightingImageViews[i]);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Lighting imageview created with result = %d\n", result);

        VkImageView attachments[] = {
            euclid.handle[eh].lightingImageViews[i],
            euclid.handle[eh].lightingDepthImageViews[i],
        };

        VkFramebufferCreateInfo framebufferInfo = {0};
        framebufferInfo.sType = VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO;
        framebufferInfo.renderPass = euclid.handle[eh].lightingRenderPass;
        framebufferInfo.attachmentCount = 2;
        framebufferInfo.pAttachments = attachments;
        framebufferInfo.width = euclid.handle[eh].renderResolutionX;
        framebufferInfo.height = euclid.handle[eh].renderResolutionY;
        framebufferInfo.layers = 1;

        result = vkCreateFramebuffer(euclid.handle[eh].device, &framebufferInfo, NULL, &euclid.handle[eh].lightingFramebuffers[i]);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Lighting framebuffer created with result = %d\n", result);
    }
}

void createFrameBuffers(uint32_t eh){
    uint32_t queueFamilyIndices[] = {euclid.handle[eh].chosenqueuefam, euclid.handle[eh].chosenpresentqueue};
    VkImageCreateInfo depthCreateInfo = {0};
    depthCreateInfo.sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
    depthCreateInfo.arrayLayers = 1;
    depthCreateInfo.format = VK_FORMAT_D32_SFLOAT;
    depthCreateInfo.tiling = VK_IMAGE_TILING_OPTIMAL;
    depthCreateInfo.usage = VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT;
    depthCreateInfo.mipLevels = 1;
    depthCreateInfo.extent.depth = 1;
    depthCreateInfo.imageType = VK_IMAGE_TYPE_2D;
    depthCreateInfo.extent.width = euclid.handle[eh].resolutionX[0];
    depthCreateInfo.extent.height = euclid.handle[eh].resolutionY[0];
    depthCreateInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
    depthCreateInfo.samples = VK_SAMPLE_COUNT_1_BIT;
    depthCreateInfo.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    VkResult result = vkCreateImage(euclid.handle[eh].device, &depthCreateInfo, NULL, &euclid.handle[eh].depthImage);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: depth image created with result = %d\n", result);

    VkMemoryRequirements memRequirements;
    vkGetImageMemoryRequirements(euclid.handle[eh].device, euclid.handle[eh].depthImage, &memRequirements);

    VkMemoryAllocateInfo allocInfo = {0};
    allocInfo.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    allocInfo.allocationSize = memRequirements.size;
    allocInfo.memoryTypeIndex = findMemoryType(memRequirements.memoryTypeBits, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT, eh);

    result = vkAllocateMemory(euclid.handle[eh].device, &allocInfo, NULL, &euclid.handle[eh].depthImageMemory);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: depth image memory allocated with result = %d\n", result);

    vkBindImageMemory(euclid.handle[eh].device, euclid.handle[eh].depthImage, euclid.handle[eh].depthImageMemory, 0);

    VkImageViewCreateInfo dicreateInfo = {0};
    dicreateInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
    dicreateInfo.image = euclid.handle[eh].depthImage;
    dicreateInfo.viewType = VK_IMAGE_VIEW_TYPE_2D;
    dicreateInfo.format = VK_FORMAT_D32_SFLOAT;
    dicreateInfo.components.r = VK_COMPONENT_SWIZZLE_IDENTITY;
    dicreateInfo.components.g = VK_COMPONENT_SWIZZLE_IDENTITY;
    dicreateInfo.components.b = VK_COMPONENT_SWIZZLE_IDENTITY;
    dicreateInfo.components.a = VK_COMPONENT_SWIZZLE_IDENTITY;
    dicreateInfo.subresourceRange.aspectMask = VK_IMAGE_ASPECT_DEPTH_BIT;
    dicreateInfo.subresourceRange.baseMipLevel = 0;
    dicreateInfo.subresourceRange.levelCount = 1;
    dicreateInfo.subresourceRange.baseArrayLayer = 0;
    dicreateInfo.subresourceRange.layerCount = 1;
    result = vkCreateImageView(euclid.handle[eh].device, &dicreateInfo, NULL, &euclid.handle[eh].depthImageView);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: depth imageview created with result = %d\n", result);

    euclid.handle[eh].swapChainFramebuffers = malloc(sizeof(VkFramebuffer)*euclid.handle[eh].swapChainImageCount);

    for (int i = 0; i != euclid.handle[eh].swapChainImageCount; i++) {
        VkImageView attachments[] = {
            euclid.handle[eh].swapChainImageViews[i],
            euclid.handle[eh].depthImageView,
        };
    
        VkFramebufferCreateInfo framebufferInfo = {0};
        framebufferInfo.sType = VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO;
        framebufferInfo.renderPass = euclid.handle[eh].renderPass;
        framebufferInfo.attachmentCount = 2;
        framebufferInfo.pAttachments = attachments;
        framebufferInfo.width = euclid.handle[eh].resolutionX[0];
        framebufferInfo.height = euclid.handle[eh].resolutionY[0];
        framebufferInfo.layers = 1;
    
        result = vkCreateFramebuffer(euclid.handle[eh].device, &framebufferInfo, NULL, &euclid.handle[eh].swapChainFramebuffers[i]);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: SwapChain framebuffers created with result = %d\n", result);
    }
}

void createCommandPool(uint32_t eh){
    for(int i = 0; i < 5; i++){
        VkCommandPoolCreateInfo poolInfo = {0};
        poolInfo.sType = VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;
        poolInfo.flags = VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT;
        poolInfo.queueFamilyIndex = euclid.handle[eh].chosenqueuefam;

        VkResult result = vkCreateCommandPool(euclid.handle[eh].device, &poolInfo, NULL, &euclid.handle[eh].commandPool[i]);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Command pool created with result = %d\n", result);
    }
}

void createCommandBuffer(uint32_t eh){
    euclid.handle[eh].commandBuffers = malloc(sizeof(VkCommandBuffer)*MAX_FRAMES_IN_FLIGHT*4);
    for(int i = 0; i < MAX_FRAMES_IN_FLIGHT; i++){
        for(int j = 0; j < 4; j++){
            VkCommandBufferAllocateInfo allocInfo = {0};
            allocInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
            allocInfo.commandPool = euclid.handle[eh].commandPool[j];
            allocInfo.level = VK_COMMAND_BUFFER_LEVEL_PRIMARY;
            allocInfo.commandBufferCount = 1;

            VkResult result = vkAllocateCommandBuffers( euclid.handle[eh].device, &allocInfo, &euclid.handle[eh].commandBuffers[j+i*4]);
            if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Command buffer created with result = %d\n", result);
        }
    }
}

void createSyncObjects(uint32_t eh){
    euclid.handle[eh].imageAvailableSemaphores = malloc(sizeof(VkSemaphore)*MAX_FRAMES_IN_FLIGHT);
    euclid.handle[eh].renderFinishedSemaphores = malloc(sizeof(VkSemaphore)*MAX_FRAMES_IN_FLIGHT);
    euclid.handle[eh].inFlightFences = malloc(sizeof(VkFence)*MAX_FRAMES_IN_FLIGHT);

    for(int i = 0; i != MAX_FRAMES_IN_FLIGHT; i++){
        VkSemaphoreCreateInfo semaphoreInfo = {0};
        semaphoreInfo.sType = VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO;
        VkFenceCreateInfo fenceInfo = {0};
        fenceInfo.sType = VK_STRUCTURE_TYPE_FENCE_CREATE_INFO;
        fenceInfo.flags = VK_FENCE_CREATE_SIGNALED_BIT;
        VkResult result = vkCreateSemaphore(euclid.handle[eh].device, &semaphoreInfo, NULL, &euclid.handle[eh].imageAvailableSemaphores[i]);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: imageAvailableSemaphore created with result = %d\n", result);
        result = vkCreateSemaphore(euclid.handle[eh].device, &semaphoreInfo, NULL, &euclid.handle[eh].renderFinishedSemaphores[i]);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: renderFinishedSemaphore created with result = %d\n", result);
        result = vkCreateFence(euclid.handle[eh].device, &fenceInfo, NULL, &euclid.handle[eh].inFlightFences[i]);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: inFlightFence created with result = %d\n", result);
    }
}

void recreateSwapChain(uint32_t eh){
    vkDestroySwapchainKHR(euclid.handle[eh].device, euclid.handle[eh].swapChain, NULL);
    for(int i = 0; i != euclid.handle[eh].swapChainImageCount; i++){
        vkDestroyFramebuffer(euclid.handle[eh].device, euclid.handle[eh].swapChainFramebuffers[i], NULL);
    }
    for(int i = 0; i != euclid.handle[eh].swapChainImageCount; i++){
        vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].swapChainImageViews[i], NULL);
    }
    //for(int i = 0; i != euclid.handle[eh].swapChainImageCount; i++){
    //    vkDestroyImage(euclid.handle[eh].device, euclid.handle[eh].swapChainImages[i], NULL);
    //}
    if(euclid.handle[eh].swapChainFramebuffers){
        free(euclid.handle[eh].swapChainFramebuffers);
        euclid.handle[eh].swapChainFramebuffers = NULL;
    }
    if(euclid.handle[eh].swapChainImageViews){
        free(euclid.handle[eh].swapChainImageViews);
        euclid.handle[eh].swapChainImageViews = NULL;
    }
    if(euclid.handle[eh].swapChainImages){
        free(euclid.handle[eh].swapChainImages);
        euclid.handle[eh].swapChainImages = NULL;
    }
    vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].depthImageView, NULL);
    vkDestroyImage(euclid.handle[eh].device, euclid.handle[eh].depthImage, NULL);
    vkFreeMemory(euclid.handle[eh].device, euclid.handle[eh].depthImageMemory, NULL);
    createSwapChain(eh);
    createRenderPass(eh);
    createSwapChainImageViews(eh);
    createFrameBuffers(eh);
}

void startrender(uint32_t eh){
    vkWaitForFences(euclid.handle[eh].device, 1, &euclid.handle[eh].inFlightFences[euclid.handle[eh].currentFrame], VK_TRUE, UINT64_MAX);
    vkResetFences(euclid.handle[eh].device, 1, &euclid.handle[eh].inFlightFences[euclid.handle[eh].currentFrame]);

    VkResult result = vkAcquireNextImageKHR(euclid.handle[eh].device, euclid.handle[eh].swapChain, UINT64_MAX, euclid.handle[eh].imageAvailableSemaphores[euclid.handle[eh].currentFrame], VK_NULL_HANDLE, &euclid.handle[eh].imageIndex);

    euclid.handle[eh].resolutionX[0] = euclid.handle[eh].resolutionX[1];
    euclid.handle[eh].resolutionY[0] = euclid.handle[eh].resolutionY[1];
    if(euclid.handle[eh].resolutionX[0] <= 0){
        if (euclid.handle[eh].debug == 1) printf("\e[1;35mEuclidVK\e[0;37m: ResolutionX lower than 1, %i\n", euclid.handle[eh].resolutionX[0]);
        euclid.handle[eh].resolutionX[0] = 1;
        euclid.handle[eh].resolutionX[1] = 1;
    }
    if(euclid.handle[eh].resolutionY[0] <= 0){
        if (euclid.handle[eh].debug == 1) printf("\e[1;35mEuclidVK\e[0;37m: ResolutionY lower than 1, %i\n", euclid.handle[eh].resolutionY[0]);
        euclid.handle[eh].resolutionY[0] = 1;
        euclid.handle[eh].resolutionY[1] = 1;
    }
    euclid.handle[eh].defferedCount[0] = euclid.handle[eh].defferedCount[1];
    euclid.handle[eh].resolutionScale[0] = euclid.handle[eh].resolutionScale[1];
    euclid.handle[eh].shadowMapResolution[0] = euclid.handle[eh].shadowMapResolution[1];
    euclid.handle[eh].shadowMapsCount[0] = euclid.handle[eh].shadowMapsCount[1];
    if (result == VK_ERROR_OUT_OF_DATE_KHR || euclid.handle[eh].oldx != euclid.handle[eh].resolutionX[0] || euclid.handle[eh].oldy != euclid.handle[eh].resolutionY[0] || euclid.handle[eh].resolutionScale[0] != euclid.handle[eh].oldResolutionScale || euclid.handle[eh].defferedCount[0] != euclid.handle[eh].oldDefferedCount) {
        if (euclid.handle[eh].debug == 1) printf("\e[1;35mEuclidVK\e[0;37m: Resolution changed from %dx%d to %dx%d, acquiere result = %i\n", euclid.handle[eh].oldx, euclid.handle[eh].oldy, euclid.handle[eh].resolutionX[0], euclid.handle[eh].resolutionY[0], result);
        euclid.handle[eh].oldx = euclid.handle[eh].resolutionX[0];
        euclid.handle[eh].oldy = euclid.handle[eh].resolutionY[0];
        vkDeviceWaitIdle(euclid.handle[eh].device);
        vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].defferedImageView, NULL);
        vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].defferedDepthImageView, NULL);
        for(uint32_t i = 0, b = 0; i != euclid.handle[eh].defferedCount[0]; i++, b+=3){
            vkDestroyFramebuffer(euclid.handle[eh].device, euclid.handle[eh].defferedFramebuffers[i], NULL);
            vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].defferedRenderImageViews[b], NULL);
            vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].defferedRenderImageViews[b+1], NULL);
            vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].defferedRenderImageViews[b+2], NULL);
            vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].defferedDepthRenderImageViews[i], NULL);
        }
        vkDestroyImage(euclid.handle[eh].device, euclid.handle[eh].defferedImage, NULL);
        vkFreeMemory(euclid.handle[eh].device, euclid.handle[eh].defferedImageMemory, NULL);
        vkDestroyImage(euclid.handle[eh].device, euclid.handle[eh].defferedDepthImage, NULL);
        vkFreeMemory(euclid.handle[eh].device, euclid.handle[eh].defferedDepthImageMemory, NULL);

        vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].lightingImageViews[0], NULL);
        vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].lightingDepthImageViews[0], NULL);
        vkDestroyFramebuffer(euclid.handle[eh].device, euclid.handle[eh].lightingFramebuffers[0], NULL);
        vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].lightingImageViews[1], NULL);
        vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].lightingDepthImageViews[1], NULL);
        vkDestroyFramebuffer(euclid.handle[eh].device, euclid.handle[eh].lightingFramebuffers[1], NULL);
        vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].lightingImageViewl, NULL);
        vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].lightingDepthImageViewl, NULL);
        vkDestroyImage(euclid.handle[eh].device, euclid.handle[eh].lightingImage, NULL);
        vkFreeMemory(euclid.handle[eh].device, euclid.handle[eh].lightingImageMemory, NULL);
        vkDestroyImage(euclid.handle[eh].device, euclid.handle[eh].lightingDepthImage, NULL);
        vkFreeMemory(euclid.handle[eh].device, euclid.handle[eh].lightingDepthImageMemory, NULL);

        if (euclid.handle[eh].debug == 1) printf("\e[1;35mEuclidVk\e[0;37m: Cleanup complete\n");

        vkDestroyRenderPass(euclid.handle[eh].device, euclid.handle[eh].renderPass, NULL);
        //vkDestroyRenderPass(euclid.handle[eh].device, euclid.handle[eh].lightingRenderPass, NULL);

        createDefferedData(eh);
        createLightingData(eh);
        recreateSwapChain(eh);
        euclid.handle[eh].mrec++;
        if (euclid.handle[eh].debug == 1) printf("\e[1;35mEuclidVK\e[0;37m: Recreated swapchain and deffered data\n");
    }
    if(euclid.handle[eh].shadowMapResolution[0] != euclid.handle[eh].oldshadowMapResolution || euclid.handle[eh].shadowMapsCount[0] != euclid.handle[eh].oldshadowMapsCount){
        if (euclid.handle[eh].debug == 1) printf("\e[1;35mEuclidVK\e[0;37m: Shadow map resolution changed from %i to %i, shadow count changed from %i to %i\n", euclid.handle[eh].oldshadowMapsCount, euclid.handle[eh].shadowMapResolution[0], euclid.handle[eh].shadowMapsCount[0], euclid.handle[eh].oldshadowMapsCount);
        vkDeviceWaitIdle(euclid.handle[eh].device);
        vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].shadowImageView, NULL);
        for(uint32_t i = 0; i != euclid.handle[eh].oldshadowMapsCount; i++){
            vkDestroyFramebuffer(euclid.handle[eh].device, euclid.handle[eh].shadowFramebuffers[i], NULL);
            vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].shadowRenderImageViews[i], NULL);
        }
        vkFreeMemory(euclid.handle[eh].device, euclid.handle[eh].shadowImageMemory, NULL);
        vkDestroyImage(euclid.handle[eh].device, euclid.handle[eh].shadowImage, NULL);
        euclid.handle[eh].oldshadowMapResolution = euclid.handle[eh].shadowMapResolution[0];
        euclid.handle[eh].oldshadowMapsCount = euclid.handle[eh].shadowMapsCount[0];
        createShadowData(eh);
        euclid.handle[eh].mrec++;
        if (euclid.handle[eh].debug == 1) printf("\e[1;35mEuclidVK\e[0;37m: Recreated shadow data\n");
    }

    vkResetCommandBuffer(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+0], 0);
    vkResetCommandBuffer(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+1], 0);
    vkResetCommandBuffer(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+2], 0);
    vkResetCommandBuffer(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+3], 0);

    uint8_t susedslot = (euclid.handle[eh].shadowslot+1)%2;
    uint8_t dusedslot = (euclid.handle[eh].defferedslot+1)%2;
    memcpy(euclid.handle[eh].shadowUniformBuffersMapped[0], euclid.handle[eh].shadowMatrices[susedslot], sizeof(float)*2400);
    memcpy(euclid.handle[eh].defferedUniformBuffersMapped[0], euclid.handle[eh].defferedMatrices[dusedslot], sizeof(float)*400);

    VkCommandBufferBeginInfo beginInfo = {0};
    beginInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
    beginInfo.flags = 0;
    beginInfo.pInheritanceInfo = NULL;
    vkBeginCommandBuffer(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+0], &beginInfo);
    vkBeginCommandBuffer(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+1], &beginInfo);
    vkBeginCommandBuffer(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+2], &beginInfo);
    vkBeginCommandBuffer(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+3], &beginInfo);
}

void modifyshadowdata(uint32_t eh, uint32_t ncnt, uint32_t nres, uint32_t lcnt){
    euclid.handle[eh].shadowMapResolution[1] = nres;
    euclid.handle[eh].shadowMapsCount[1] = ncnt;
    euclid.handle[eh].lightsCount = lcnt;
    if(ncnt == 0){
        euclid.handle[eh].enableShadowMaps = 0;
        euclid.handle[eh].shadowMapsCount[1] = 1;
        euclid.handle[eh].shadowMapResolution[1] = 1;
    } else {
        euclid.handle[eh].enableShadowMaps = 1;
    }
}

void modifydeffereddata(uint32_t eh, uint32_t ncnt, float nres){
    euclid.handle[eh].resolutionScale[1] = nres;
    euclid.handle[eh].defferedCount[1] = ncnt;
}

void modifyshadowuniform(uint32_t eh, uint32_t pos, float value){
    euclid.handle[eh].shadowMatrices[euclid.handle[eh].shadowslot][pos] = value;
}

void modifydeffereduniform(uint32_t eh, uint32_t pos, float value){
    euclid.handle[eh].defferedMatrices[euclid.handle[eh].defferedslot][pos] = value;
}

void startmainrenderpass(uint32_t eh){
    VkRenderPassBeginInfo renderPassInfo = {0};
    renderPassInfo.sType = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO;
    renderPassInfo.renderPass = euclid.handle[eh].renderPass;
    renderPassInfo.framebuffer = euclid.handle[eh].swapChainFramebuffers[euclid.handle[eh].imageIndex];
    renderPassInfo.renderArea.offset.x = 0;
    renderPassInfo.renderArea.offset.y = 0;
    renderPassInfo.renderArea.extent.width = euclid.handle[eh].resolutionX[0];
    renderPassInfo.renderArea.extent.height = euclid.handle[eh].resolutionY[0];
    VkClearValue clearValues[2] = {0};
    clearValues[0].color.float32[0] = 0.0;
    clearValues[0].color.float32[1] = 0.0;
    clearValues[0].color.float32[2] = 0.0;
    clearValues[0].color.float32[3] = 1.0;
    clearValues[1].depthStencil.depth = 1.0;
    clearValues[1].depthStencil.stencil = 0.0;    
    renderPassInfo.clearValueCount = 2;
    renderPassInfo.pClearValues = clearValues;
    vkCmdBeginRenderPass(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+3], &renderPassInfo, VK_SUBPASS_CONTENTS_INLINE);
}

void startdefferedrenderpass(uint32_t eh, uint32_t cc){
    VkRenderPassBeginInfo renderPassInfo = {0};
    renderPassInfo.sType = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO;
    renderPassInfo.renderPass = euclid.handle[eh].defferedRenderPass;
    renderPassInfo.framebuffer = euclid.handle[eh].defferedFramebuffers[cc];
    renderPassInfo.renderArea.offset.x = 0;
    renderPassInfo.renderArea.offset.y = 0;
    renderPassInfo.renderArea.extent.width = euclid.handle[eh].renderResolutionX;
    renderPassInfo.renderArea.extent.height = euclid.handle[eh].renderResolutionY;
    VkClearValue clearValues[4] = {0};
    for(uint32_t i = 0; i != 3; i++){
        clearValues[i].color.float32[0] = 0.0;
        clearValues[i].color.float32[1] = 0.0;
        clearValues[i].color.float32[2] = 0.0;
        clearValues[i].color.float32[3] = 1.0;
    }
    clearValues[3].depthStencil.depth = 1.0;
    clearValues[3].depthStencil.stencil = 0.0;    
    renderPassInfo.clearValueCount = 4;
    renderPassInfo.pClearValues = clearValues;
    vkCmdBeginRenderPass(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+1], &renderPassInfo, VK_SUBPASS_CONTENTS_INLINE);
}

void startlightingrenderpass(uint32_t eh){
    euclid.handle[eh].lightattn = (euclid.handle[eh].lightattn+1)%2;
    VkRenderPassBeginInfo renderPassInfo = {0};
    renderPassInfo.sType = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO;
    renderPassInfo.renderPass = euclid.handle[eh].lightingRenderPass;
    renderPassInfo.framebuffer = euclid.handle[eh].lightingFramebuffers[euclid.handle[eh].lightattn];
    renderPassInfo.renderArea.offset.x = 0;
    renderPassInfo.renderArea.offset.y = 0;
    renderPassInfo.renderArea.extent.width = euclid.handle[eh].renderResolutionX;
    renderPassInfo.renderArea.extent.height = euclid.handle[eh].renderResolutionY;
    VkClearValue clearValues[2] = {0};
    clearValues[0].color.float32[0] = 0.0;
    clearValues[0].color.float32[1] = 0.0;
    clearValues[0].color.float32[2] = 0.0;
    clearValues[0].color.float32[3] = 1.0;
    clearValues[1].depthStencil.depth = 1.0;
    clearValues[1].depthStencil.stencil = 0.0;
    renderPassInfo.clearValueCount = 2;
    renderPassInfo.pClearValues = clearValues;
    vkCmdBeginRenderPass(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+2], &renderPassInfo, VK_SUBPASS_CONTENTS_INLINE);
}

void startshadowrenderpass(uint32_t eh, uint32_t nm){
    VkRenderPassBeginInfo renderPassInfo = {0};
    renderPassInfo.sType = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO;
    renderPassInfo.renderPass = euclid.handle[eh].shadowRenderPass;
    renderPassInfo.framebuffer = euclid.handle[eh].shadowFramebuffers[nm];
    renderPassInfo.renderArea.offset.x = 0;
    renderPassInfo.renderArea.offset.y = 0;
    renderPassInfo.renderArea.extent.width = euclid.handle[eh].shadowMapResolution[0];
    renderPassInfo.renderArea.extent.height = euclid.handle[eh].shadowMapResolution[0];
    VkClearValue clearValue;
    clearValue.depthStencil.depth = 1.0;
    clearValue.depthStencil.stencil = 0.0;    
    renderPassInfo.clearValueCount = 1;
    renderPassInfo.pClearValues = &clearValue;
    vkCmdBeginRenderPass(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4], &renderPassInfo, VK_SUBPASS_CONTENTS_INLINE);
}

void endrenderpass(uint32_t eh, int j){
    vkCmdEndRenderPass(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+j]);
}

void endrender(uint32_t eh){
    vkEndCommandBuffer(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+0]);
    vkEndCommandBuffer(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+1]);
    vkEndCommandBuffer(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+2]);
    vkEndCommandBuffer(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+3]);

    VkSubmitInfo submitInfo = {0};
    submitInfo.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO;

    VkPipelineStageFlags waitStages[] = {VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT};
    submitInfo.waitSemaphoreCount = 1;
    submitInfo.pWaitSemaphores = &euclid.handle[eh].imageAvailableSemaphores[euclid.handle[eh].currentFrame];
    submitInfo.pWaitDstStageMask = waitStages;

    VkCommandBuffer commandBuffers[] = {
        euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4],
        euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+1],
        euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+2],
        euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+3]
    };

    submitInfo.commandBufferCount = 4;
    submitInfo.pCommandBuffers = commandBuffers;

    submitInfo.signalSemaphoreCount = 1;
    submitInfo.pSignalSemaphores = &euclid.handle[eh].renderFinishedSemaphores[euclid.handle[eh].currentFrame];
    vkQueueSubmit(euclid.handle[eh].graphicsQueue, 1, &submitInfo, euclid.handle[eh].inFlightFences[euclid.handle[eh].currentFrame]);

    VkPresentInfoKHR presentInfo = {0};
    presentInfo.sType = VK_STRUCTURE_TYPE_PRESENT_INFO_KHR;

    presentInfo.waitSemaphoreCount = 1;
    presentInfo.pWaitSemaphores = &euclid.handle[eh].renderFinishedSemaphores[euclid.handle[eh].currentFrame];
    presentInfo.swapchainCount = 1;
    presentInfo.pSwapchains = &euclid.handle[eh].swapChain;
    presentInfo.pImageIndices = &euclid.handle[eh].imageIndex;
    presentInfo.pResults = NULL;

    vkQueuePresentKHR(euclid.handle[eh].presentQueue, &presentInfo);

    euclid.handle[eh].currentFrame = (euclid.handle[eh].currentFrame + 1) % MAX_FRAMES_IN_FLIGHT;
    euclid.handle[eh].totalFrames++;
}

static void free_creation_queue_item(euclidCreationQueue *item){
    if(item == NULL){
        return;
    }
    free(item->vert);
    free(item->frag);
    free(item->shadow);
    free(item->vertices);
    free(item->uv);
    free(item->normals);
    free(item->pixels);
    free(item->te);
    //memset(item, 0, sizeof(*item));
}

static uint32_t newmaterial_real(uint32_t eh, uint32_t em, euclidCreationQueue *item);
static uint32_t newmodel_real(uint32_t eh, uint32_t em, euclidCreationQueue *item);
static uint32_t newmesh_real(uint32_t eh, uint32_t eme, euclidCreationQueue *item);
static uint32_t newtexture_real(uint32_t eh, uint32_t te, euclidCreationQueue *item);

static void push_creation_queue(uint32_t eh, const euclidCreationQueue *item){
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidOQ\e[0;37m: queue reallocation from %i to %i\n", euclid.handle[eh].queuesize, euclid.handle[eh].queuesize+1);
    euclid.handle[eh].eq = realloc(euclid.handle[eh].eq, sizeof(euclidCreationQueue) * (euclid.handle[eh].queuesize + 1));
    if(euclid.handle[eh].eq == NULL){
        if(euclid.handle[eh].debug == 1) printf("\e[1;31mError\e[0;37m: Failed to allocate creation queue entry\n");
        exit(-1);
    }
    euclid.handle[eh].eq[euclid.handle[eh].queuesize] = item[0];
    euclid.handle[eh].queuesize++;
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidOQ\e[0;37m: queue reallocation complete\n");
}

static void process_creation_queue(uint32_t eh){
    if(euclid.msize != euclid.vmsize){
        euclid.msize = euclid.vmsize;
        euclid.materials = realloc(euclid.materials, sizeof(euclidmaterial) * euclid.msize);
    }
    if(euclid.mosize != euclid.vmosize){
        euclid.mosize = euclid.vmosize;
        euclid.models = realloc(euclid.models, sizeof(euclidmodel) * euclid.mosize);
    }
    if(euclid.mesize != euclid.vmesize){
        euclid.mesize = euclid.vmesize;
        euclid.meshes = realloc(euclid.meshes, sizeof(euclidmesh) * euclid.mesize);
    }
    if(euclid.tsize != euclid.vtsize){
        euclid.tsize = euclid.vtsize;
        euclid.textures = realloc(euclid.textures, sizeof(euclidtexture) * euclid.tsize);
    }
    if (euclid.handle[eh].debug == 1) printf("\e[1;35mEuclidVK\e[0;37m: Material size = %i\n", euclid.msize);
    if (euclid.handle[eh].debug == 1) printf("\e[1;35mEuclidVK\e[0;37m: Models size = %i\n", euclid.mosize);
    if (euclid.handle[eh].debug == 1) printf("\e[1;35mEuclidVK\e[0;37m: Meshes size = %i\n", euclid.mesize);
    if (euclid.handle[eh].debug == 1) printf("\e[1;35mEuclidVK\e[0;37m: Textures size = %i\n", euclid.tsize);
    if (euclid.handle[eh].debug == 1) printf("\e[1;35mEuclidVK\e[0;37m: Creation queue size = %i\n", euclid.handle[eh].queuesize);
    for(uint32_t i = 0; i < euclid.handle[eh].queuesize; i++){
        if (euclid.handle[eh].debug == 1) printf("\e[1;35mEuclidVK\e[0;37m: Creating item at index = %i, type = %i\n", i, euclid.handle[eh].eq[i].type);
        switch(euclid.handle[eh].eq[i].type){
            case EUCLID_QUEUE_TYPE_MATERIAL:
                newmaterial_real(eh, euclid.handle[eh].eq[i].em, &euclid.handle[eh].eq[i]);
                break;
            case EUCLID_QUEUE_TYPE_MODEL:
                newmodel_real(eh, euclid.handle[eh].eq[i].em, &euclid.handle[eh].eq[i]);
                break;
            case EUCLID_QUEUE_TYPE_MESH:
                newmesh_real(eh, euclid.handle[eh].eq[i].meshid, &euclid.handle[eh].eq[i]);
                break;
            case EUCLID_QUEUE_TYPE_TEXTURE:
                newtexture_real(eh, euclid.handle[eh].eq[i].em, &euclid.handle[eh].eq[i]);
                break;
            default:
                break;
        }
        free_creation_queue_item(&euclid.handle[eh].eq[i]);
    }
    if (euclid.handle[eh].debug == 1) printf("\e[1;35mEuclidVK\e[0;37m: Creation complete\n");
    euclid.handle[eh].queuesize = 0;
    free(euclid.handle[eh].eq);
    euclid.handle[eh].eq = NULL;
}

static uint32_t newmaterial_real(uint32_t eh, uint32_t em, euclidCreationQueue *item){
    VkShaderModuleCreateInfo vcreateInfo = {0};
    vcreateInfo.sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
    vcreateInfo.codeSize = item->svert;
    vcreateInfo.pCode = (uint32_t*) item->vert;
    VkResult result = vkCreateShaderModule(euclid.handle[eh].device, &vcreateInfo, NULL, &euclid.materials[em].vertModule);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMT\e[0;37m: Vertex shader module created with result = %d\n", result);

    VkShaderModuleCreateInfo fcreateInfo = {0};
    fcreateInfo.sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
    fcreateInfo.codeSize = item->sfrag;
    fcreateInfo.pCode = (uint32_t*) item->frag;
    result = vkCreateShaderModule(euclid.handle[eh].device, &fcreateInfo, NULL, &euclid.materials[em].fragModule);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMT\e[0;37m: Fragment shader module created with result = %d\n", result);

    if(item->sshadow != 0){
        VkShaderModuleCreateInfo screateInfo = {0};
        screateInfo.sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
        screateInfo.codeSize = item->sshadow;
        screateInfo.pCode = (uint32_t*) item->shadow;
        result = vkCreateShaderModule(euclid.handle[eh].device, &screateInfo, NULL, &euclid.materials[em].shadowModule);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMT\e[0;37m: Shadow shader module created with result = %d\n", result);
    }

    euclid.materials[em].cullMode = item->cullmode;
    euclid.materials[em].shcullMode = item->scullmode;
    euclid.materials[em].polygonMode = VK_POLYGON_MODE_FILL;
    euclid.materials[em].lineWidth = 1.0;

    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMT\e[0;37m: Shader module created by id = %d\n", em);
    return em;
}

uint32_t newmaterial(uint32_t eh, uint32_t *vert, uint32_t *frag, uint32_t *shadow, uint32_t svert, uint32_t sfrag, uint32_t sshadow, uint32_t cullmode, uint32_t scullmode){
    uint32_t em = euclid.vmsize;
    euclid.vmsize++;

    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidOQ\e[0;37m: New material is being pushed in queue creation, index = %i\n", em);

    euclidCreationQueue item = {0};
    item.type = EUCLID_QUEUE_TYPE_MATERIAL;
    item.em = em;
    item.svert = svert;
    item.sfrag = sfrag;
    item.sshadow = sshadow;
    item.cullmode = cullmode;
    item.scullmode = scullmode;
    if(svert > 0){
        item.vert = malloc(svert);
        memcpy(item.vert, vert, svert);
    }
    if(sfrag > 0){
        item.frag = malloc(sfrag);
        memcpy(item.frag, frag, sfrag);
    }
    if(sshadow > 0){
        item.shadow = malloc(sshadow);
        memcpy(item.shadow, shadow, sshadow);
    }

    push_creation_queue(eh, &item);

    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidOQ\e[0;37m: New material pushed in queue creation, index = %i\n", em);

    return em;
}

static uint32_t newmodel_real(uint32_t eh, uint32_t em, euclidCreationQueue *item){
    float *tg = malloc(sizeof(float)*item->size*3);
    float *ctg = malloc(sizeof(float)*item->size*3);
    for (uint32_t i = 0, u = 0; i < item->size*3; i+=9, u+=6){
        float v0[] = { item->vertices[i], item->vertices[i+1], item->vertices[i+2] };
        float v1[] = { item->vertices[i+3], item->vertices[i+4], item->vertices[i+5] };
        float v2[] = { item->vertices[i+6], item->vertices[i+7], item->vertices[i+8] };
        float uv0[] = { item->uv[u], item->uv[u+1]+1.0f };
        float uv1[] = { item->uv[u+2], item->uv[u+3]+1.0f };
        float uv2[] = { item->uv[u+4], item->uv[u+5]+1.0f };
        float deltapos1[] = { v1[0]-v0[0], v1[1]-v0[1], v1[2]-v0[2]};
        float deltapos2[] = { v2[0]-v0[0], v2[1]-v0[1], v2[2]-v0[2]};
        float delta_uv1[] = {uv1[0]-uv0[0], uv1[1]-uv0[1]};
        float delta_uv2[] = {uv2[0]-uv0[0], uv2[1]-uv0[1]};
        float r = 1.0 / (delta_uv1[0] * delta_uv2[1] - delta_uv1[1] * delta_uv2[0]);
        tg[i] = (deltapos1[0] * delta_uv2[1] - deltapos2[0] * delta_uv1[1])*r;
        tg[i+1] = (deltapos1[1] * delta_uv2[1] - deltapos2[1] * delta_uv1[1])*r;
        tg[i+2] = (deltapos1[2] * delta_uv2[1] - deltapos2[2] * delta_uv1[1])*r;
        tg[i+3] = (deltapos1[0] * delta_uv2[1] - deltapos2[0] * delta_uv1[1])*r;
        tg[i+4] = (deltapos1[1] * delta_uv2[1] - deltapos2[1] * delta_uv1[1])*r;
        tg[i+5] = (deltapos1[2] * delta_uv2[1] - deltapos2[2] * delta_uv1[1])*r;
        tg[i+6] = (deltapos1[0] * delta_uv2[1] - deltapos2[0] * delta_uv1[1])*r;
        tg[i+7] = (deltapos1[1] * delta_uv2[1] - deltapos2[1] * delta_uv1[1])*r;
        tg[i+8] = (deltapos1[2] * delta_uv2[1] - deltapos2[2] * delta_uv1[1])*r;
        ctg[i] = (deltapos2[0] * delta_uv1[0] - deltapos1[0] * delta_uv2[0])*r;
        ctg[i+1] = (deltapos2[1] * delta_uv1[0] - deltapos1[1] * delta_uv2[0])*r;
        ctg[i+2] = (deltapos2[2] * delta_uv1[0] - deltapos1[2] * delta_uv2[0])*r;
        ctg[i+3] = (deltapos2[0] * delta_uv1[0] - deltapos1[0] * delta_uv2[0])*r;
        ctg[i+4] = (deltapos2[1] * delta_uv1[0] - deltapos1[1] * delta_uv2[0])*r;
        ctg[i+5] = (deltapos2[2] * delta_uv1[0] - deltapos1[2] * delta_uv2[0])*r;
        ctg[i+6] = (deltapos2[0] * delta_uv1[0] - deltapos1[0] * delta_uv2[0])*r;
        ctg[i+7] = (deltapos2[1] * delta_uv1[0] - deltapos1[1] * delta_uv2[0])*r;
        ctg[i+8] = (deltapos2[2] * delta_uv1[0] - deltapos1[2] * delta_uv2[0])*r;
    }
    float *res = malloc(sizeof(float)*item->size*14);
    for(uint32_t i = 0, v = 0, u = 0; i < item->size*14; i+=14, v+=3, u+=2){
        res[i] = item->vertices[v];
        res[i+1] = item->vertices[v+1];
        res[i+2] = item->vertices[v+2];
        res[i+3] = item->uv[u];
        res[i+4] = item->uv[u+1];
        res[i+5] = item->normals[v];
        res[i+6] = item->normals[v+1];
        res[i+7] = item->normals[v+2];
        res[i+8] = tg[v];
        res[i+9] = tg[v+1];
        res[i+10] = tg[v+2];
        res[i+11] = ctg[v];
        res[i+12] = ctg[v+1];
        res[i+13] = ctg[v+2];
    }
    free(tg);
    free(ctg);

    VkBuffer stagingBuffer;
    VkDeviceMemory stagingBufferMemory;
    VkDeviceSize bufferSize = sizeof(float) * item->size * 14;
    VkBufferCreateInfo bufferInfo = {0};
    bufferInfo.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
    bufferInfo.size = bufferSize;
    bufferInfo.usage = VK_BUFFER_USAGE_VERTEX_BUFFER_BIT | VK_BUFFER_USAGE_TRANSFER_DST_BIT;
    bufferInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
    VkResult result = vkCreateBuffer(euclid.handle[eh].device, &bufferInfo, NULL, &stagingBuffer);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Vertex buffer created with result = %d\n", result);

    VkMemoryRequirements memRequirements;
    vkGetBufferMemoryRequirements(euclid.handle[eh].device, stagingBuffer, &memRequirements);
    VkMemoryAllocateInfo allocInfo = {0};
    allocInfo.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    allocInfo.allocationSize = memRequirements.size;
    allocInfo.memoryTypeIndex = findMemoryType(memRequirements.memoryTypeBits, VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT, eh);
    vkAllocateMemory(euclid.handle[eh].device, &allocInfo, NULL, &stagingBufferMemory);
    void* data;
    vkMapMemory(euclid.handle[eh].device, stagingBufferMemory, 0, bufferSize, 0, &data);
    memcpy(data, res, bufferSize);
    vkUnmapMemory(euclid.handle[eh].device, stagingBufferMemory);
    vkBindBufferMemory(euclid.handle[eh].device, stagingBuffer, stagingBufferMemory, 0);

    euclid.models[em].vertexBuffer = stagingBuffer;
    euclid.models[em].vertexBufferMemory = stagingBufferMemory;
    euclid.models[em].vertnum = item->size;
    free(res);
    return em;
}

uint32_t newmodel(uint32_t eh, float *vertices, float *uv, float *normals, uint32_t size){
    uint32_t em = euclid.vmosize;
    euclid.vmosize++;

    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidOQ\e[0;37m: New model is being pushed in queue creation, index = %i\n", em);

    euclidCreationQueue item = {0};
    item.type = EUCLID_QUEUE_TYPE_MODEL;
    item.size = size;
    item.em = em;
    if(size > 0){
        item.vertices = malloc(size*sizeof(float)*3);
        item.uv = malloc(size*sizeof(float)*2);
        item.normals = malloc(size*sizeof(float)*3);
        memcpy(item.vertices, vertices, size*sizeof(float)*3);
        memcpy(item.uv, uv, size*sizeof(float)*2);
        memcpy(item.normals, normals, size*sizeof(float)*3);
    }

    push_creation_queue(eh, &item);

    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidOQ\e[0;37m: New model pushed in queue creation, index = %i\n", em);

    return em;
}

void createDescriptorSetLayout(uint32_t eh, uint32_t eme) {
    uint32_t texCount = euclid.meshes[eme].texnm;
    uint32_t bindingCount = 11 + (texCount > 0 ? 2 * (texCount - 1) : 0);
    VkDescriptorSetLayoutBinding *uboLayoutBinding = malloc(sizeof(VkDescriptorSetLayoutBinding) * bindingCount);
    memset(uboLayoutBinding, 0, sizeof(VkDescriptorSetLayoutBinding) * bindingCount);

    uboLayoutBinding[0].binding = 0;
    uboLayoutBinding[0].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    uboLayoutBinding[0].descriptorCount = 1;
    uboLayoutBinding[0].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[0].pImmutableSamplers = NULL;

    uboLayoutBinding[1].binding = 1;
    uboLayoutBinding[1].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    uboLayoutBinding[1].descriptorCount = 1;
    uboLayoutBinding[1].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[1].pImmutableSamplers = NULL;

    uboLayoutBinding[2].binding = 2;
    uboLayoutBinding[2].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    uboLayoutBinding[2].descriptorCount = 1;
    uboLayoutBinding[2].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[2].pImmutableSamplers = NULL;

    uboLayoutBinding[3].binding = 3;
    uboLayoutBinding[3].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    uboLayoutBinding[3].descriptorCount = 1;
    uboLayoutBinding[3].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[3].pImmutableSamplers = NULL;

    uboLayoutBinding[4].binding = 4;
    uboLayoutBinding[4].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    uboLayoutBinding[4].descriptorCount = 1;
    uboLayoutBinding[4].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[4].pImmutableSamplers = NULL;

    uboLayoutBinding[5].binding = 5;
    uboLayoutBinding[5].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    uboLayoutBinding[5].descriptorCount = 1;
    uboLayoutBinding[5].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[5].pImmutableSamplers = NULL;

    uboLayoutBinding[6].binding = 6;
    uboLayoutBinding[6].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    uboLayoutBinding[6].descriptorCount = 1;
    uboLayoutBinding[6].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[6].pImmutableSamplers = NULL;

    uboLayoutBinding[7].binding = 7;
    uboLayoutBinding[7].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    uboLayoutBinding[7].descriptorCount = 1;
    uboLayoutBinding[7].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[7].pImmutableSamplers = NULL;

    uboLayoutBinding[8].binding = 8;
    uboLayoutBinding[8].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    uboLayoutBinding[8].descriptorCount = 1;
    uboLayoutBinding[8].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[8].pImmutableSamplers = NULL;

    uboLayoutBinding[9].binding = 9;
    uboLayoutBinding[9].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLER;
    uboLayoutBinding[9].descriptorCount = 1;
    uboLayoutBinding[9].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[9].pImmutableSamplers = NULL;

    uboLayoutBinding[10].binding = 10;
    uboLayoutBinding[10].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLER;
    uboLayoutBinding[10].descriptorCount = 1;
    uboLayoutBinding[10].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[10].pImmutableSamplers = NULL;

    for(uint32_t i = 1; i < texCount; i++){
        uint32_t samplerBinding = 11 + (2 * (i - 1));
        uint32_t imageBinding = samplerBinding + 1;
        uboLayoutBinding[samplerBinding].binding = samplerBinding;
        uboLayoutBinding[samplerBinding].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
        uboLayoutBinding[samplerBinding].descriptorCount = 1;
        uboLayoutBinding[samplerBinding].stageFlags = VK_SHADER_STAGE_ALL;
        uboLayoutBinding[samplerBinding].pImmutableSamplers = NULL;

        uboLayoutBinding[imageBinding].binding = imageBinding;
        uboLayoutBinding[imageBinding].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLER;
        uboLayoutBinding[imageBinding].descriptorCount = 1;
        uboLayoutBinding[imageBinding].stageFlags = VK_SHADER_STAGE_ALL;
        uboLayoutBinding[imageBinding].pImmutableSamplers = NULL;
    }

    VkDescriptorSetLayoutCreateInfo layoutInfo = {0};
    layoutInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO;
    layoutInfo.bindingCount = bindingCount;
    layoutInfo.pBindings = uboLayoutBinding;
    VkResult result = vkCreateDescriptorSetLayout(euclid.handle[eh].device, &layoutInfo, NULL, &euclid.meshes[eme].descriptorSetLayout);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Descriptor set layout created with result = %d\n", result);
    free(uboLayoutBinding);
}

void createLightingDescriptorSetLayout(uint32_t eh, uint32_t eme) {
    uint32_t texCount = euclid.meshes[eme].texnm;
    uint32_t bindingCount = 9 + (texCount > 0 ? 2 * (texCount - 1) : 0);
    VkDescriptorSetLayoutBinding *uboLayoutBinding = malloc(sizeof(VkDescriptorSetLayoutBinding) * bindingCount);
    memset(uboLayoutBinding, 0, sizeof(VkDescriptorSetLayoutBinding) * bindingCount);

    uboLayoutBinding[0].binding = 0;
    uboLayoutBinding[0].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    uboLayoutBinding[0].descriptorCount = 1;
    uboLayoutBinding[0].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[0].pImmutableSamplers = NULL;

    uboLayoutBinding[1].binding = 1;
    uboLayoutBinding[1].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    uboLayoutBinding[1].descriptorCount = 1;
    uboLayoutBinding[1].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[1].pImmutableSamplers = NULL;

    uboLayoutBinding[2].binding = 2;
    uboLayoutBinding[2].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    uboLayoutBinding[2].descriptorCount = 1;
    uboLayoutBinding[2].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[2].pImmutableSamplers = NULL;

    uboLayoutBinding[3].binding = 3;
    uboLayoutBinding[3].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    uboLayoutBinding[3].descriptorCount = 1;
    uboLayoutBinding[3].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[3].pImmutableSamplers = NULL;

    uboLayoutBinding[4].binding = 4;
    uboLayoutBinding[4].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    uboLayoutBinding[4].descriptorCount = 1;
    uboLayoutBinding[4].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[4].pImmutableSamplers = NULL;

    uboLayoutBinding[5].binding = 5;
    uboLayoutBinding[5].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    uboLayoutBinding[5].descriptorCount = 1;
    uboLayoutBinding[5].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[5].pImmutableSamplers = NULL;

    uboLayoutBinding[6].binding = 6;
    uboLayoutBinding[6].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    uboLayoutBinding[6].descriptorCount = 1;
    uboLayoutBinding[6].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[6].pImmutableSamplers = NULL;

    uboLayoutBinding[7].binding = 7;
    uboLayoutBinding[7].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLER;
    uboLayoutBinding[7].descriptorCount = 1;
    uboLayoutBinding[7].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[7].pImmutableSamplers = NULL;

    uboLayoutBinding[8].binding = 8;
    uboLayoutBinding[8].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLER;
    uboLayoutBinding[8].descriptorCount = 1;
    uboLayoutBinding[8].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[8].pImmutableSamplers = NULL;

    for(uint32_t i = 1; i < texCount; i++){
        uint32_t samplerBinding = 9 + (2 * (i - 1));
        uint32_t imageBinding = samplerBinding + 1;
        uboLayoutBinding[samplerBinding].binding = samplerBinding;
        uboLayoutBinding[samplerBinding].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
        uboLayoutBinding[samplerBinding].descriptorCount = 1;
        uboLayoutBinding[samplerBinding].stageFlags = VK_SHADER_STAGE_ALL;
        uboLayoutBinding[samplerBinding].pImmutableSamplers = NULL;

        uboLayoutBinding[imageBinding].binding = imageBinding;
        uboLayoutBinding[imageBinding].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLER;
        uboLayoutBinding[imageBinding].descriptorCount = 1;
        uboLayoutBinding[imageBinding].stageFlags = VK_SHADER_STAGE_ALL;
        uboLayoutBinding[imageBinding].pImmutableSamplers = NULL;
    }

    VkDescriptorSetLayoutCreateInfo layoutInfo = {0};
    layoutInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO;
    layoutInfo.bindingCount = bindingCount;
    layoutInfo.pBindings = uboLayoutBinding;
    VkResult result = vkCreateDescriptorSetLayout(euclid.handle[eh].device, &layoutInfo, NULL, &euclid.meshes[eme].lightingDescriptorSetLayout);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Lighting descriptor set layout created with result = %d\n", result);
    free(uboLayoutBinding);
}

void createShadowDescriptorSetLayout(uint32_t eh, uint32_t eme) {
    VkDescriptorSetLayoutBinding uboLayoutBinding[2] = {0};
    uboLayoutBinding[0].binding = 0;
    uboLayoutBinding[0].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    uboLayoutBinding[0].descriptorCount = 1;
    uboLayoutBinding[0].stageFlags = VK_SHADER_STAGE_VERTEX_BIT;
    uboLayoutBinding[0].pImmutableSamplers = NULL;

    uboLayoutBinding[1].binding = 1;
    uboLayoutBinding[1].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    uboLayoutBinding[1].descriptorCount = 1;
    uboLayoutBinding[1].stageFlags = VK_SHADER_STAGE_VERTEX_BIT;
    uboLayoutBinding[1].pImmutableSamplers = NULL;

    VkDescriptorSetLayoutCreateInfo layoutInfo = {0};
    layoutInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO;
    layoutInfo.bindingCount = 2;
    layoutInfo.pBindings = uboLayoutBinding;
    VkResult result = vkCreateDescriptorSetLayout(euclid.handle[eh].device, &layoutInfo, NULL, &euclid.meshes[eme].shadowDescriptorSetLayout);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Shadow Descriptor set layout created with result = %d\n", result);
}

void createDefferedDescriptorSetLayout(uint32_t eh, uint32_t eme) {
    uint32_t texCount = euclid.meshes[eme].texnm;
    uint32_t bindingCount = 4 + (texCount > 0 ? 2 * (texCount - 1) : 0);
    VkDescriptorSetLayoutBinding *uboLayoutBinding = malloc(sizeof(VkDescriptorSetLayoutBinding) * bindingCount);
    memset(uboLayoutBinding, 0, sizeof(VkDescriptorSetLayoutBinding) * bindingCount);

    uboLayoutBinding[0].binding = 0;
    uboLayoutBinding[0].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    uboLayoutBinding[0].descriptorCount = 1;
    uboLayoutBinding[0].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[0].pImmutableSamplers = NULL;

    uboLayoutBinding[1].binding = 1;
    uboLayoutBinding[1].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    uboLayoutBinding[1].descriptorCount = 1;
    uboLayoutBinding[1].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[1].pImmutableSamplers = NULL;

    uboLayoutBinding[2].binding = 2;
    uboLayoutBinding[2].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    uboLayoutBinding[2].descriptorCount = 1;
    uboLayoutBinding[2].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[2].pImmutableSamplers = NULL;

    uboLayoutBinding[3].binding = 3;
    uboLayoutBinding[3].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLER;
    uboLayoutBinding[3].descriptorCount = 1;
    uboLayoutBinding[3].stageFlags = VK_SHADER_STAGE_ALL;
    uboLayoutBinding[3].pImmutableSamplers = NULL;

    for(uint32_t i = 1; i < texCount; i++){
        uint32_t samplerBinding = 4 + (2 * (i - 1));
        uint32_t imageBinding = samplerBinding + 1;
        uboLayoutBinding[samplerBinding].binding = samplerBinding;
        uboLayoutBinding[samplerBinding].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
        uboLayoutBinding[samplerBinding].descriptorCount = 1;
        uboLayoutBinding[samplerBinding].stageFlags = VK_SHADER_STAGE_ALL;
        uboLayoutBinding[samplerBinding].pImmutableSamplers = NULL;

        uboLayoutBinding[imageBinding].binding = imageBinding;
        uboLayoutBinding[imageBinding].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLER;
        uboLayoutBinding[imageBinding].descriptorCount = 1;
        uboLayoutBinding[imageBinding].stageFlags = VK_SHADER_STAGE_ALL;
        uboLayoutBinding[imageBinding].pImmutableSamplers = NULL;
    }

    VkDescriptorSetLayoutCreateInfo layoutInfo = {0};
    layoutInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO;
    layoutInfo.bindingCount = bindingCount;
    layoutInfo.pBindings = uboLayoutBinding;
    VkResult result = vkCreateDescriptorSetLayout(euclid.handle[eh].device, &layoutInfo, NULL, &euclid.meshes[eme].defferedDescriptorSetLayout);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Deffered Descriptor set layout created with result = %d\n", result);
    free(uboLayoutBinding);
}

void createUniformBuffer(uint32_t eh, uint32_t eme){
    VkDeviceSize bufferSize = sizeof(float)*60;

    euclid.meshes[eme].uniformBuffers = malloc(sizeof(VkBuffer)*(MAX_FRAMES_IN_FLIGHT+1));
    euclid.meshes[eme].uniformBuffersMemory = malloc(sizeof(VkDeviceMemory)*(MAX_FRAMES_IN_FLIGHT+1));
    euclid.meshes[eme].uniformBuffersMapped = malloc(sizeof(void*)*(MAX_FRAMES_IN_FLIGHT+1));
    for (size_t i = 0; i < MAX_FRAMES_IN_FLIGHT+1; i++) {
        VkBufferCreateInfo bufferInfo = {0};
        bufferInfo.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        bufferInfo.size = bufferSize;
        bufferInfo.usage = VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT;
        bufferInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
        VkResult result = vkCreateBuffer(euclid.handle[eh].device, &bufferInfo, NULL, &euclid.meshes[eme].uniformBuffers[i]);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMD\e[0;37m: Uniform buffer created with result = %d\n", result);
        VkMemoryRequirements memRequirements;
        vkGetBufferMemoryRequirements(euclid.handle[eh].device, euclid.meshes[eme].uniformBuffers[i], &memRequirements);
        
        VkMemoryAllocateInfo allocInfo = {0};
        allocInfo.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        allocInfo.allocationSize = memRequirements.size;
        allocInfo.memoryTypeIndex = findMemoryType(memRequirements.memoryTypeBits, VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT, eh);
        
        vkAllocateMemory(euclid.handle[eh].device, &allocInfo, NULL, &euclid.meshes[eme].uniformBuffersMemory[i]);
        vkBindBufferMemory(euclid.handle[eh].device, euclid.meshes[eme].uniformBuffers[i], euclid.meshes[eme].uniformBuffersMemory[i], 0);

        vkMapMemory(euclid.handle[eh].device, euclid.meshes[eme].uniformBuffersMemory[i], 0, bufferSize, 0, &euclid.meshes[eme].uniformBuffersMapped[i]);
    }
}

void createDescriptorPool(uint32_t eh, uint32_t eme){
    uint32_t texCount = euclid.meshes[eme].texnm;
    VkDescriptorPoolSize *poolSize = malloc(sizeof(VkDescriptorPoolSize) * 3);
    memset(poolSize, 0, sizeof(VkDescriptorPoolSize) * 3);

    poolSize[0].type = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    poolSize[0].descriptorCount = MAX_FRAMES_IN_FLIGHT * 3;

    poolSize[1].type = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    poolSize[1].descriptorCount = MAX_FRAMES_IN_FLIGHT * (5 + texCount);

    poolSize[2].type = VK_DESCRIPTOR_TYPE_SAMPLER;
    poolSize[2].descriptorCount = MAX_FRAMES_IN_FLIGHT * (2 + texCount);

    VkDescriptorPoolCreateInfo poolInfo = {0};
    poolInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO;
    poolInfo.poolSizeCount = 3;
    poolInfo.pPoolSizes = poolSize;
    poolInfo.maxSets = MAX_FRAMES_IN_FLIGHT;

    VkResult result = vkCreateDescriptorPool(euclid.handle[eh].device, &poolInfo, NULL, &euclid.meshes[eme].descriptorPool);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Descriptor pool created with result = %d\n", result);
    free(poolSize);
}

void createLightingDescriptorPool(uint32_t eh, uint32_t eme){
    uint32_t texCount = euclid.meshes[eme].texnm;
    VkDescriptorPoolSize *poolSize = malloc(sizeof(VkDescriptorPoolSize) * 3);
    memset(poolSize, 0, sizeof(VkDescriptorPoolSize) * 3);

    poolSize[0].type = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    poolSize[0].descriptorCount = 3;

    poolSize[1].type = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    poolSize[1].descriptorCount = 3 + texCount;

    poolSize[2].type = VK_DESCRIPTOR_TYPE_SAMPLER;
    poolSize[2].descriptorCount = 2 + texCount;

    VkDescriptorPoolCreateInfo poolInfo = {0};
    poolInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO;
    poolInfo.poolSizeCount = 3;
    poolInfo.pPoolSizes = poolSize;
    poolInfo.maxSets = 1;

    VkResult result = vkCreateDescriptorPool(euclid.handle[eh].device, &poolInfo, NULL, &euclid.meshes[eme].lightingDescriptorPool);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Lighting descriptor pool created with result = %d\n", result);
    free(poolSize);
}

void createShadowDescriptorPool(uint32_t eh, uint32_t eme){
    VkDescriptorPoolSize poolSize[2] = {0};
    poolSize[0].type = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    poolSize[0].descriptorCount = 100;

    poolSize[1].type = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    poolSize[1].descriptorCount = 100;

    VkDescriptorPoolCreateInfo poolInfo = {0};
    poolInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO;
    poolInfo.poolSizeCount = 2;
    poolInfo.pPoolSizes = poolSize;
    poolInfo.maxSets = 100;

    VkResult result = vkCreateDescriptorPool(euclid.handle[eh].device, &poolInfo, NULL, &euclid.meshes[eme].shadowDescriptorPool);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Shadow descriptor pool created with result = %d\n", result);
}

void createDeferredDescriptorPool(uint32_t eh, uint32_t eme){
    uint32_t texCount = euclid.meshes[eme].texnm;
    VkDescriptorPoolSize *poolSize = malloc(sizeof(VkDescriptorPoolSize) * 4);
    memset(poolSize, 0, sizeof(VkDescriptorPoolSize) * 4);

    poolSize[0].type = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    poolSize[0].descriptorCount = 20;

    poolSize[1].type = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    poolSize[1].descriptorCount = 20;

    poolSize[2].type = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    poolSize[2].descriptorCount = 10 + texCount;

    poolSize[3].type = VK_DESCRIPTOR_TYPE_SAMPLER;
    poolSize[3].descriptorCount = 10 + texCount;

    VkDescriptorPoolCreateInfo poolInfo = {0};
    poolInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO;
    poolInfo.poolSizeCount = 4;
    poolInfo.pPoolSizes = poolSize;
    poolInfo.maxSets = 10;

    VkResult result = vkCreateDescriptorPool(euclid.handle[eh].device, &poolInfo, NULL, &euclid.meshes[eme].defferedDescriptorPool);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Deffered descriptor pool created with result = %d\n", result);
    free(poolSize);
}

void createDescriptorSets(uint32_t eh, uint32_t eme){
    VkDescriptorSetLayout *ldcs = malloc(sizeof(VkDescriptorSetLayout)*MAX_FRAMES_IN_FLIGHT);
    for(int i = 0; i != MAX_FRAMES_IN_FLIGHT; i++){
        ldcs[i] = euclid.meshes[eme].descriptorSetLayout;
    }

    VkDescriptorSetAllocateInfo allocInfo = {0};
    allocInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO;
    allocInfo.descriptorPool = euclid.meshes[eme].descriptorPool;
    allocInfo.descriptorSetCount = MAX_FRAMES_IN_FLIGHT;
    allocInfo.pSetLayouts = ldcs;

    euclid.meshes[eme].descriptorSets = malloc(sizeof(VkDescriptorSet)*MAX_FRAMES_IN_FLIGHT);
    VkResult result = vkAllocateDescriptorSets(euclid.handle[eh].device, &allocInfo, euclid.meshes[eme].descriptorSets);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Descriptor sets allocated with result = %d\n", result);

    uint32_t texCount = euclid.meshes[eme].texnm;
    uint32_t descriptorWriteCount = 11 + (texCount > 0 ? 2 * (texCount - 1) : 0);
    VkDescriptorImageInfo *textureImageInfos = malloc(sizeof(VkDescriptorImageInfo) * texCount);
    VkDescriptorImageInfo *textureSamplerInfos = malloc(sizeof(VkDescriptorImageInfo) * texCount);

    for (size_t i = 0; i < MAX_FRAMES_IN_FLIGHT; i++) {
        VkDescriptorBufferInfo bufferInfo = {0};
        bufferInfo.buffer = euclid.meshes[eme].uniformBuffers[i];
        bufferInfo.offset = 0;
        bufferInfo.range = 64*sizeof(float);

        VkDescriptorBufferInfo shbufferInfo = {0};
        shbufferInfo.buffer = euclid.handle[eh].shadowUniformBuffer;
        shbufferInfo.offset = 0;
        shbufferInfo.range = 2400*sizeof(float);

        VkDescriptorBufferInfo dfbufferInfo = {0};
        dfbufferInfo.buffer = euclid.handle[eh].defferedUniformBuffer;
        dfbufferInfo.offset = 0;
        dfbufferInfo.range = 400*sizeof(float);

        VkDescriptorImageInfo colInfo = {0};
        colInfo.imageLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
        colInfo.imageView = euclid.handle[eh].lightingImageViewl;
        colInfo.sampler = euclid.handle[eh].attachmentSampler;

        VkDescriptorImageInfo depthInfo = {0};
        depthInfo.imageLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
        depthInfo.imageView = euclid.handle[eh].lightingDepthImageViewl;
        depthInfo.sampler = euclid.handle[eh].attachmentSampler;

        VkDescriptorImageInfo deffcolInfo = {0};
        deffcolInfo.imageLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
        deffcolInfo.imageView = euclid.handle[eh].defferedImageView;
        deffcolInfo.sampler = euclid.handle[eh].attachmentSampler;

        VkDescriptorImageInfo deffdepthInfo = {0};
        deffdepthInfo.imageLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
        deffdepthInfo.imageView = euclid.handle[eh].defferedDepthImageView;
        deffdepthInfo.sampler = euclid.handle[eh].attachmentSampler;

        VkDescriptorImageInfo shInfo = {0};
        shInfo.imageLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
        shInfo.imageView = euclid.handle[eh].shadowImageView;
        shInfo.sampler = euclid.handle[eh].attachmentSampler;

        VkDescriptorImageInfo attsam = {0};
        attsam.imageLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
        attsam.sampler = euclid.handle[eh].attachmentSampler;

        for (uint32_t j = 0; j < texCount; j++) {
            textureImageInfos[j].imageLayout = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
            textureImageInfos[j].imageView = euclid.textures[euclid.meshes[eme].savedtex[j]].textureImageView;
            textureImageInfos[j].sampler = euclid.textures[euclid.meshes[eme].savedtex[j]].sampler;
            textureSamplerInfos[j].imageLayout = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
            textureSamplerInfos[j].sampler = euclid.textures[euclid.meshes[eme].savedtex[j]].sampler;
        }

        VkWriteDescriptorSet *descriptorWrite = malloc(sizeof(VkWriteDescriptorSet) * descriptorWriteCount);
        memset(descriptorWrite, 0, sizeof(VkWriteDescriptorSet) * descriptorWriteCount);
        uint32_t writeIndex = 0;

        descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].descriptorSets[i];
        descriptorWrite[writeIndex].dstBinding = 0;
        descriptorWrite[writeIndex].dstArrayElement = 0;
        descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
        descriptorWrite[writeIndex].descriptorCount = 1;
        descriptorWrite[writeIndex].pBufferInfo = &bufferInfo;
        writeIndex++;

        descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].descriptorSets[i];
        descriptorWrite[writeIndex].dstBinding = 1;
        descriptorWrite[writeIndex].dstArrayElement = 0;
        descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
        descriptorWrite[writeIndex].descriptorCount = 1;
        descriptorWrite[writeIndex].pBufferInfo = &shbufferInfo;
        writeIndex++;

        descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].descriptorSets[i];
        descriptorWrite[writeIndex].dstBinding = 2;
        descriptorWrite[writeIndex].dstArrayElement = 0;
        descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
        descriptorWrite[writeIndex].descriptorCount = 1;
        descriptorWrite[writeIndex].pBufferInfo = &dfbufferInfo;
        writeIndex++;

        descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].descriptorSets[i];
        descriptorWrite[writeIndex].dstBinding = 3;
        descriptorWrite[writeIndex].dstArrayElement = 0;
        descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
        descriptorWrite[writeIndex].descriptorCount = 1;
        descriptorWrite[writeIndex].pImageInfo = &textureImageInfos[0];
        writeIndex++;

        descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].descriptorSets[i];
        descriptorWrite[writeIndex].dstBinding = 4;
        descriptorWrite[writeIndex].dstArrayElement = 0;
        descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
        descriptorWrite[writeIndex].descriptorCount = 1;
        descriptorWrite[writeIndex].pImageInfo = &colInfo;
        writeIndex++;

        descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].descriptorSets[i];
        descriptorWrite[writeIndex].dstBinding = 5;
        descriptorWrite[writeIndex].dstArrayElement = 0;
        descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
        descriptorWrite[writeIndex].descriptorCount = 1;
        descriptorWrite[writeIndex].pImageInfo = &depthInfo;
        writeIndex++;

        descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].descriptorSets[i];
        descriptorWrite[writeIndex].dstBinding = 6;
        descriptorWrite[writeIndex].dstArrayElement = 0;
        descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
        descriptorWrite[writeIndex].descriptorCount = 1;
        descriptorWrite[writeIndex].pImageInfo = &deffcolInfo;
        writeIndex++;

        descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].descriptorSets[i];
        descriptorWrite[writeIndex].dstBinding = 7;
        descriptorWrite[writeIndex].dstArrayElement = 0;
        descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
        descriptorWrite[writeIndex].descriptorCount = 1;
        descriptorWrite[writeIndex].pImageInfo = &deffdepthInfo;
        writeIndex++;

        descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].descriptorSets[i];
        descriptorWrite[writeIndex].dstBinding = 8;
        descriptorWrite[writeIndex].dstArrayElement = 0;
        descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
        descriptorWrite[writeIndex].descriptorCount = 1;
        descriptorWrite[writeIndex].pImageInfo = &shInfo;
        writeIndex++;

        descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].descriptorSets[i];
        descriptorWrite[writeIndex].dstBinding = 9;
        descriptorWrite[writeIndex].dstArrayElement = 0;
        descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLER;
        descriptorWrite[writeIndex].descriptorCount = 1;
        descriptorWrite[writeIndex].pImageInfo = &textureSamplerInfos[0];
        writeIndex++;

        descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].descriptorSets[i];
        descriptorWrite[writeIndex].dstBinding = 10;
        descriptorWrite[writeIndex].dstArrayElement = 0;
        descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLER;
        descriptorWrite[writeIndex].descriptorCount = 1;
        descriptorWrite[writeIndex].pImageInfo = &attsam;
        writeIndex++;

        for (uint32_t j = 1; j < texCount; j++) {
            uint32_t imageBinding = 11 + (2 * (j - 1));
            uint32_t samplerBinding = imageBinding + 1;
            descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
            descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].descriptorSets[i];
            descriptorWrite[writeIndex].dstBinding = imageBinding;
            descriptorWrite[writeIndex].dstArrayElement = 0;
            descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
            descriptorWrite[writeIndex].descriptorCount = 1;
            descriptorWrite[writeIndex].pImageInfo = &textureImageInfos[j];
            writeIndex++;

            descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
            descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].descriptorSets[i];
            descriptorWrite[writeIndex].dstBinding = samplerBinding;
            descriptorWrite[writeIndex].dstArrayElement = 0;
            descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLER;
            descriptorWrite[writeIndex].descriptorCount = 1;
            descriptorWrite[writeIndex].pImageInfo = &textureSamplerInfos[j];
            writeIndex++;
        }

        vkUpdateDescriptorSets(euclid.handle[eh].device, descriptorWriteCount, descriptorWrite, 0, NULL);
        free(descriptorWrite);
    }
    free(textureImageInfos);
    free(textureSamplerInfos);
    free(ldcs);
}

void createLightingDescriptorSets(uint32_t eh, uint32_t eme){
    VkDescriptorSetLayout *ldcs = malloc(sizeof(VkDescriptorSetLayout)*2);
    ldcs[0] = euclid.meshes[eme].lightingDescriptorSetLayout;

    VkDescriptorSetAllocateInfo allocInfo = {0};
    allocInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO;
    allocInfo.descriptorPool = euclid.meshes[eme].lightingDescriptorPool;
    allocInfo.descriptorSetCount = 1;
    allocInfo.pSetLayouts = ldcs;

    euclid.meshes[eme].lightingDescriptorSets = malloc(sizeof(VkDescriptorSet)*2);
    VkResult result = vkAllocateDescriptorSets(euclid.handle[eh].device, &allocInfo, &euclid.meshes[eme].lightingDescriptorSets);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Descriptor sets allocated with result = %d\n", result);

    uint32_t texCount = euclid.meshes[eme].texnm;
    uint32_t descriptorWriteCount = 9 + (texCount > 0 ? 2 * (texCount - 1) : 0);
    VkDescriptorImageInfo *textureImageInfos = malloc(sizeof(VkDescriptorImageInfo) * texCount);
    VkDescriptorImageInfo *textureSamplerInfos = malloc(sizeof(VkDescriptorImageInfo) * texCount);

    VkDescriptorBufferInfo bufferInfo = {0};
    bufferInfo.buffer = euclid.meshes[eme].uniformBuffers[MAX_FRAMES_IN_FLIGHT];
    bufferInfo.offset = 0;
    bufferInfo.range = 64*sizeof(float);

    VkDescriptorBufferInfo shbufferInfo = {0};
    shbufferInfo.buffer = euclid.handle[eh].shadowUniformBuffer;
    shbufferInfo.offset = 0;
    shbufferInfo.range = 2400*sizeof(float);

    VkDescriptorBufferInfo dfbufferInfo = {0};
    dfbufferInfo.buffer = euclid.handle[eh].defferedUniformBuffer;
    dfbufferInfo.offset = 0;
    dfbufferInfo.range = 400*sizeof(float);

    VkDescriptorImageInfo colInfo = {0};
    colInfo.imageLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
    colInfo.imageView = euclid.handle[eh].defferedImageView;
    colInfo.sampler = euclid.handle[eh].attachmentSampler;

    VkDescriptorImageInfo depthInfo = {0};
    depthInfo.imageLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
    depthInfo.imageView = euclid.handle[eh].defferedDepthImageView;
    depthInfo.sampler = euclid.handle[eh].attachmentSampler;

    VkDescriptorImageInfo shInfo = {0};
    shInfo.imageLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
    shInfo.imageView = euclid.handle[eh].shadowImageView;
    shInfo.sampler = euclid.handle[eh].attachmentSampler;

    VkDescriptorImageInfo attsam = {0};
    attsam.imageLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
    attsam.sampler = euclid.handle[eh].attachmentSampler;

    for (uint32_t j = 0; j < texCount; j++) {
        textureImageInfos[j].imageLayout = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
        textureImageInfos[j].imageView = euclid.textures[euclid.meshes[eme].savedtex[j]].textureImageView;
        textureImageInfos[j].sampler = euclid.textures[euclid.meshes[eme].savedtex[j]].sampler;
        textureSamplerInfos[j].imageLayout = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
        textureSamplerInfos[j].sampler = euclid.textures[euclid.meshes[eme].savedtex[j]].sampler;
    }

    VkWriteDescriptorSet *descriptorWrite = malloc(sizeof(VkWriteDescriptorSet) * descriptorWriteCount);
    memset(descriptorWrite, 0, sizeof(VkWriteDescriptorSet) * descriptorWriteCount);
    uint32_t writeIndex = 0;

    descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
    descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].lightingDescriptorSets;
    descriptorWrite[writeIndex].dstBinding = 0;
    descriptorWrite[writeIndex].dstArrayElement = 0;
    descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    descriptorWrite[writeIndex].descriptorCount = 1;
    descriptorWrite[writeIndex].pBufferInfo = &bufferInfo;
    writeIndex++;

    descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
    descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].lightingDescriptorSets;
    descriptorWrite[writeIndex].dstBinding = 1;
    descriptorWrite[writeIndex].dstArrayElement = 0;
    descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    descriptorWrite[writeIndex].descriptorCount = 1;
    descriptorWrite[writeIndex].pBufferInfo = &shbufferInfo;
    writeIndex++;

    descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
    descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].lightingDescriptorSets;
    descriptorWrite[writeIndex].dstBinding = 2;
    descriptorWrite[writeIndex].dstArrayElement = 0;
    descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    descriptorWrite[writeIndex].descriptorCount = 1;
    descriptorWrite[writeIndex].pBufferInfo = &dfbufferInfo;
    writeIndex++;

    descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
    descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].lightingDescriptorSets;
    descriptorWrite[writeIndex].dstBinding = 3;
    descriptorWrite[writeIndex].dstArrayElement = 0;
    descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    descriptorWrite[writeIndex].descriptorCount = 1;
    descriptorWrite[writeIndex].pImageInfo = &textureImageInfos[0];
    writeIndex++;

    descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
    descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].lightingDescriptorSets;
    descriptorWrite[writeIndex].dstBinding = 4;
    descriptorWrite[writeIndex].dstArrayElement = 0;
    descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    descriptorWrite[writeIndex].descriptorCount = 1;
    descriptorWrite[writeIndex].pImageInfo = &colInfo;
    writeIndex++;

    descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
    descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].lightingDescriptorSets;
    descriptorWrite[writeIndex].dstBinding = 5;
    descriptorWrite[writeIndex].dstArrayElement = 0;
    descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    descriptorWrite[writeIndex].descriptorCount = 1;
    descriptorWrite[writeIndex].pImageInfo = &depthInfo;
    writeIndex++;

    descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
    descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].lightingDescriptorSets;
    descriptorWrite[writeIndex].dstBinding = 6;
    descriptorWrite[writeIndex].dstArrayElement = 0;
    descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
    descriptorWrite[writeIndex].descriptorCount = 1;
    descriptorWrite[writeIndex].pImageInfo = &shInfo;
    writeIndex++;

    descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
    descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].lightingDescriptorSets;
    descriptorWrite[writeIndex].dstBinding = 7;
    descriptorWrite[writeIndex].dstArrayElement = 0;
    descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLER;
    descriptorWrite[writeIndex].descriptorCount = 1;
    descriptorWrite[writeIndex].pImageInfo = &textureSamplerInfos[0];
    writeIndex++;

    descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
    descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].lightingDescriptorSets;
    descriptorWrite[writeIndex].dstBinding = 8;
    descriptorWrite[writeIndex].dstArrayElement = 0;
    descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLER;
    descriptorWrite[writeIndex].descriptorCount = 1;
    descriptorWrite[writeIndex].pImageInfo = &attsam;
    writeIndex++;

    for (uint32_t j = 1; j < texCount; j++) {
        uint32_t imageBinding = 9 + (2 * (j - 1));
        uint32_t samplerBinding = imageBinding + 1;
        descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].lightingDescriptorSets;
        descriptorWrite[writeIndex].dstBinding = imageBinding;
        descriptorWrite[writeIndex].dstArrayElement = 0;
        descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
        descriptorWrite[writeIndex].descriptorCount = 1;
        descriptorWrite[writeIndex].pImageInfo = &textureImageInfos[j];
        writeIndex++;

        descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].lightingDescriptorSets;
        descriptorWrite[writeIndex].dstBinding = samplerBinding;
        descriptorWrite[writeIndex].dstArrayElement = 0;
        descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLER;
        descriptorWrite[writeIndex].descriptorCount = 1;
        descriptorWrite[writeIndex].pImageInfo = &textureSamplerInfos[j];
        writeIndex++;
    }

    vkUpdateDescriptorSets(euclid.handle[eh].device, descriptorWriteCount, descriptorWrite, 0, NULL);
    free(descriptorWrite);
    free(textureImageInfos);
    free(textureSamplerInfos);
    free(ldcs);
}

void createShadowDescriptorSets(uint32_t eh, uint32_t eme){
    VkDescriptorSetLayout ldcs[100];
    for(int i = 0; i != 100; i++){
        ldcs[i] = euclid.meshes[eme].shadowDescriptorSetLayout;
    }

    VkDescriptorSetAllocateInfo allocInfo = {0};
    allocInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO;
    allocInfo.descriptorPool = euclid.meshes[eme].shadowDescriptorPool;
    allocInfo.descriptorSetCount = 100;
    allocInfo.pSetLayouts = ldcs;
    VkResult result = vkAllocateDescriptorSets(euclid.handle[eh].device, &allocInfo, euclid.meshes[eme].shadowDescriptorSets);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Shadow descriptor sets allocated with result = %d\n", result);

    for (size_t i = 0; i < 100; i++) {
        VkDescriptorBufferInfo bufferInfo[2] = {0};
        bufferInfo[0].buffer = euclid.handle[eh].shadowUniformBuffer;
        bufferInfo[0].offset = sizeof(float)*16*i;
        bufferInfo[0].range = sizeof(float)*16;

        bufferInfo[1].buffer = euclid.meshes[eme].uniformBuffers[MAX_FRAMES_IN_FLIGHT];
        bufferInfo[1].offset = 0;
        bufferInfo[1].range = sizeof(float)*64;

        VkWriteDescriptorSet descriptorWrite[2] = {0};
        descriptorWrite[0].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[0].dstSet = euclid.meshes[eme].shadowDescriptorSets[i];
        descriptorWrite[0].dstBinding = 0;
        descriptorWrite[0].dstArrayElement = 0;
        descriptorWrite[0].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
        descriptorWrite[0].descriptorCount = 1;
        descriptorWrite[0].pBufferInfo = &bufferInfo[0];
        descriptorWrite[0].pImageInfo = NULL;
        descriptorWrite[0].pTexelBufferView = NULL;

        descriptorWrite[1].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[1].dstSet = euclid.meshes[eme].shadowDescriptorSets[i];
        descriptorWrite[1].dstBinding = 1;
        descriptorWrite[1].dstArrayElement = 0;
        descriptorWrite[1].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
        descriptorWrite[1].descriptorCount = 1;
        descriptorWrite[1].pBufferInfo = &bufferInfo[1];
        descriptorWrite[1].pImageInfo = NULL;
        descriptorWrite[1].pTexelBufferView = NULL;

        vkUpdateDescriptorSets(euclid.handle[eh].device, 2, descriptorWrite, 0, NULL);
    }
}

void createDefferedDescriptorSets(uint32_t eh, uint32_t eme){
    VkDescriptorSetLayout ldcs[10];
    for(int i = 0; i != 10; i++){
        ldcs[i] = euclid.meshes[eme].defferedDescriptorSetLayout;
    }

    VkDescriptorSetAllocateInfo allocInfo = {0};
    allocInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO;
    allocInfo.descriptorPool = euclid.meshes[eme].defferedDescriptorPool;
    allocInfo.descriptorSetCount = 10;
    allocInfo.pSetLayouts = ldcs;
    VkResult result = vkAllocateDescriptorSets(euclid.handle[eh].device, &allocInfo, euclid.meshes[eme].defferedDescriptorSets);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Deffered descriptor sets allocated with result = %d\n", result);

    uint32_t texCount = euclid.meshes[eme].texnm;
    uint32_t descriptorWriteCount = 2 + (2 * texCount);
    VkDescriptorImageInfo *textureImageInfos = malloc(sizeof(VkDescriptorImageInfo) * texCount);
    VkDescriptorImageInfo *textureSamplerInfos = malloc(sizeof(VkDescriptorImageInfo) * texCount);

    for (size_t i = 0; i < 10; i++) {
        VkDescriptorBufferInfo bufferInfo[2] = {0};
        bufferInfo[0].buffer = euclid.handle[eh].defferedUniformBuffer;
        bufferInfo[0].offset = sizeof(float)*16*i;
        bufferInfo[0].range = sizeof(float)*16;

        bufferInfo[1].buffer = euclid.meshes[eme].uniformBuffers[MAX_FRAMES_IN_FLIGHT];
        bufferInfo[1].offset = 0;
        bufferInfo[1].range = sizeof(float)*64;

        for (uint32_t j = 0; j < texCount; j++) {
            textureImageInfos[j].imageLayout = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
            textureImageInfos[j].imageView = euclid.textures[euclid.meshes[eme].savedtex[j]].textureImageView;
            textureImageInfos[j].sampler = euclid.textures[euclid.meshes[eme].savedtex[j]].sampler;
            textureSamplerInfos[j].imageLayout = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
            textureSamplerInfos[j].sampler = euclid.textures[euclid.meshes[eme].savedtex[j]].sampler;
        }

        VkWriteDescriptorSet *descriptorWrite = malloc(sizeof(VkWriteDescriptorSet) * descriptorWriteCount);
        memset(descriptorWrite, 0, sizeof(VkWriteDescriptorSet) * descriptorWriteCount);
        uint32_t writeIndex = 0;

        descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].defferedDescriptorSets[i];
        descriptorWrite[writeIndex].dstBinding = 0;
        descriptorWrite[writeIndex].dstArrayElement = 0;
        descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
        descriptorWrite[writeIndex].descriptorCount = 1;
        descriptorWrite[writeIndex].pBufferInfo = &bufferInfo[0];
        writeIndex++;

        descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].defferedDescriptorSets[i];
        descriptorWrite[writeIndex].dstBinding = 1;
        descriptorWrite[writeIndex].dstArrayElement = 0;
        descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
        descriptorWrite[writeIndex].descriptorCount = 1;
        descriptorWrite[writeIndex].pBufferInfo = &bufferInfo[1];
        writeIndex++;

        for (uint32_t j = 0; j < texCount; j++) {
            uint32_t imageBinding = 2 + (2 * j);
            uint32_t samplerBinding = imageBinding + 1;
            descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
            descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].defferedDescriptorSets[i];
            descriptorWrite[writeIndex].dstBinding = imageBinding;
            descriptorWrite[writeIndex].dstArrayElement = 0;
            descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE;
            descriptorWrite[writeIndex].descriptorCount = 1;
            descriptorWrite[writeIndex].pImageInfo = &textureImageInfos[j];
            writeIndex++;

            descriptorWrite[writeIndex].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
            descriptorWrite[writeIndex].dstSet = euclid.meshes[eme].defferedDescriptorSets[i];
            descriptorWrite[writeIndex].dstBinding = samplerBinding;
            descriptorWrite[writeIndex].dstArrayElement = 0;
            descriptorWrite[writeIndex].descriptorType = VK_DESCRIPTOR_TYPE_SAMPLER;
            descriptorWrite[writeIndex].descriptorCount = 1;
            descriptorWrite[writeIndex].pImageInfo = &textureSamplerInfos[j];
            writeIndex++;
        }

        vkUpdateDescriptorSets(euclid.handle[eh].device, descriptorWriteCount, descriptorWrite, 0, NULL);
        free(descriptorWrite);
    }
    free(textureImageInfos);
    free(textureSamplerInfos);
}

void createPipeline(uint32_t eh, uint32_t eme, uint32_t es, uint32_t em){
    euclid.meshes[eme].modelId = em;
    VkPipelineShaderStageCreateInfo vertShaderStageInfo = {0};
    vertShaderStageInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
    vertShaderStageInfo.stage = VK_SHADER_STAGE_VERTEX_BIT;
    vertShaderStageInfo.module = euclid.materials[es].vertModule;
    vertShaderStageInfo.pName = "main";

    VkPipelineShaderStageCreateInfo fragShaderStageInfo = {0};
    fragShaderStageInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
    fragShaderStageInfo.stage = VK_SHADER_STAGE_FRAGMENT_BIT;
    fragShaderStageInfo.module = euclid.materials[es].fragModule;
    fragShaderStageInfo.pName = "main";

    VkPipelineShaderStageCreateInfo shaderStages[] = {vertShaderStageInfo, fragShaderStageInfo};

    VkDynamicState dynamicStates[] = {
        VK_DYNAMIC_STATE_VIEWPORT,
        VK_DYNAMIC_STATE_SCISSOR
    };
    
    VkPipelineDynamicStateCreateInfo dynamicState = {0};
    dynamicState.sType = VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO;
    dynamicState.dynamicStateCount = 2;
    dynamicState.pDynamicStates = dynamicStates;

    VkVertexInputBindingDescription bindingDescription = {0};
    bindingDescription.binding = 0;
    bindingDescription.stride = 56;
    bindingDescription.inputRate = VK_VERTEX_INPUT_RATE_VERTEX;

    VkVertexInputAttributeDescription attributeDescriptions[5] = {0};

    attributeDescriptions[0].binding = 0;
    attributeDescriptions[0].location = 0;
    attributeDescriptions[0].format = VK_FORMAT_R32G32B32_SFLOAT;
    attributeDescriptions[0].offset = 0;

    attributeDescriptions[1].binding = 0;
    attributeDescriptions[1].location = 1;
    attributeDescriptions[1].format = VK_FORMAT_R32G32_SFLOAT;
    attributeDescriptions[1].offset = 12;

    attributeDescriptions[2].binding = 0;
    attributeDescriptions[2].location = 2;
    attributeDescriptions[2].format = VK_FORMAT_R32G32B32_SFLOAT;
    attributeDescriptions[2].offset = 20;

    attributeDescriptions[3].binding = 0;
    attributeDescriptions[3].location = 3;
    attributeDescriptions[3].format = VK_FORMAT_R32G32B32_SFLOAT;
    attributeDescriptions[3].offset = 32;

    attributeDescriptions[4].binding = 0;
    attributeDescriptions[4].location = 4;
    attributeDescriptions[4].format = VK_FORMAT_R32G32B32_SFLOAT;
    attributeDescriptions[4].offset = 44;

    VkPipelineVertexInputStateCreateInfo vertexInputInfo = {0};
    vertexInputInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
    vertexInputInfo.vertexBindingDescriptionCount = 1;
    vertexInputInfo.vertexAttributeDescriptionCount = 5;
    vertexInputInfo.pVertexBindingDescriptions = &bindingDescription;
    vertexInputInfo.pVertexAttributeDescriptions = attributeDescriptions;

    VkPipelineInputAssemblyStateCreateInfo inputAssembly = {0};
    inputAssembly.sType = VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO;
    inputAssembly.topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST;
    inputAssembly.primitiveRestartEnable = VK_FALSE;

    VkViewport viewport = {0};
    viewport.x = 0.0f;
    viewport.y = 0.0f;
    viewport.width = (float) euclid.handle[eh].resolutionX[0];
    viewport.height = (float) euclid.handle[eh].resolutionY[0];
    viewport.minDepth = 0.0f;
    viewport.maxDepth = 1.0f;

    VkRect2D scissor = {0};
    scissor.offset.x = 0;
    scissor.offset.y = 0;
    scissor.extent.width = euclid.handle[eh].resolutionX[0];
    scissor.extent.height = euclid.handle[eh].resolutionY[0];

    VkPipelineViewportStateCreateInfo viewportState = {0};
    viewportState.sType = VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO;
    viewportState.viewportCount = 1;
    viewportState.pViewports = &viewport;
    viewportState.scissorCount = 1;
    viewportState.pScissors = &scissor;

    VkPipelineRasterizationStateCreateInfo rasterizer = {0};
    rasterizer.sType = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO;
    rasterizer.depthClampEnable = VK_FALSE;
    rasterizer.rasterizerDiscardEnable = VK_FALSE;
    rasterizer.polygonMode = (VkPolygonMode) euclid.materials[es].polygonMode;
    rasterizer.lineWidth = euclid.materials[es].lineWidth;
    rasterizer.cullMode = (VkCullModeFlags) euclid.materials[es].cullMode;
    rasterizer.frontFace = VK_FRONT_FACE_CLOCKWISE;
    rasterizer.depthBiasEnable = VK_FALSE;
    rasterizer.depthBiasConstantFactor = 0.0f;
    rasterizer.depthBiasClamp = 0.0f;
    rasterizer.depthBiasSlopeFactor = 0.0f;

    VkPipelineMultisampleStateCreateInfo multisampling = {0};
    multisampling.sType = VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO;
    multisampling.sampleShadingEnable = VK_FALSE;
    multisampling.rasterizationSamples = VK_SAMPLE_COUNT_1_BIT;
    multisampling.minSampleShading = 1.0f;
    multisampling.pSampleMask = NULL;
    multisampling.alphaToCoverageEnable = VK_FALSE;
    multisampling.alphaToOneEnable = VK_FALSE;

    VkPipelineColorBlendAttachmentState colorBlendAttachment = {0};
    colorBlendAttachment.colorWriteMask = VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT | VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT;
    colorBlendAttachment.blendEnable = VK_FALSE;
    colorBlendAttachment.srcColorBlendFactor = VK_BLEND_FACTOR_ONE;
    colorBlendAttachment.dstColorBlendFactor = VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA;
    colorBlendAttachment.colorBlendOp = VK_BLEND_OP_ADD;
    colorBlendAttachment.srcAlphaBlendFactor = VK_BLEND_FACTOR_ONE;
    colorBlendAttachment.dstAlphaBlendFactor = VK_BLEND_FACTOR_ZERO;
    colorBlendAttachment.alphaBlendOp = VK_BLEND_OP_ADD;

    VkPipelineColorBlendStateCreateInfo colorBlending = {0};
    colorBlending.sType = VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO;
    colorBlending.logicOpEnable = VK_FALSE;
    colorBlending.logicOp = VK_LOGIC_OP_COPY;
    colorBlending.attachmentCount = 1;
    colorBlending.pAttachments = &colorBlendAttachment;
    colorBlending.blendConstants[0] = 0.0f;
    colorBlending.blendConstants[1] = 0.0f;
    colorBlending.blendConstants[2] = 0.0f;
    colorBlending.blendConstants[3] = 0.0f;

    VkPipelineDepthStencilStateCreateInfo depthStencil = {0};
    depthStencil.sType = VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO;
    depthStencil.depthTestEnable = VK_TRUE;
    depthStencil.depthWriteEnable = VK_TRUE;
    depthStencil.depthCompareOp = VK_COMPARE_OP_LESS_OR_EQUAL;
    depthStencil.depthBoundsTestEnable = VK_FALSE;
    depthStencil.minDepthBounds = 0.0f;
    depthStencil.maxDepthBounds = 1.0f;
    depthStencil.stencilTestEnable = VK_FALSE;
    VkStencilOpState dpinfo = {0};
    depthStencil.front = dpinfo;
    depthStencil.back = dpinfo;

    VkPipelineLayoutCreateInfo pipelineLayoutInfo = {0};
    pipelineLayoutInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
    pipelineLayoutInfo.setLayoutCount = 0;
    pipelineLayoutInfo.pSetLayouts = NULL;
    pipelineLayoutInfo.pushConstantRangeCount = 0;
    pipelineLayoutInfo.pPushConstantRanges = NULL;
    pipelineLayoutInfo.setLayoutCount = 1;
    pipelineLayoutInfo.pSetLayouts = &euclid.meshes[eme].descriptorSetLayout;

    VkResult result = vkCreatePipelineLayout(euclid.handle[eh].device, &pipelineLayoutInfo, NULL, &euclid.meshes[eme].pipelineLayout);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Pipeline layout created with result = %d\n", result);

    VkGraphicsPipelineCreateInfo pipelineInfo = {0};
    pipelineInfo.sType = VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO;
    pipelineInfo.stageCount = 2;
    pipelineInfo.pStages = shaderStages;
    pipelineInfo.pVertexInputState = &vertexInputInfo;
    pipelineInfo.pInputAssemblyState = &inputAssembly;
    pipelineInfo.pViewportState = &viewportState;
    pipelineInfo.pRasterizationState = &rasterizer;
    pipelineInfo.pMultisampleState = &multisampling;
    pipelineInfo.pDepthStencilState = &depthStencil;
    pipelineInfo.pColorBlendState = &colorBlending;
    pipelineInfo.pDynamicState = &dynamicState;
    pipelineInfo.layout = euclid.meshes[eme].pipelineLayout;
    pipelineInfo.renderPass = euclid.handle[eh].renderPass;
    pipelineInfo.subpass = 0;
    pipelineInfo.basePipelineHandle = VK_NULL_HANDLE;
    pipelineInfo.basePipelineIndex = -1;

    result = vkCreateGraphicsPipelines(euclid.handle[eh].device, VK_NULL_HANDLE, 1, &pipelineInfo, NULL, &euclid.meshes[eme].graphicsPipeline);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Pipeline created with result = %d\n", result);
}

void createLightingPipeline(uint32_t eh, uint32_t eme, uint32_t es, uint32_t em){
    euclid.meshes[eme].modelId = em;
    VkPipelineShaderStageCreateInfo vertShaderStageInfo = {0};
    vertShaderStageInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
    vertShaderStageInfo.stage = VK_SHADER_STAGE_VERTEX_BIT;
    vertShaderStageInfo.module = euclid.materials[es].vertModule;
    vertShaderStageInfo.pName = "main";

    VkPipelineShaderStageCreateInfo fragShaderStageInfo = {0};
    fragShaderStageInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
    fragShaderStageInfo.stage = VK_SHADER_STAGE_FRAGMENT_BIT;
    fragShaderStageInfo.module = euclid.materials[es].fragModule;
    fragShaderStageInfo.pName = "main";

    VkPipelineShaderStageCreateInfo shaderStages[] = {vertShaderStageInfo, fragShaderStageInfo};

    VkDynamicState dynamicStates[] = {
        VK_DYNAMIC_STATE_VIEWPORT,
        VK_DYNAMIC_STATE_SCISSOR
    };
    
    VkPipelineDynamicStateCreateInfo dynamicState = {0};
    dynamicState.sType = VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO;
    dynamicState.dynamicStateCount = 2;
    dynamicState.pDynamicStates = dynamicStates;

    VkVertexInputBindingDescription bindingDescription = {0};
    bindingDescription.binding = 0;
    bindingDescription.stride = 56;
    bindingDescription.inputRate = VK_VERTEX_INPUT_RATE_VERTEX;

    VkVertexInputAttributeDescription attributeDescriptions[5] = {0};

    attributeDescriptions[0].binding = 0;
    attributeDescriptions[0].location = 0;
    attributeDescriptions[0].format = VK_FORMAT_R32G32B32_SFLOAT;
    attributeDescriptions[0].offset = 0;

    attributeDescriptions[1].binding = 0;
    attributeDescriptions[1].location = 1;
    attributeDescriptions[1].format = VK_FORMAT_R32G32_SFLOAT;
    attributeDescriptions[1].offset = 12;

    attributeDescriptions[2].binding = 0;
    attributeDescriptions[2].location = 2;
    attributeDescriptions[2].format = VK_FORMAT_R32G32B32_SFLOAT;
    attributeDescriptions[2].offset = 20;

    attributeDescriptions[3].binding = 0;
    attributeDescriptions[3].location = 3;
    attributeDescriptions[3].format = VK_FORMAT_R32G32B32_SFLOAT;
    attributeDescriptions[3].offset = 32;

    attributeDescriptions[4].binding = 0;
    attributeDescriptions[4].location = 4;
    attributeDescriptions[4].format = VK_FORMAT_R32G32B32_SFLOAT;
    attributeDescriptions[4].offset = 44;

    VkPipelineVertexInputStateCreateInfo vertexInputInfo = {0};
    vertexInputInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
    vertexInputInfo.vertexBindingDescriptionCount = 1;
    vertexInputInfo.vertexAttributeDescriptionCount = 5;
    vertexInputInfo.pVertexBindingDescriptions = &bindingDescription;
    vertexInputInfo.pVertexAttributeDescriptions = attributeDescriptions;

    VkPipelineInputAssemblyStateCreateInfo inputAssembly = {0};
    inputAssembly.sType = VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO;
    inputAssembly.topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST;
    inputAssembly.primitiveRestartEnable = VK_FALSE;

    VkViewport viewport = {0};
    viewport.x = 0.0f;
    viewport.y = 0.0f;
    viewport.width = (float) euclid.handle[eh].renderResolutionX;
    viewport.height = (float) euclid.handle[eh].renderResolutionY;
    viewport.minDepth = 0.0f;
    viewport.maxDepth = 1.0f;

    VkRect2D scissor = {0};
    scissor.offset.x = 0;
    scissor.offset.y = 0;
    scissor.extent.width = euclid.handle[eh].renderResolutionX;
    scissor.extent.height = euclid.handle[eh].renderResolutionY;

    VkPipelineViewportStateCreateInfo viewportState = {0};
    viewportState.sType = VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO;
    viewportState.viewportCount = 1;
    viewportState.pViewports = &viewport;
    viewportState.scissorCount = 1;
    viewportState.pScissors = &scissor;

    VkPipelineRasterizationStateCreateInfo rasterizer = {0};
    rasterizer.sType = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO;
    rasterizer.depthClampEnable = VK_FALSE;
    rasterizer.rasterizerDiscardEnable = VK_FALSE;
    rasterizer.polygonMode = (VkPolygonMode) euclid.materials[es].polygonMode;
    rasterizer.lineWidth = euclid.materials[es].lineWidth;
    rasterizer.cullMode = (VkCullModeFlags) euclid.materials[es].cullMode;
    rasterizer.frontFace = VK_FRONT_FACE_CLOCKWISE;
    rasterizer.depthBiasEnable = VK_FALSE;
    rasterizer.depthBiasConstantFactor = 0.0f;
    rasterizer.depthBiasClamp = 0.0f;
    rasterizer.depthBiasSlopeFactor = 0.0f;

    VkPipelineMultisampleStateCreateInfo multisampling = {0};
    multisampling.sType = VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO;
    multisampling.sampleShadingEnable = VK_FALSE;
    multisampling.rasterizationSamples = VK_SAMPLE_COUNT_1_BIT;
    multisampling.minSampleShading = 1.0f;
    multisampling.pSampleMask = NULL;
    multisampling.alphaToCoverageEnable = VK_FALSE;
    multisampling.alphaToOneEnable = VK_FALSE;

    VkPipelineColorBlendAttachmentState colorBlendAttachment = {0};
    colorBlendAttachment.colorWriteMask = VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT | VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT;
    colorBlendAttachment.blendEnable = VK_FALSE;
    colorBlendAttachment.srcColorBlendFactor = VK_BLEND_FACTOR_ONE;
    colorBlendAttachment.dstColorBlendFactor = VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA;
    colorBlendAttachment.colorBlendOp = VK_BLEND_OP_ADD;
    colorBlendAttachment.srcAlphaBlendFactor = VK_BLEND_FACTOR_ONE;
    colorBlendAttachment.dstAlphaBlendFactor = VK_BLEND_FACTOR_ZERO;
    colorBlendAttachment.alphaBlendOp = VK_BLEND_OP_ADD;

    VkPipelineColorBlendStateCreateInfo colorBlending = {0};
    colorBlending.sType = VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO;
    colorBlending.logicOpEnable = VK_FALSE;
    colorBlending.logicOp = VK_LOGIC_OP_COPY;
    colorBlending.attachmentCount = 1;
    colorBlending.pAttachments = &colorBlendAttachment;
    colorBlending.blendConstants[0] = 0.0f;
    colorBlending.blendConstants[1] = 0.0f;
    colorBlending.blendConstants[2] = 0.0f;
    colorBlending.blendConstants[3] = 0.0f;

    VkPipelineDepthStencilStateCreateInfo depthStencil = {0};
    depthStencil.sType = VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO;
    depthStencil.depthTestEnable = VK_TRUE;
    depthStencil.depthWriteEnable = VK_TRUE;
    depthStencil.depthCompareOp = VK_COMPARE_OP_LESS_OR_EQUAL;
    depthStencil.depthBoundsTestEnable = VK_FALSE;
    depthStencil.minDepthBounds = 0.0f;
    depthStencil.maxDepthBounds = 1.0f;
    depthStencil.stencilTestEnable = VK_FALSE;
    VkStencilOpState dpinfo = {0};
    depthStencil.front = dpinfo;
    depthStencil.back = dpinfo;

    VkPipelineLayoutCreateInfo pipelineLayoutInfo = {0};
    pipelineLayoutInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
    pipelineLayoutInfo.setLayoutCount = 0;
    pipelineLayoutInfo.pSetLayouts = NULL;
    pipelineLayoutInfo.pushConstantRangeCount = 0;
    pipelineLayoutInfo.pPushConstantRanges = NULL;
    pipelineLayoutInfo.setLayoutCount = 1;
    pipelineLayoutInfo.pSetLayouts = &euclid.meshes[eme].lightingDescriptorSetLayout;

    VkResult result = vkCreatePipelineLayout(euclid.handle[eh].device, &pipelineLayoutInfo, NULL, &euclid.meshes[eme].pipelineLayout);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Lighting pipeline layout created with result = %d\n", result);

    VkGraphicsPipelineCreateInfo pipelineInfo = {0};
    pipelineInfo.sType = VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO;
    pipelineInfo.stageCount = 2;
    pipelineInfo.pStages = shaderStages;
    pipelineInfo.pVertexInputState = &vertexInputInfo;
    pipelineInfo.pInputAssemblyState = &inputAssembly;
    pipelineInfo.pViewportState = &viewportState;
    pipelineInfo.pRasterizationState = &rasterizer;
    pipelineInfo.pMultisampleState = &multisampling;
    pipelineInfo.pDepthStencilState = &depthStencil;
    pipelineInfo.pColorBlendState = &colorBlending;
    pipelineInfo.pDynamicState = &dynamicState;
    pipelineInfo.layout = euclid.meshes[eme].pipelineLayout;
    pipelineInfo.renderPass = euclid.handle[eh].lightingRenderPass;
    pipelineInfo.subpass = 0;
    pipelineInfo.basePipelineHandle = VK_NULL_HANDLE;
    pipelineInfo.basePipelineIndex = -1;

    result = vkCreateGraphicsPipelines(euclid.handle[eh].device, VK_NULL_HANDLE, 1, &pipelineInfo, NULL, &euclid.meshes[eme].graphicsPipeline);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Lighting pipeline created with result = %d\n", result);
}

void createshadowPipeline(uint32_t eh, uint32_t eme, uint32_t es, uint32_t em){
    euclid.meshes[eme].modelId = em;
    VkPipelineShaderStageCreateInfo vertShaderStageInfo = {0};
    vertShaderStageInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
    vertShaderStageInfo.stage = VK_SHADER_STAGE_VERTEX_BIT;
    vertShaderStageInfo.module = euclid.materials[es].shadowModule;
    vertShaderStageInfo.pName = "main";

    VkPipelineShaderStageCreateInfo shaderStages[] = {vertShaderStageInfo};

    VkDynamicState dynamicStates[] = {
        VK_DYNAMIC_STATE_VIEWPORT,
        VK_DYNAMIC_STATE_SCISSOR
    };
    
    VkPipelineDynamicStateCreateInfo dynamicState = {0};
    dynamicState.sType = VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO;
    dynamicState.dynamicStateCount = 2;
    dynamicState.pDynamicStates = dynamicStates;

    VkVertexInputBindingDescription bindingDescription = {0};
    bindingDescription.binding = 0;
    bindingDescription.stride = 56;
    bindingDescription.inputRate = VK_VERTEX_INPUT_RATE_VERTEX;

    VkVertexInputAttributeDescription attributeDescriptions[5] = {0};

    attributeDescriptions[0].binding = 0;
    attributeDescriptions[0].location = 0;
    attributeDescriptions[0].format = VK_FORMAT_R32G32B32_SFLOAT;
    attributeDescriptions[0].offset = 0;

    attributeDescriptions[1].binding = 0;
    attributeDescriptions[1].location = 1;
    attributeDescriptions[1].format = VK_FORMAT_R32G32_SFLOAT;
    attributeDescriptions[1].offset = 12;

    attributeDescriptions[2].binding = 0;
    attributeDescriptions[2].location = 2;
    attributeDescriptions[2].format = VK_FORMAT_R32G32B32_SFLOAT;
    attributeDescriptions[2].offset = 20;

    attributeDescriptions[3].binding = 0;
    attributeDescriptions[3].location = 3;
    attributeDescriptions[3].format = VK_FORMAT_R32G32B32_SFLOAT;
    attributeDescriptions[3].offset = 32;

    attributeDescriptions[4].binding = 0;
    attributeDescriptions[4].location = 4;
    attributeDescriptions[4].format = VK_FORMAT_R32G32B32_SFLOAT;
    attributeDescriptions[4].offset = 44;

    VkPipelineVertexInputStateCreateInfo vertexInputInfo = {0};
    vertexInputInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
    vertexInputInfo.vertexBindingDescriptionCount = 1;
    vertexInputInfo.vertexAttributeDescriptionCount = 5;
    vertexInputInfo.pVertexBindingDescriptions = &bindingDescription;
    vertexInputInfo.pVertexAttributeDescriptions = attributeDescriptions;

    VkPipelineInputAssemblyStateCreateInfo inputAssembly = {0};
    inputAssembly.sType = VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO;
    inputAssembly.topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST;
    inputAssembly.primitiveRestartEnable = VK_FALSE;

    VkViewport viewport = {0};
    viewport.x = 0.0f;
    viewport.y = 0.0f;
    viewport.width = (float) euclid.handle[eh].shadowMapResolution[0];
    viewport.height = (float) euclid.handle[eh].shadowMapResolution[0];
    viewport.minDepth = 0.0f;
    viewport.maxDepth = 1.0f;

    VkRect2D scissor = {0};
    scissor.offset.x = 0;
    scissor.offset.y = 0;
    scissor.extent.width = euclid.handle[eh].shadowMapResolution[0];
    scissor.extent.height = euclid.handle[eh].shadowMapResolution[0];

    VkPipelineViewportStateCreateInfo viewportState = {0};
    viewportState.sType = VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO;
    viewportState.viewportCount = 1;
    viewportState.pViewports = &viewport;
    viewportState.scissorCount = 1;
    viewportState.pScissors = &scissor;

    VkPipelineRasterizationStateCreateInfo rasterizer = {0};
    rasterizer.sType = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO;
    rasterizer.depthClampEnable = VK_FALSE;
    rasterizer.rasterizerDiscardEnable = VK_FALSE;
    rasterizer.polygonMode = (VkPolygonMode) euclid.materials[es].polygonMode;
    rasterizer.lineWidth = euclid.materials[es].lineWidth;
    rasterizer.cullMode = (VkCullModeFlags) euclid.materials[es].shcullMode;
    rasterizer.frontFace = VK_FRONT_FACE_CLOCKWISE;
    rasterizer.depthBiasEnable = VK_FALSE;
    rasterizer.depthBiasConstantFactor = 0.0f;
    rasterizer.depthBiasClamp = 0.0f;
    rasterizer.depthBiasSlopeFactor = 0.0f;

    VkPipelineMultisampleStateCreateInfo multisampling = {0};
    multisampling.sType = VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO;
    multisampling.sampleShadingEnable = VK_FALSE;
    multisampling.rasterizationSamples = VK_SAMPLE_COUNT_1_BIT;
    multisampling.minSampleShading = 1.0f;
    multisampling.pSampleMask = NULL;
    multisampling.alphaToCoverageEnable = VK_FALSE;
    multisampling.alphaToOneEnable = VK_FALSE;

    VkPipelineColorBlendStateCreateInfo colorBlending = {0};
    colorBlending.sType = VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO;
    colorBlending.logicOpEnable = VK_FALSE;
    colorBlending.logicOp = VK_LOGIC_OP_COPY;
    colorBlending.attachmentCount = 0;
    colorBlending.pAttachments = NULL;
    colorBlending.blendConstants[0] = 0.0f;
    colorBlending.blendConstants[1] = 0.0f;
    colorBlending.blendConstants[2] = 0.0f;
    colorBlending.blendConstants[3] = 0.0f;

    VkPipelineDepthStencilStateCreateInfo depthStencil = {0};
    depthStencil.sType = VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO;
    depthStencil.depthTestEnable = VK_TRUE;
    depthStencil.depthWriteEnable = VK_TRUE;
    depthStencil.depthCompareOp = VK_COMPARE_OP_LESS_OR_EQUAL;
    depthStencil.depthBoundsTestEnable = VK_FALSE;
    depthStencil.minDepthBounds = 0.0f;
    depthStencil.maxDepthBounds = 1.0f;
    depthStencil.stencilTestEnable = VK_FALSE;
    VkStencilOpState dpinfo = {0};
    depthStencil.front = dpinfo;
    depthStencil.back = dpinfo;

    VkPipelineLayoutCreateInfo pipelineLayoutInfo = {0};
    pipelineLayoutInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
    pipelineLayoutInfo.setLayoutCount = 0;
    pipelineLayoutInfo.pSetLayouts = NULL;
    pipelineLayoutInfo.pushConstantRangeCount = 0;
    pipelineLayoutInfo.pPushConstantRanges = NULL;
    pipelineLayoutInfo.setLayoutCount = 1;
    pipelineLayoutInfo.pSetLayouts = &euclid.meshes[eme].shadowDescriptorSetLayout;

    VkResult result = vkCreatePipelineLayout(euclid.handle[eh].device, &pipelineLayoutInfo, NULL, &euclid.meshes[eme].shadowPipelineLayout);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Shadow pipeline layout created with result = %d\n", result);

    VkGraphicsPipelineCreateInfo pipelineInfo = {0};
    pipelineInfo.sType = VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO;
    pipelineInfo.stageCount = 1;
    pipelineInfo.pStages = shaderStages;
    pipelineInfo.pVertexInputState = &vertexInputInfo;
    pipelineInfo.pInputAssemblyState = &inputAssembly;
    pipelineInfo.pViewportState = &viewportState;
    pipelineInfo.pRasterizationState = &rasterizer;
    pipelineInfo.pMultisampleState = &multisampling;
    pipelineInfo.pDepthStencilState = &depthStencil;
    pipelineInfo.pColorBlendState = &colorBlending;
    pipelineInfo.pDynamicState = &dynamicState;
    pipelineInfo.layout = euclid.meshes[eme].shadowPipelineLayout;
    pipelineInfo.renderPass = euclid.handle[eh].shadowRenderPass;
    pipelineInfo.subpass = 0;
    pipelineInfo.basePipelineHandle = VK_NULL_HANDLE;
    pipelineInfo.basePipelineIndex = -1;

    result = vkCreateGraphicsPipelines(euclid.handle[eh].device, VK_NULL_HANDLE, 1, &pipelineInfo, NULL, &euclid.meshes[eme].shadowPipeline);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Shadow pipeline created with result = %d\n", result);
}

void createdefferedPipeline(uint32_t eh, uint32_t eme, uint32_t es, uint32_t em){
    euclid.meshes[eme].modelId = em;
    VkPipelineShaderStageCreateInfo vertShaderStageInfo = {0};
    vertShaderStageInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
    vertShaderStageInfo.stage = VK_SHADER_STAGE_VERTEX_BIT;
    vertShaderStageInfo.module = euclid.materials[es].vertModule;
    vertShaderStageInfo.pName = "main";

    VkPipelineShaderStageCreateInfo fragShaderStageInfo = {0};
    fragShaderStageInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
    fragShaderStageInfo.stage = VK_SHADER_STAGE_FRAGMENT_BIT;
    fragShaderStageInfo.module = euclid.materials[es].fragModule;
    fragShaderStageInfo.pName = "main";

    VkPipelineShaderStageCreateInfo shaderStages[] = {vertShaderStageInfo, fragShaderStageInfo};

    VkDynamicState dynamicStates[] = {
        VK_DYNAMIC_STATE_VIEWPORT,
        VK_DYNAMIC_STATE_SCISSOR
    };
    
    VkPipelineDynamicStateCreateInfo dynamicState = {0};
    dynamicState.sType = VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO;
    dynamicState.dynamicStateCount = 2;
    dynamicState.pDynamicStates = dynamicStates;

    VkVertexInputBindingDescription bindingDescription = {0};
    bindingDescription.binding = 0;
    bindingDescription.stride = 56;
    bindingDescription.inputRate = VK_VERTEX_INPUT_RATE_VERTEX;

    VkVertexInputAttributeDescription attributeDescriptions[5] = {0};

    attributeDescriptions[0].binding = 0;
    attributeDescriptions[0].location = 0;
    attributeDescriptions[0].format = VK_FORMAT_R32G32B32_SFLOAT;
    attributeDescriptions[0].offset = 0;

    attributeDescriptions[1].binding = 0;
    attributeDescriptions[1].location = 1;
    attributeDescriptions[1].format = VK_FORMAT_R32G32_SFLOAT;
    attributeDescriptions[1].offset = 12;

    attributeDescriptions[2].binding = 0;
    attributeDescriptions[2].location = 2;
    attributeDescriptions[2].format = VK_FORMAT_R32G32B32_SFLOAT;
    attributeDescriptions[2].offset = 20;

    attributeDescriptions[3].binding = 0;
    attributeDescriptions[3].location = 3;
    attributeDescriptions[3].format = VK_FORMAT_R32G32B32_SFLOAT;
    attributeDescriptions[3].offset = 32;

    attributeDescriptions[4].binding = 0;
    attributeDescriptions[4].location = 4;
    attributeDescriptions[4].format = VK_FORMAT_R32G32B32_SFLOAT;
    attributeDescriptions[4].offset = 44;

    VkPipelineVertexInputStateCreateInfo vertexInputInfo = {0};
    vertexInputInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
    vertexInputInfo.vertexBindingDescriptionCount = 1;
    vertexInputInfo.vertexAttributeDescriptionCount = 5;
    vertexInputInfo.pVertexBindingDescriptions = &bindingDescription;
    vertexInputInfo.pVertexAttributeDescriptions = attributeDescriptions;

    VkPipelineInputAssemblyStateCreateInfo inputAssembly = {0};
    inputAssembly.sType = VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO;
    inputAssembly.topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST;
    inputAssembly.primitiveRestartEnable = VK_FALSE;

    VkViewport viewport = {0};
    viewport.x = 0.0f;
    viewport.y = 0.0f;
    viewport.width = (float) euclid.handle[eh].renderResolutionX;
    viewport.height = (float) euclid.handle[eh].renderResolutionY;
    viewport.minDepth = 0.0f;
    viewport.maxDepth = 1.0f;

    VkRect2D scissor = {0};
    scissor.offset.x = 0;
    scissor.offset.y = 0;
    scissor.extent.width = euclid.handle[eh].renderResolutionX;
    scissor.extent.height = euclid.handle[eh].renderResolutionY;

    VkPipelineViewportStateCreateInfo viewportState = {0};
    viewportState.sType = VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO;
    viewportState.viewportCount = 1;
    viewportState.pViewports = &viewport;
    viewportState.scissorCount = 1;
    viewportState.pScissors = &scissor;

    VkPipelineRasterizationStateCreateInfo rasterizer = {0};
    rasterizer.sType = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO;
    rasterizer.depthClampEnable = VK_FALSE;
    rasterizer.rasterizerDiscardEnable = VK_FALSE;
    rasterizer.polygonMode = (VkPolygonMode) euclid.materials[es].polygonMode;
    rasterizer.lineWidth = euclid.materials[es].lineWidth;
    rasterizer.cullMode = (VkCullModeFlags) euclid.materials[es].cullMode;
    rasterizer.frontFace = VK_FRONT_FACE_CLOCKWISE;
    rasterizer.depthBiasEnable = VK_FALSE;
    rasterizer.depthBiasConstantFactor = 0.0f;
    rasterizer.depthBiasClamp = 0.0f;
    rasterizer.depthBiasSlopeFactor = 0.0f;

    VkPipelineMultisampleStateCreateInfo multisampling = {0};
    multisampling.sType = VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO;
    multisampling.sampleShadingEnable = VK_FALSE;
    multisampling.rasterizationSamples = VK_SAMPLE_COUNT_1_BIT;
    multisampling.minSampleShading = 1.0f;
    multisampling.pSampleMask = NULL;
    multisampling.alphaToCoverageEnable = VK_FALSE;
    multisampling.alphaToOneEnable = VK_FALSE;

    VkPipelineColorBlendAttachmentState colorBlendAttachment[4] = {0};
    for(uint32_t i = 0; i != 3; i++){
        colorBlendAttachment[i].colorWriteMask = VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT | VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT;
        colorBlendAttachment[i].blendEnable = VK_FALSE;
        colorBlendAttachment[i].srcColorBlendFactor = VK_BLEND_FACTOR_ONE;
        colorBlendAttachment[i].dstColorBlendFactor = VK_BLEND_FACTOR_ZERO;
        colorBlendAttachment[i].colorBlendOp = VK_BLEND_OP_ADD;
        colorBlendAttachment[i].srcAlphaBlendFactor = VK_BLEND_FACTOR_ONE;
        colorBlendAttachment[i].dstAlphaBlendFactor = VK_BLEND_FACTOR_ZERO;
        colorBlendAttachment[i].alphaBlendOp = VK_BLEND_OP_ADD;
    }

    VkPipelineColorBlendStateCreateInfo colorBlending = {0};
    colorBlending.sType = VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO;
    colorBlending.logicOpEnable = VK_FALSE;
    colorBlending.logicOp = VK_LOGIC_OP_COPY;
    colorBlending.attachmentCount = 4;
    colorBlending.pAttachments = colorBlendAttachment;
    colorBlending.blendConstants[0] = 0.0f;
    colorBlending.blendConstants[1] = 0.0f;
    colorBlending.blendConstants[2] = 0.0f;
    colorBlending.blendConstants[3] = 0.0f;

    VkPipelineDepthStencilStateCreateInfo depthStencil = {0};
    depthStencil.sType = VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO;
    depthStencil.depthTestEnable = VK_TRUE;
    depthStencil.depthWriteEnable = VK_TRUE;
    depthStencil.depthCompareOp = VK_COMPARE_OP_LESS_OR_EQUAL;
    depthStencil.depthBoundsTestEnable = VK_FALSE;
    depthStencil.minDepthBounds = 0.0f;
    depthStencil.maxDepthBounds = 1.0f;
    depthStencil.stencilTestEnable = VK_FALSE;
    VkStencilOpState dpinfo = {0};
    depthStencil.front = dpinfo;
    depthStencil.back = dpinfo;

    VkPipelineLayoutCreateInfo pipelineLayoutInfo = {0};
    pipelineLayoutInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
    pipelineLayoutInfo.setLayoutCount = 0;
    pipelineLayoutInfo.pSetLayouts = NULL;
    pipelineLayoutInfo.pushConstantRangeCount = 0;
    pipelineLayoutInfo.pPushConstantRanges = NULL;
    pipelineLayoutInfo.setLayoutCount = 1;
    pipelineLayoutInfo.pSetLayouts = &euclid.meshes[eme].defferedDescriptorSetLayout;

    VkResult result = vkCreatePipelineLayout(euclid.handle[eh].device, &pipelineLayoutInfo, NULL, &euclid.meshes[eme].defferedPipelineLayout);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Deffered pipeline layout created with result = %d\n", result);

    VkGraphicsPipelineCreateInfo pipelineInfo = {0};
    pipelineInfo.sType = VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO;
    pipelineInfo.stageCount = 2;
    pipelineInfo.pStages = shaderStages;
    pipelineInfo.pVertexInputState = &vertexInputInfo;
    pipelineInfo.pInputAssemblyState = &inputAssembly;
    pipelineInfo.pViewportState = &viewportState;
    pipelineInfo.pRasterizationState = &rasterizer;
    pipelineInfo.pMultisampleState = &multisampling;
    pipelineInfo.pDepthStencilState = &depthStencil;
    pipelineInfo.pColorBlendState = &colorBlending;
    pipelineInfo.pDynamicState = &dynamicState;
    pipelineInfo.layout = euclid.meshes[eme].defferedPipelineLayout;
    pipelineInfo.renderPass = euclid.handle[eh].defferedRenderPass;
    pipelineInfo.subpass = 0;
    pipelineInfo.basePipelineHandle = VK_NULL_HANDLE;
    pipelineInfo.basePipelineIndex = -1;

    result = vkCreateGraphicsPipelines(euclid.handle[eh].device, VK_NULL_HANDLE, 1, &pipelineInfo, NULL, &euclid.meshes[eme].defferedPipeline);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Deffered pipeline created with result = %d\n", result);
}

static uint32_t newmesh_real(uint32_t eh, uint32_t eme, euclidCreationQueue *item){
    euclid.meshes[eme].drawable = 1;
    euclid.meshes[eme].uniformslot = 0;
    euclid.meshes[eme].euclidid = eh;
    euclid.meshes[eme].camrend = -1;
    euclid.meshes[eme].paramslot = 0;
    euclid.meshes[eme].usage = item->usage;
    euclid.meshes[eme].mrec = euclid.handle[eh].mrec;

    euclid.meshes[eme].savpapparam[0] = item->es;
    euclid.meshes[eme].savpapparam[1] = item->em;

    euclid.meshes[eme].savedtex = malloc(sizeof(uint32_t) * item->tn);
    euclid.meshes[eme].texnm = item->tn;
    if (item->tn > 0) {
        memcpy(euclid.meshes[eme].savedtex, item->te, sizeof(uint32_t) * item->tn);
    }

    createUniformBuffer(eh, eme);

    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidMS\e[0;37m: Mesh usage = %i, used material = %i, used model = %i\n", item->usage, item->es, item->em);

    if(item->usage == 0){
        createDescriptorPool(eh, eme);
        createDescriptorSetLayout(eh, eme);
        createDescriptorSets(eh, eme);
        createPipeline(eh, eme, item->es, item->em);
    }
    if(item->usage == 2 || item->usage == 3){
        createShadowDescriptorPool(eh, eme);
        createShadowDescriptorSetLayout(eh, eme);
        createShadowDescriptorSets(eh, eme);
        createshadowPipeline(eh, eme, item->es, item->em);
    }
    if(item->usage == 1 || item->usage == 3){
        createDeferredDescriptorPool(eh, eme);
        createDefferedDescriptorSetLayout(eh, eme);
        createDefferedDescriptorSets(eh, eme);
        createdefferedPipeline(eh, eme, item->es, item->em);
    }
    if(item->usage == 4){
        createLightingDescriptorPool(eh, eme);
        createLightingDescriptorSetLayout(eh, eme);
        createLightingDescriptorSets(eh, eme);
        createLightingPipeline(eh, eme, item->es, item->em);
    }
    return eme;
}

uint32_t newmesh(uint32_t eh, uint32_t es, uint32_t em, uint32_t *te, uint32_t tn, uint32_t usage){
    uint32_t eme = euclid.vmesize;
    euclid.vmesize++;

    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidOQ\e[0;37m: New mesh is being pushed in queue creation, index = %i\n", eme);

    euclidCreationQueue item = {0};
    item.type = EUCLID_QUEUE_TYPE_MESH;
    item.meshid = eme;
    item.es = es;
    item.em = em;
    item.tn = tn;
    item.usage = usage;
    if(tn > 0){
        item.te = malloc(sizeof(uint32_t) * tn);
        memcpy(item.te, te, sizeof(uint32_t) * tn);
    }

    push_creation_queue(eh, &item);

    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidOQ\e[0;37m: New mesh pushed in queue creation, index = %i\n", eme);

    return eme;
}

void setrendercamera(uint32_t eme, int8_t val){
    if(eme >= euclid.mesize){
        return;
    }
    euclid.meshes[eme].camrend = val;
}

void setmeshbuf(uint32_t eme, uint32_t i, float val){
    if(eme >= euclid.mesize){
        return;
    }
    if(i + 8 < 64){
        euclid.meshes[eme].lub[euclid.meshes[eme].uniformslot][i+8] = val;
    }
}

void setdrawable(uint32_t eme, uint8_t val){
    if(eme >= euclid.mesize){
        return;
    }
    euclid.meshes[eme].drawable = val;
}

VkCommandBuffer beginSingleTimeCommands(uint32_t eh) {
    VkCommandBufferAllocateInfo allocInfo = {0};
    allocInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
    allocInfo.level = VK_COMMAND_BUFFER_LEVEL_PRIMARY;
    allocInfo.commandPool = euclid.handle[eh].commandPool[4];
    allocInfo.commandBufferCount = 1;

    VkCommandBuffer commandBuffer;
    vkAllocateCommandBuffers(euclid.handle[eh].device, &allocInfo, &commandBuffer);

    VkCommandBufferBeginInfo beginInfo = {0};
    beginInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
    beginInfo.flags = VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT;

    vkBeginCommandBuffer(commandBuffer, &beginInfo);

    return commandBuffer;
}

void endSingleTimeCommands(uint32_t eh, VkCommandBuffer commandBuffer) {
    vkEndCommandBuffer(commandBuffer);

    VkSubmitInfo submitInfo = {0};
    submitInfo.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO;
    submitInfo.commandBufferCount = 1;
    submitInfo.pCommandBuffers = &commandBuffer;

    vkQueueSubmit(euclid.handle[eh].graphicsQueue, 1, &submitInfo, VK_NULL_HANDLE);
    vkQueueWaitIdle(euclid.handle[eh].graphicsQueue);

    vkFreeCommandBuffers(euclid.handle[eh].device, euclid.handle[eh].commandPool[4], 1, &commandBuffer);
}

void generateMipmaps(VkImage image, int32_t texWidth, int32_t texHeight, uint32_t mipLevels, uint32_t layercnt, uint32_t eh) {
    VkCommandBuffer commandBuffer = beginSingleTimeCommands(eh);

    VkImageMemoryBarrier barrier = {0};
    barrier.sType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER;
    barrier.image = image;
    barrier.srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
    barrier.dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
    barrier.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
    barrier.subresourceRange.baseArrayLayer = 0;
    barrier.subresourceRange.layerCount = layercnt;
    barrier.subresourceRange.levelCount = 1;

    int32_t mipWidth = texWidth;
    int32_t mipHeight = texHeight;

    for (uint32_t i = 1; i < mipLevels; i++) {
        barrier.subresourceRange.baseMipLevel = i - 1;
        barrier.oldLayout = VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL;
        barrier.newLayout = VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL;
        barrier.srcAccessMask = VK_ACCESS_TRANSFER_WRITE_BIT;
        barrier.dstAccessMask = VK_ACCESS_TRANSFER_READ_BIT;

        vkCmdPipelineBarrier(commandBuffer,
        VK_PIPELINE_STAGE_TRANSFER_BIT, VK_PIPELINE_STAGE_TRANSFER_BIT, 0,
        0, NULL,
        0, NULL,
        1, &barrier);

        for(uint32_t j = 0; j != layercnt; j++){
            VkImageBlit blit = {0};
            blit.srcOffsets[0].x = 0;
            blit.srcOffsets[0].y = 0;
            blit.srcOffsets[0].z = 0;
            blit.srcOffsets[1].x = mipWidth;
            blit.srcOffsets[1].y = mipHeight;
            blit.srcOffsets[1].z = 1;
            blit.srcSubresource.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
            blit.srcSubresource.mipLevel = i - 1;
            blit.srcSubresource.baseArrayLayer = j;
            blit.srcSubresource.layerCount = 1;
            blit.dstOffsets[0].x = 0;
            blit.dstOffsets[0].y = 0;
            blit.dstOffsets[0].z = 0;
            blit.dstOffsets[1].x = mipWidth > 1 ? mipWidth / 2 : 1;
            blit.dstOffsets[1].y = mipHeight > 1 ? mipHeight / 2 : 1;
            blit.dstOffsets[1].z = 1;
            blit.dstSubresource.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
            blit.dstSubresource.mipLevel = i;
            blit.dstSubresource.baseArrayLayer = j;
            blit.dstSubresource.layerCount = 1;

            vkCmdBlitImage(commandBuffer,
            image, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
            image, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
            1, &blit,
            VK_FILTER_LINEAR);
        }

        barrier.oldLayout = VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL;
        barrier.newLayout = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
        barrier.srcAccessMask = VK_ACCESS_TRANSFER_READ_BIT;
        barrier.dstAccessMask = VK_ACCESS_SHADER_READ_BIT;

        vkCmdPipelineBarrier(commandBuffer,
        VK_PIPELINE_STAGE_TRANSFER_BIT, VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT, 0,
        0, NULL,
        0, NULL,
        1, &barrier);

        if (mipWidth > 1) mipWidth /= 2;
        if (mipHeight > 1) mipHeight /= 2;
    }

    barrier.subresourceRange.baseMipLevel = mipLevels - 1;
    barrier.oldLayout = VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL;
    barrier.newLayout = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
    barrier.srcAccessMask = VK_ACCESS_TRANSFER_WRITE_BIT;
    barrier.dstAccessMask = VK_ACCESS_SHADER_READ_BIT;

    vkCmdPipelineBarrier(commandBuffer,
    VK_PIPELINE_STAGE_TRANSFER_BIT, VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT, 0,
    0, NULL,
    0, NULL,
    1, &barrier);

    endSingleTimeCommands(eh, commandBuffer);
}

void generateMipmaps3D(VkImage image, int32_t texWidth, int32_t texHeight, uint32_t texDepth, uint32_t mipLevels, uint32_t eh) {
    VkCommandBuffer commandBuffer = beginSingleTimeCommands(eh);

    VkImageMemoryBarrier barrier = {0};
    barrier.sType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER;
    barrier.image = image;
    barrier.srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
    barrier.dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
    barrier.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
    barrier.subresourceRange.baseArrayLayer = 0;
    barrier.subresourceRange.layerCount = 1;
    barrier.subresourceRange.levelCount = 1;

    int32_t mipWidth = texWidth;
    int32_t mipHeight = texHeight;
    int32_t mipDepth = texDepth;

    for (uint32_t i = 1; i < mipLevels; i++) {
        barrier.subresourceRange.baseMipLevel = i - 1;
        barrier.oldLayout = VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL;
        barrier.newLayout = VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL;
        barrier.srcAccessMask = VK_ACCESS_TRANSFER_WRITE_BIT;
        barrier.dstAccessMask = VK_ACCESS_TRANSFER_READ_BIT;

        vkCmdPipelineBarrier(commandBuffer,
        VK_PIPELINE_STAGE_TRANSFER_BIT, VK_PIPELINE_STAGE_TRANSFER_BIT, 0,
        0, NULL,
        0, NULL,
        1, &barrier);

        VkImageBlit blit = {0};
        blit.srcOffsets[0].x = 0;
        blit.srcOffsets[0].y = 0;
        blit.srcOffsets[0].z = 0;
        blit.srcOffsets[1].x = mipWidth;
        blit.srcOffsets[1].y = mipHeight;
        blit.srcOffsets[1].z = mipDepth;
        blit.srcSubresource.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
        blit.srcSubresource.mipLevel = i - 1;
        blit.srcSubresource.baseArrayLayer = 0;
        blit.srcSubresource.layerCount = 1;
        blit.dstOffsets[0].x = 0;
        blit.dstOffsets[0].y = 0;
        blit.dstOffsets[0].z = 0;
        blit.dstOffsets[1].x = mipWidth > 1 ? mipWidth / 2 : 1;
        blit.dstOffsets[1].y = mipHeight > 1 ? mipHeight / 2 : 1;
        blit.dstOffsets[1].z = mipDepth > 1 ? mipDepth / 2 : 1;
        blit.dstSubresource.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
        blit.dstSubresource.mipLevel = i;
        blit.dstSubresource.baseArrayLayer = 0;
        blit.dstSubresource.layerCount = 1;

        vkCmdBlitImage(commandBuffer,
        image, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
        image, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
        1, &blit,
        VK_FILTER_LINEAR);

        barrier.oldLayout = VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL;
        barrier.newLayout = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
        barrier.srcAccessMask = VK_ACCESS_TRANSFER_READ_BIT;
        barrier.dstAccessMask = VK_ACCESS_SHADER_READ_BIT;

        vkCmdPipelineBarrier(commandBuffer,
        VK_PIPELINE_STAGE_TRANSFER_BIT, VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT, 0,
        0, NULL,
        0, NULL,
        1, &barrier);

        if (mipWidth > 1) mipWidth /= 2;
        if (mipHeight > 1) mipHeight /= 2;
        if (mipDepth > 1) mipDepth /= 2;
    }

    barrier.subresourceRange.baseMipLevel = mipLevels - 1;
    barrier.oldLayout = VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL;
    barrier.newLayout = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
    barrier.srcAccessMask = VK_ACCESS_TRANSFER_WRITE_BIT;
    barrier.dstAccessMask = VK_ACCESS_SHADER_READ_BIT;

    vkCmdPipelineBarrier(commandBuffer,
    VK_PIPELINE_STAGE_TRANSFER_BIT, VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT, 0,
    0, NULL,
    0, NULL,
    1, &barrier);

    endSingleTimeCommands(eh, commandBuffer);
}

static uint32_t newtexture_real(uint32_t eh, uint32_t te, euclidCreationQueue *item){
    VkBuffer stagingBuffer;
    VkDeviceMemory stagingBufferMemory;

    VkDeviceSize imageSize = item->xsize * item->ysize * item->zsize * item->byteperpixel;
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidTEX\e[0;37m: passed image size = %dx%dx%d, total size number = %d\n", item->xsize, item->ysize, item->zsize, imageSize);

    VkBufferCreateInfo bufferInfo = {0};
    bufferInfo.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
    bufferInfo.size = imageSize;
    bufferInfo.usage = VK_BUFFER_USAGE_TRANSFER_SRC_BIT;
    bufferInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
    VkResult result = vkCreateBuffer(euclid.handle[eh].device, &bufferInfo, NULL, &stagingBuffer);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidTEX\e[0;37m: Staging buffer created with result = %d\n", result);
    VkMemoryRequirements memRequirements;
    vkGetBufferMemoryRequirements(euclid.handle[eh].device, stagingBuffer, &memRequirements);
    VkMemoryAllocateInfo allocInfo = {0};
    allocInfo.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    allocInfo.allocationSize = memRequirements.size;
    allocInfo.memoryTypeIndex = findMemoryType(memRequirements.memoryTypeBits, VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT, eh);
    vkAllocateMemory(euclid.handle[eh].device, &allocInfo, NULL, &stagingBufferMemory);

    void* data;
    vkMapMemory(euclid.handle[eh].device, stagingBufferMemory, 0, imageSize, 0, &data);
        memcpy(data, item->pixels, imageSize);
    vkUnmapMemory(euclid.handle[eh].device, stagingBufferMemory);

    vkBindBufferMemory(euclid.handle[eh].device, stagingBuffer, stagingBufferMemory, 0);
    
    euclid.textures[te].mipLevels = 1;
    if(item->genmips == 1){
        euclid.textures[te].mipLevels = floor(log2(fmaxf(item->xsize, item->ysize)))+1;
        if(item->is3d == 1){
            euclid.textures[te].mipLevels = floor(log2(fmax(fmax(item->xsize, item->ysize), item->zsize)))+1;
        }
    }

    VkImageCreateInfo imageInfo = {0};
    imageInfo.sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
    imageInfo.imageType = VK_IMAGE_TYPE_2D;
    imageInfo.extent.width = item->xsize;
    imageInfo.extent.height = item->ysize;
    imageInfo.extent.depth = 1;
    imageInfo.mipLevels = euclid.textures[te].mipLevels;
    imageInfo.arrayLayers = item->zsize;
    imageInfo.format = item->imageformat;
    imageInfo.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    imageInfo.usage = VK_IMAGE_USAGE_TRANSFER_DST_BIT | VK_IMAGE_USAGE_SAMPLED_BIT;
    imageInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
    imageInfo.samples = VK_SAMPLE_COUNT_1_BIT;
    imageInfo.flags = 0;
    if(item->is3d == 1){
        imageInfo.imageType = VK_IMAGE_TYPE_3D;
        imageInfo.extent.depth = item->zsize;
        imageInfo.arrayLayers = 1;
    }
    result = vkCreateImage(euclid.handle[eh].device, &imageInfo, NULL, &euclid.textures[te].texture);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidTEX\e[0;37m: Texture created with result = %d\n", result);

    VkMemoryRequirements memRequirementsi;
    vkGetImageMemoryRequirements(euclid.handle[eh].device, euclid.textures[te].texture, &memRequirementsi);
    
    VkMemoryAllocateInfo allocInfoi = {0};
    allocInfoi.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    allocInfoi.allocationSize = memRequirementsi.size;
    allocInfoi.memoryTypeIndex = findMemoryType(memRequirementsi.memoryTypeBits, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT, eh);
    
    result = vkAllocateMemory(euclid.handle[eh].device, &allocInfoi, NULL, &euclid.textures[te].textureImageMemory);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidTEX\e[0;37m: Texture memory alocated with result = %d\n", result);

    vkBindImageMemory(euclid.handle[eh].device, euclid.textures[te].texture, euclid.textures[te].textureImageMemory, 0);

    VkImageMemoryBarrier barrier = {0};
    barrier.sType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER;
    barrier.oldLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    barrier.newLayout = VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL;
    barrier.srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
    barrier.dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
    barrier.image = euclid.textures[te].texture;
    barrier.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
    barrier.subresourceRange.baseMipLevel = 0;
    barrier.subresourceRange.levelCount = euclid.textures[te].mipLevels;
    barrier.subresourceRange.baseArrayLayer = 0;
    barrier.subresourceRange.layerCount = item->zsize;
    if(item->is3d == 1){
        barrier.subresourceRange.layerCount = 1;
    }
    barrier.srcAccessMask = 0;
    barrier.dstAccessMask = VK_ACCESS_TRANSFER_WRITE_BIT;

    VkCommandBuffer commandBuffer = beginSingleTimeCommands(eh);

    vkCmdPipelineBarrier(
        commandBuffer,
        VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT, VK_PIPELINE_STAGE_TRANSFER_BIT,
        0,
        0, NULL,
        0, NULL,
        1, &barrier
    );

    endSingleTimeCommands(eh, commandBuffer);
    
    VkBufferImageCopy region = {0};
    region.bufferOffset = 0;
    region.bufferRowLength = 0;
    region.bufferImageHeight = 0;

    region.imageSubresource.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
    region.imageSubresource.mipLevel = 0;
    region.imageSubresource.baseArrayLayer = 0;
    region.imageSubresource.layerCount = item->zsize;

    region.imageOffset.x = 0;
    region.imageOffset.y = 0;
    region.imageOffset.z = 0;
    region.imageExtent.width = item->xsize;
    region.imageExtent.height = item->ysize;
    region.imageExtent.depth = 1;

    if(item->is3d == 1){
        region.imageSubresource.layerCount = 1;
        region.imageExtent.depth = item->zsize;
    }

    commandBuffer = beginSingleTimeCommands(eh);
    
    vkCmdCopyBufferToImage(
        commandBuffer,
        stagingBuffer,
        euclid.textures[te].texture,
        VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
        1,
        &region
    );

    endSingleTimeCommands(eh, commandBuffer);

    vkDestroyBuffer(euclid.handle[eh].device, stagingBuffer, NULL);
    vkFreeMemory(euclid.handle[eh].device, stagingBufferMemory, NULL);

    if(item->genmips == 1){
        if(item->is3d == 1){
            generateMipmaps3D(euclid.textures[te].texture, item->xsize, item->ysize, item->zsize, euclid.textures[te].mipLevels, eh);
        }else{
            generateMipmaps(euclid.textures[te].texture, item->xsize, item->ysize, euclid.textures[te].mipLevels, item->zsize, eh);
        }
    }

    VkImageViewCreateInfo viewInfo = {0};
    viewInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
    viewInfo.image = euclid.textures[te].texture;
    viewInfo.viewType = VK_IMAGE_VIEW_TYPE_2D_ARRAY;
    if(item->is3d == 1){
        viewInfo.viewType = VK_IMAGE_VIEW_TYPE_3D;
    }
    viewInfo.format = item->imageformat;
    viewInfo.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
    viewInfo.subresourceRange.baseMipLevel = 0;
    viewInfo.subresourceRange.levelCount = euclid.textures[te].mipLevels;
    viewInfo.subresourceRange.baseArrayLayer = 0;
    viewInfo.subresourceRange.layerCount = item->zsize;

    result = vkCreateImageView(euclid.handle[eh].device, &viewInfo, NULL, &euclid.textures[te].textureImageView);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidTEX\e[0;37m: Texture view created with result = %d\n", result);

    VkSamplerCreateInfo samplerInfo = {0};
    samplerInfo.sType = VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO;
    samplerInfo.magFilter = VK_FILTER_LINEAR;
    samplerInfo.minFilter = VK_FILTER_LINEAR;
    samplerInfo.addressModeU = VK_SAMPLER_ADDRESS_MODE_REPEAT;
    samplerInfo.addressModeV = VK_SAMPLER_ADDRESS_MODE_REPEAT;
    samplerInfo.addressModeW = VK_SAMPLER_ADDRESS_MODE_REPEAT;
    samplerInfo.anisotropyEnable = VK_TRUE;

    VkPhysicalDeviceProperties properties = {0};
    vkGetPhysicalDeviceProperties(euclid.handle[eh].physicalDevices[euclid.handle[eh].chosenDevice], &properties);

    samplerInfo.maxAnisotropy = properties.limits.maxSamplerAnisotropy;
    samplerInfo.borderColor = VK_BORDER_COLOR_INT_OPAQUE_BLACK;
    samplerInfo.unnormalizedCoordinates = VK_FALSE;
    samplerInfo.compareEnable = VK_FALSE;
    samplerInfo.compareOp = VK_COMPARE_OP_ALWAYS;
    samplerInfo.mipmapMode = VK_SAMPLER_MIPMAP_MODE_LINEAR;
    samplerInfo.mipLodBias = 0.0f;
    samplerInfo.minLod = 0.0f;
    samplerInfo.maxLod = euclid.textures[te].mipLevels;

    result = vkCreateSampler(euclid.handle[eh].device, &samplerInfo, NULL, &euclid.textures[te].sampler);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidTEX\e[0;37m: Sampler created with result = %d\n", result);
    return te;
}

uint32_t newtexture(uint32_t eh, uint32_t xsize, uint32_t ysize, uint32_t zsize, uint32_t byteperpixel, char *pixels, uint8_t is3d, uint32_t imageformat, uint8_t genmips){
    uint32_t te = euclid.vtsize;
    euclid.vtsize++;

    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidOQ\e[0;37m: New texture is being pushed in queue creation, index = %i\n", te);

    euclidCreationQueue item = {0};
    item.type = EUCLID_QUEUE_TYPE_TEXTURE;
    item.em = te;
    item.xsize = xsize;
    item.ysize = ysize;
    item.zsize = zsize;
    item.byteperpixel = byteperpixel;
    item.is3d = is3d;
    item.imageformat = imageformat;
    item.genmips = genmips;
    if(xsize > 0 && ysize > 0 && zsize > 0 && byteperpixel > 0){
        uint32_t pixelSize = xsize * ysize * zsize * byteperpixel;
        item.pixels = malloc(pixelSize);
        memcpy(item.pixels, pixels, pixelSize);
    }

    push_creation_queue(eh, &item);

    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidOQ\e[0;37m: New texture pushed in queue creation, index = %i\n", te);

    return te;
}

void draw(uint32_t eh, uint32_t eme){
    if(euclid.handle[eh].mrec != euclid.meshes[eme].mrec && euclid.meshes[eme].usage == 0){
        vkFreeDescriptorSets(euclid.handle[eh].device, euclid.meshes[eme].descriptorPool, MAX_FRAMES_IN_FLIGHT, euclid.meshes[eme].descriptorSets);
        vkDestroyDescriptorPool(euclid.handle[eh].device, euclid.meshes[eme].descriptorPool, NULL);
        free(euclid.meshes[eme].descriptorSets);
        createDescriptorPool(eh, eme);
        createDescriptorSets(eh, eme);
        vkDestroyPipeline(euclid.handle[eh].device, euclid.meshes[eme].graphicsPipeline, NULL);
        createPipeline(eh, eme, euclid.meshes[eme].savpapparam[0], euclid.meshes[eme].savpapparam[1]);
        euclid.meshes[eme].mrec = euclid.handle[eh].mrec;
    }

    vkCmdBindPipeline(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+3], VK_PIPELINE_BIND_POINT_GRAPHICS, euclid.meshes[eme].graphicsPipeline);

    VkDeviceSize offsets[] = {0};
    vkCmdBindVertexBuffers(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+3], 0, 1, &euclid.models[euclid.meshes[eme].modelId].vertexBuffer, offsets);

    VkViewport viewport = {0};
    viewport.x = 0.0f;
    viewport.y = 0.0f;
    viewport.width = euclid.handle[eh].resolutionX[0];
    viewport.height = euclid.handle[eh].resolutionY[0];
    viewport.minDepth = 0.0f;
    viewport.maxDepth = 1.0f;
    vkCmdSetViewport(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+3], 0, 1, &viewport);

    VkRect2D scissor = {0};
    scissor.offset.x = 0;
    scissor.offset.y = 0;
    scissor.extent.width = euclid.handle[eh].resolutionX[0];
    scissor.extent.height = euclid.handle[eh].resolutionY[0];
    vkCmdSetScissor(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+3], 0, 1, &scissor);

    uint8_t usedslot = (euclid.meshes[eme].uniformslot+1)%2;
    euclid.meshes[eme].lub[usedslot][0] = (float) euclid.handle[eh].resolutionX[0];
    euclid.meshes[eme].lub[usedslot][1] = (float) euclid.handle[eh].resolutionY[0];
    euclid.meshes[eme].lub[usedslot][2] = (float) euclid.handle[eh].shadowMapResolution[0];
    euclid.meshes[eme].lub[usedslot][3] = (float) euclid.handle[eh].lightattn;
    euclid.meshes[eme].lub[usedslot][4] = (float) euclid.handle[eh].shadowMapsCount[0];
    euclid.meshes[eme].lub[usedslot][5] = (float) euclid.handle[eh].resolutionScale[0];
    euclid.meshes[eme].lub[usedslot][6] = (float) euclid.handle[eh].defferedCount[0];
    euclid.meshes[eme].lub[usedslot][7] = (float) euclid.handle[eh].lightsCount;
    memcpy(euclid.meshes[eme].uniformBuffersMapped[euclid.handle[eh].currentFrame], euclid.meshes[eme].lub[usedslot], sizeof(euclid.meshes[eme].lub[usedslot]));
    vkCmdBindDescriptorSets(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+3], VK_PIPELINE_BIND_POINT_GRAPHICS, euclid.meshes[eme].pipelineLayout, 0, 1, &euclid.meshes[eme].descriptorSets[euclid.handle[eh].currentFrame], 0, NULL);

    vkCmdDraw(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+3], euclid.models[euclid.meshes[eme].modelId].vertnum, 1, 0, 0);
}

void drawlighting(uint32_t eh, uint32_t eme){
    if(euclid.handle[eh].mrec != euclid.meshes[eme].mrec && euclid.meshes[eme].usage == 4){
        vkFreeDescriptorSets(euclid.handle[eh].device, euclid.meshes[eme].lightingDescriptorPool, 1, &euclid.meshes[eme].lightingDescriptorSets);
        vkDestroyDescriptorPool(euclid.handle[eh].device, euclid.meshes[eme].lightingDescriptorPool, NULL);
        //free(euclid.meshes[eme].lightingDescriptorSets);
        createLightingDescriptorPool(eh, eme);
        createLightingDescriptorSets(eh, eme);
        vkDestroyPipeline(euclid.handle[eh].device, euclid.meshes[eme].graphicsPipeline, NULL);
        createLightingPipeline(eh, eme, euclid.meshes[eme].savpapparam[0], euclid.meshes[eme].savpapparam[1]);
        euclid.meshes[eme].mrec = euclid.handle[eh].mrec;
    }

    vkCmdBindPipeline(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+2], VK_PIPELINE_BIND_POINT_GRAPHICS, euclid.meshes[eme].graphicsPipeline);

    VkDeviceSize offsets[] = {0};
    vkCmdBindVertexBuffers(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+2], 0, 1, &euclid.models[euclid.meshes[eme].modelId].vertexBuffer, offsets);

    VkViewport viewport = {0};
    viewport.x = 0.0f;
    viewport.y = 0.0f;
    viewport.width = euclid.handle[eh].renderResolutionX;
    viewport.height = euclid.handle[eh].renderResolutionY;
    viewport.minDepth = 0.0f;
    viewport.maxDepth = 1.0f;
    vkCmdSetViewport(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+2], 0, 1, &viewport);

    VkRect2D scissor = {0};
    scissor.offset.x = 0;
    scissor.offset.y = 0;
    scissor.extent.width = euclid.handle[eh].renderResolutionX;
    scissor.extent.height = euclid.handle[eh].renderResolutionY;
    vkCmdSetScissor(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+2], 0, 1, &scissor);

    uint8_t usedslot = (euclid.meshes[eme].uniformslot+1)%2;
    euclid.meshes[eme].lub[usedslot][0] = (float) euclid.handle[eh].renderResolutionX;
    euclid.meshes[eme].lub[usedslot][1] = (float) euclid.handle[eh].renderResolutionY;
    euclid.meshes[eme].lub[usedslot][2] = (float) euclid.handle[eh].shadowMapResolution[0];
    euclid.meshes[eme].lub[usedslot][3] = (float) euclid.handle[eh].lightattn;
    euclid.meshes[eme].lub[usedslot][4] = (float) euclid.handle[eh].shadowMapsCount[0];
    euclid.meshes[eme].lub[usedslot][5] = (float) euclid.handle[eh].resolutionScale[0];
    euclid.meshes[eme].lub[usedslot][6] = (float) euclid.handle[eh].defferedCount[0];
    euclid.meshes[eme].lub[usedslot][7] = (float) euclid.handle[eh].lightsCount;
    memcpy(euclid.meshes[eme].uniformBuffersMapped[MAX_FRAMES_IN_FLIGHT], euclid.meshes[eme].lub[usedslot], sizeof(euclid.meshes[eme].lub[usedslot]));
    vkCmdBindDescriptorSets(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+2], VK_PIPELINE_BIND_POINT_GRAPHICS, euclid.meshes[eme].pipelineLayout, 0, 1, &euclid.meshes[eme].lightingDescriptorSets, 0, NULL);

    vkCmdDraw(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+2], euclid.models[euclid.meshes[eme].modelId].vertnum, 1, 0, 0);
}

void drawshadow(uint32_t eh, uint32_t eme, uint32_t cs){
    vkCmdBindPipeline(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4], VK_PIPELINE_BIND_POINT_GRAPHICS, euclid.meshes[eme].shadowPipeline);

    VkDeviceSize offsets[] = {0};
    vkCmdBindVertexBuffers(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4], 0, 1, &euclid.models[euclid.meshes[eme].modelId].vertexBuffer, offsets);

    VkViewport viewport = {0};
    viewport.x = 0.0f;
    viewport.y = 0.0f;
    viewport.width = euclid.handle[eh].shadowMapResolution[0];
    viewport.height = euclid.handle[eh].shadowMapResolution[0];
    viewport.minDepth = 0.0f;
    viewport.maxDepth = 1.0f;
    vkCmdSetViewport(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4], 0, 1, &viewport);

    VkRect2D scissor = {0};
    scissor.offset.x = 0;
    scissor.offset.y = 0;
    scissor.extent.height = euclid.handle[eh].shadowMapResolution[0];
    scissor.extent.width = euclid.handle[eh].shadowMapResolution[0];
    vkCmdSetScissor(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4], 0, 1, &scissor);

    // NOTE: the shared usage==3 globals UBO (uniformBuffersMapped[MAX_FRAMES_IN_FLIGHT]) is now
    // written once per mesh by updatePassGlobalsUniform() on the coordinator thread, before the
    // shadow/deferred passes are spawned, to avoid two threads racing to write identical data
    // into the same host-mapped memory (see renderarbeite()).
    vkCmdBindDescriptorSets(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4], VK_PIPELINE_BIND_POINT_GRAPHICS, euclid.meshes[eme].shadowPipelineLayout, 0, 1, &euclid.meshes[eme].shadowDescriptorSets[cs], 0, NULL);

    vkCmdDraw(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4], euclid.models[euclid.meshes[eme].modelId].vertnum, 1, 0, 0);
}

void drawdeffered(uint32_t eh, uint32_t eme, uint32_t cs){
    vkCmdBindPipeline(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+1], VK_PIPELINE_BIND_POINT_GRAPHICS, euclid.meshes[eme].defferedPipeline);

    VkDeviceSize offsets[] = {0};
    vkCmdBindVertexBuffers(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+1], 0, 1, &euclid.models[euclid.meshes[eme].modelId].vertexBuffer, offsets);

    VkViewport viewport = {0};
    viewport.x = 0.0f;
    viewport.y = 0.0f;
    viewport.width = (float) euclid.handle[eh].renderResolutionX;
    viewport.height = (float) euclid.handle[eh].renderResolutionY;
    viewport.minDepth = 0.0f;
    viewport.maxDepth = 1.0f;
    vkCmdSetViewport(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+1], 0, 1, &viewport);

    VkRect2D scissor = {0};
    scissor.offset.x = 0;
    scissor.offset.y = 0;
    scissor.extent.width = euclid.handle[eh].renderResolutionX;
    scissor.extent.height = euclid.handle[eh].renderResolutionY;
    vkCmdSetScissor(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+1], 0, 1, &scissor);

    // NOTE: see drawshadow() — the shared usage==3 globals UBO write was hoisted out of here too.
    vkCmdBindDescriptorSets(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+1], VK_PIPELINE_BIND_POINT_GRAPHICS, euclid.meshes[eme].defferedPipelineLayout, 0, 1, &euclid.meshes[eme].defferedDescriptorSets[cs], 0, NULL);

    vkCmdDraw(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame*4+1], euclid.models[euclid.meshes[eme].modelId].vertnum, 1, 0, 0);
}

// Writes the per-mesh "globals" uniform block (resolution, shadow map info, light count, etc.)
// used by the shadow and deferred passes. For usage==3 meshes both passes run on separate
// threads (shadowpassrender / deffredpassrender) but need this identical data, so it must be
// written exactly once here, by the coordinator thread, before those threads are created —
// pthread_create()'s happens-before guarantee makes the write visible to both without a race.
static void updatePassGlobalsUniform(uint32_t eh, uint32_t eme){
    uint8_t usedslot = (euclid.meshes[eme].uniformslot+1)%2;
    euclid.meshes[eme].lub[usedslot][0] = (float) euclid.handle[eh].resolutionX[0];
    euclid.meshes[eme].lub[usedslot][1] = (float) euclid.handle[eh].resolutionY[0];
    euclid.meshes[eme].lub[usedslot][2] = (float) euclid.handle[eh].shadowMapResolution[0];
    euclid.meshes[eme].lub[usedslot][3] = (float) euclid.handle[eh].totalFrames;
    euclid.meshes[eme].lub[usedslot][4] = (float) euclid.handle[eh].shadowMapsCount[0];
    euclid.meshes[eme].lub[usedslot][5] = (float) euclid.handle[eh].renderResolutionX;
    euclid.meshes[eme].lub[usedslot][6] = (float) euclid.handle[eh].renderResolutionY;
    euclid.meshes[eme].lub[usedslot][7] = (float) euclid.handle[eh].lightsCount;
    memcpy(euclid.meshes[eme].uniformBuffersMapped[MAX_FRAMES_IN_FLIGHT], euclid.meshes[eme].lub[usedslot], sizeof(euclid.meshes[eme].lub[usedslot]));
}

void* shadowpassrender(void *arg){
    uint32_t *deh = (uint32_t*) arg;
    uint32_t eh = deh[0];
    if(euclid.handle[eh].enableShadowMaps){
        for(uint32_t i = 0; i != euclid.handle[eh].shadowMapsCount[0]; i++){
            startshadowrenderpass(eh, i);
            for(uint32_t j = 0; j != euclid.mesize; j++){
                if(euclid.meshes[j].euclidid == eh && (euclid.meshes[j].drawable == 1 || euclid.meshes[j].drawable == 2) && (euclid.meshes[j].usage == 2 || euclid.meshes[j].usage == 3)){
                    drawshadow(eh, j, i);
                }
            }
            endrenderpass(eh, 0);
        }
    }
    return NULL;
}

void* deffredpassrender(void *arg){
    uint32_t *deh = (uint32_t*) arg;
    uint32_t eh = deh[0];
    for(uint32_t i = 0; i != euclid.handle[eh].defferedCount[0]; i++){
        startdefferedrenderpass(eh, i);
        for(uint32_t j = 0; j != euclid.mesize; j++){
            if(euclid.meshes[j].euclidid == eh && (euclid.meshes[j].drawable == 1 || euclid.meshes[j].drawable == 3) && (euclid.meshes[j].usage == 1 || euclid.meshes[j].usage == 3) && (euclid.meshes[j].camrend == -1 || euclid.meshes[j].camrend == i || (euclid.meshes[j].camrend - 10 != i && euclid.meshes[j].camrend >= 10))){
                drawdeffered(eh, j, i);
            }
        }
        endrenderpass(eh, 1);
    }
    return NULL;
}

void* lightingpassrender(void *arg){
    uint32_t *deh = (uint32_t*) arg;
    uint32_t eh = deh[0];
    startlightingrenderpass(eh);
    for(uint32_t i = 0; i != euclid.mesize; i++){
        if(euclid.meshes[i].euclidid == eh && euclid.meshes[i].drawable == 1 && euclid.meshes[i].usage == 4){
            drawlighting(eh, i);
        }
    }
    endrenderpass(eh, 2);
    return NULL;
}

void* renderarbeite(void *arg){
    uint32_t *deh = (uint32_t*) arg;
    uint32_t eh = deh[0];
    while(euclid.handle[eh].lpcont != 0){
        if(euclid.handle[eh].halt_render == 0){
            startrender(eh);
            for(uint32_t i = 0; i != euclid.mesize; i++){
                if(euclid.meshes[i].euclidid != eh || euclid.meshes[i].usage == 0 || euclid.meshes[i].usage == 4){
                    continue;
                }
                uint8_t shadoweligible = euclid.handle[eh].enableShadowMaps && (euclid.meshes[i].drawable == 1 || euclid.meshes[i].drawable == 2) && (euclid.meshes[i].usage == 2 || euclid.meshes[i].usage == 3);
                uint8_t defferedeligible = (euclid.meshes[i].drawable == 1 || euclid.meshes[i].drawable == 3) && (euclid.meshes[i].usage == 1 || euclid.meshes[i].usage == 3);
                if(shadoweligible || defferedeligible){
                    updatePassGlobalsUniform(eh, i);
                }
            }
            pthread_create(&euclid.handle[eh].threads[0], NULL, shadowpassrender, arg);
            pthread_create(&euclid.handle[eh].threads[1], NULL, deffredpassrender, arg);
            pthread_create(&euclid.handle[eh].threads[2], NULL, lightingpassrender, arg);
            startmainrenderpass(eh);
            for(uint32_t i = 0; i != euclid.mesize; i++){
                if(euclid.meshes[i].euclidid == eh && euclid.meshes[i].drawable == 1 && euclid.meshes[i].usage == 0){
                    draw(eh, i);
                }
            }
            endrenderpass(eh, 3);
            pthread_join(euclid.handle[eh].threads[0], NULL);
            pthread_join(euclid.handle[eh].threads[1], NULL);
            pthread_join(euclid.handle[eh].threads[2], NULL);
            endrender(eh);
            for(uint32_t i = 0; i != euclid.mesize; i++){
                euclid.meshes[i].uniformslot = (euclid.meshes[i].uniformslot+1)%2;
            }
            euclid.handle[eh].defferedslot = (euclid.handle[eh].defferedslot+1)%2;
            euclid.handle[eh].shadowslot = (euclid.handle[eh].shadowslot+1)%2;
        }else if(euclid.handle[eh].halt_render == 1){
            if (euclid.handle[eh].debug == 1) printf("\e[1;35mEuclidVK\e[0;37m: Rendering paused\n");
            euclid.handle[eh].halt_render = 2;
        }else{
            // halt_render == 2: waiting for the main thread's loopcont() to drain the creation
            // queue and reset halt_render to 0. Sleep briefly instead of busy-spinning a full core.
            struct timespec ts = {0, 500000L}; // 0.5ms
            nanosleep(&ts, NULL);
        }
    }
    return NULL;
}

uint32_t neweng(uint32_t shadowMapResolution, uint8_t debug){
    uint32_t eh = euclid.size;
    euclid.size++;
    euclidh *grown = realloc(euclid.handle, sizeof(euclidh)*euclid.size);
    if(grown == NULL){
        if(debug == 1) printf("\e[1;31mError\e[0;37m: Failed to allocate engine handle table\n");
        exit(-1);
    }
    euclid.handle = grown;
    // NOTE: growing this table still isn't safe to call concurrently with another handle's
    // render thread (renderarbeite) dereferencing euclid.handle[...] — realloc can move the
    // whole array. Only call neweng() for additional handles when no other handle's render
    // thread is running, or add external synchronization around this table if that's needed.
    euclid.handle[eh].chosenDevice = -1;
    euclid.handle[eh].usedPresentMode = 0;
    euclid.handle[eh].debug = debug;
    euclid.handle[eh].resolutionX[0] = 800;
    euclid.handle[eh].resolutionY[0] = 600;
    euclid.handle[eh].resolutionX[1] = 800;
    euclid.handle[eh].resolutionY[1] = 600;
    euclid.handle[eh].shadowMapsCount[0] = 1;
    euclid.handle[eh].shadowMapsCount[1] = 1;
    euclid.handle[eh].lightsCount = 1;
    euclid.handle[eh].oldshadowMapsCount = 1;
    euclid.handle[eh].shadowMapResolution[0] = shadowMapResolution;
    euclid.handle[eh].shadowMapResolution[1] = shadowMapResolution;
    euclid.handle[eh].oldshadowMapResolution = shadowMapResolution;
    euclid.handle[eh].resolutionScale[0] = 1;
    euclid.handle[eh].defferedCount[0] = 1;
    euclid.handle[eh].resolutionScale[1] = 1;
    euclid.handle[eh].defferedCount[1] = 1;
    euclid.handle[eh].lpcont = 1;
    euclid.handle[eh].halt_render = 0;
    euclid.handle[eh].currentFrame = 0;
    euclid.handle[eh].imageIndex = 0;
    euclid.handle[eh].totalFrames = 0;
    euclid.handle[eh].lightattn = 0;
    euclid.handle[eh].mrec = 0;
    euclid.handle[eh].frametime = 0;
    euclid.handle[eh].defferedslot = 0;
    euclid.handle[eh].shadowslot = 0;

    createInstance(eh);
    getDevice(eh);
    glfwInit();
    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    glfwWindowHint(GLFW_TRANSPARENT_FRAMEBUFFER, GLFW_TRUE);
    euclid.handle[eh].window = glfwCreateWindow(800, 600, "Schnellwerke", NULL, NULL);
    glfwCreateWindowSurface(euclid.handle[eh].instance, euclid.handle[eh].window, NULL, &euclid.handle[eh].surface);
    getPresentFamily(eh);
    createDevice(eh);
    createSwapChain(eh);
    createSwapChainImageViews(eh);
    createRenderPass(eh);
    createShadowRenderPass(eh);
    createDefferedRenderPass(eh);
    createLightingRenderPass(eh);
    createShadowData(eh);
    createDefferedData(eh);
    createLightingData(eh);
    createFrameBuffers(eh);
    createCommandPool(eh);
    createCommandBuffer(eh);
    createSyncObjects(eh);

    {
        VkSamplerCreateInfo samplerInfo = {0};
        samplerInfo.sType = VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO;
        samplerInfo.magFilter = VK_FILTER_NEAREST;
        samplerInfo.minFilter = VK_FILTER_NEAREST;
        samplerInfo.addressModeU = VK_SAMPLER_ADDRESS_MODE_REPEAT;
        samplerInfo.addressModeV = VK_SAMPLER_ADDRESS_MODE_REPEAT;
        samplerInfo.addressModeW = VK_SAMPLER_ADDRESS_MODE_REPEAT;
        samplerInfo.anisotropyEnable = VK_FALSE;
        samplerInfo.maxAnisotropy = 0;
        samplerInfo.borderColor = VK_BORDER_COLOR_INT_OPAQUE_BLACK;
        samplerInfo.unnormalizedCoordinates = VK_FALSE;
        samplerInfo.compareEnable = VK_FALSE;
        samplerInfo.compareOp = VK_COMPARE_OP_ALWAYS;
        samplerInfo.mipmapMode = VK_SAMPLER_MIPMAP_MODE_NEAREST;
        samplerInfo.mipLodBias = 0.0f;
        samplerInfo.minLod = 0.0f;
        samplerInfo.maxLod = 0.0f;

        VkResult result = vkCreateSampler(euclid.handle[eh].device, &samplerInfo, NULL, &euclid.handle[eh].attachmentSampler);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidTEX\e[0;37m: Attachment sampler created with result = %d\n", result);
    }

    {
        VkDeviceSize bufferSize = sizeof(float)*2400;
        VkBufferCreateInfo bufferInfo = {0};
        bufferInfo.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        bufferInfo.size = bufferSize;
        bufferInfo.usage = VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT;
        bufferInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
        VkResult result = vkCreateBuffer(euclid.handle[eh].device, &bufferInfo, NULL, &euclid.handle[eh].shadowUniformBuffer);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Shadow uniform buffer created with result = %d\n", result);
        VkMemoryRequirements memRequirements;
        vkGetBufferMemoryRequirements(euclid.handle[eh].device, euclid.handle[eh].shadowUniformBuffer, &memRequirements);
        
        VkMemoryAllocateInfo allocInfo = {0};
        allocInfo.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        allocInfo.allocationSize = memRequirements.size;
        allocInfo.memoryTypeIndex = findMemoryType(memRequirements.memoryTypeBits, VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT, eh);
        
        euclid.handle[eh].shadowUniformBuffersMapped = malloc(sizeof(void*));
        
        vkAllocateMemory(euclid.handle[eh].device, &allocInfo, NULL, &euclid.handle[eh].shadowUniformBuffersMemory);
        vkBindBufferMemory(euclid.handle[eh].device, euclid.handle[eh].shadowUniformBuffer, euclid.handle[eh].shadowUniformBuffersMemory, 0);
        vkMapMemory(euclid.handle[eh].device, euclid.handle[eh].shadowUniformBuffersMemory, 0, bufferSize, 0, euclid.handle[eh].shadowUniformBuffersMapped);
    }

    {
        VkDeviceSize bufferSize = sizeof(float)*400;
        VkBufferCreateInfo bufferInfo = {0};
        bufferInfo.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        bufferInfo.size = bufferSize;
        bufferInfo.usage = VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT;
        bufferInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
        VkResult result = vkCreateBuffer(euclid.handle[eh].device, &bufferInfo, NULL, &euclid.handle[eh].defferedUniformBuffer);
        if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Deffered uniform buffer created with result = %d\n", result);
        VkMemoryRequirements memRequirements;
        vkGetBufferMemoryRequirements(euclid.handle[eh].device, euclid.handle[eh].defferedUniformBuffer, &memRequirements);
        
        VkMemoryAllocateInfo allocInfo = {0};
        allocInfo.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        allocInfo.allocationSize = memRequirements.size;
        allocInfo.memoryTypeIndex = findMemoryType(memRequirements.memoryTypeBits, VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT, eh);

        euclid.handle[eh].defferedUniformBuffersMapped = malloc(sizeof(void*));

        vkAllocateMemory(euclid.handle[eh].device, &allocInfo, NULL, &euclid.handle[eh].defferedUniformBuffersMemory);
        vkBindBufferMemory(euclid.handle[eh].device, euclid.handle[eh].defferedUniformBuffer, euclid.handle[eh].defferedUniformBuffersMemory, 0);
        vkMapMemory(euclid.handle[eh].device, euclid.handle[eh].defferedUniformBuffersMemory, 0, bufferSize, 0, euclid.handle[eh].defferedUniformBuffersMapped);
    }

    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Creating rendering thread\n");

    void *eht = malloc(sizeof(uint32_t));

    memcpy(eht, &eh, sizeof(uint32_t));

    pthread_create(&euclid.handle[eh].threads[3], NULL, renderarbeite, eht);

    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Engine initializaion complete\n");

    return eh;
}

uint32_t loopcont(uint32_t eh){
    glfwPollEvents();
    euclid.handle[eh].frametime = glfwGetTime();
    glfwSetTime(0);
    keywork(eh);
    uint32_t tsx, tsy;
    glfwGetFramebufferSize(euclid.handle[eh].window, &tsx, &tsy);
    euclid.handle[eh].resolutionX[1] = tsx;
    euclid.handle[eh].resolutionY[1] = tsy;

    if(euclid.handle[eh].queuesize > 0 && euclid.handle[eh].halt_render == 0){
        euclid.handle[eh].halt_render = 1;
    }
    if(euclid.handle[eh].halt_render == 2){
        process_creation_queue(eh);
        euclid.handle[eh].halt_render = 0;
        if (euclid.handle[eh].debug == 1) printf("\e[1;35mEuclidVK\e[0;37m: Rendering resumed\n");
    }

    euclid.handle[eh].lpcont = !glfwWindowShouldClose(euclid.handle[eh].window);
    return euclid.handle[eh].lpcont;
}

void destroy(uint32_t eh){
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Destroing handle by id = %d...\n", eh);
    pthread_join(euclid.handle[eh].threads[3], NULL);
    vkDeviceWaitIdle(euclid.handle[eh].device);
    vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].defferedImageView, NULL);
    for(uint32_t i = 0; i != euclid.handle[eh].defferedCount[0]; i++){
        vkDestroyFramebuffer(euclid.handle[eh].device, euclid.handle[eh].defferedFramebuffers[i], NULL);
        vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].defferedRenderImageViews[i], NULL);
    }
    vkFreeMemory(euclid.handle[eh].device, euclid.handle[eh].defferedImageMemory, NULL);
    vkDestroyImage(euclid.handle[eh].device, euclid.handle[eh].defferedImage, NULL);
    vkDestroyBuffer(euclid.handle[eh].device, euclid.handle[eh].defferedUniformBuffer, NULL);
    vkFreeMemory(euclid.handle[eh].device, euclid.handle[eh].defferedUniformBuffersMemory, NULL);
    free(euclid.handle[eh].defferedUniformBuffersMapped);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Destroyed deffered data\n");
    vkDestroyFramebuffer(euclid.handle[eh].device, euclid.handle[eh].lightingFramebuffers[0], NULL);
    vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].lightingImageViews[0], NULL);
    vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].lightingDepthImageViews[0], NULL);
    vkDestroyFramebuffer(euclid.handle[eh].device, euclid.handle[eh].lightingFramebuffers[1], NULL);
    vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].lightingImageViews[1], NULL);
    vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].lightingDepthImageViews[1], NULL);
    vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].lightingImageViewl, NULL);
    vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].lightingDepthImageViewl, NULL);
    vkDestroyImage(euclid.handle[eh].device, euclid.handle[eh].lightingImage, NULL);
    vkFreeMemory(euclid.handle[eh].device, euclid.handle[eh].lightingImageMemory, NULL);
    vkDestroyImage(euclid.handle[eh].device, euclid.handle[eh].lightingDepthImage, NULL);
    vkFreeMemory(euclid.handle[eh].device, euclid.handle[eh].lightingDepthImageMemory, NULL);
    vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].shadowImageView, NULL);
    for(uint32_t i = 0; i != euclid.handle[eh].shadowMapsCount[0]; i++){
        vkDestroyFramebuffer(euclid.handle[eh].device, euclid.handle[eh].shadowFramebuffers[i], NULL);
        vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].shadowRenderImageViews[i], NULL);
    }
    vkFreeMemory(euclid.handle[eh].device, euclid.handle[eh].shadowImageMemory, NULL);
    vkDestroyImage(euclid.handle[eh].device, euclid.handle[eh].shadowImage, NULL);
    vkDestroyBuffer(euclid.handle[eh].device, euclid.handle[eh].shadowUniformBuffer, NULL);
    vkFreeMemory(euclid.handle[eh].device, euclid.handle[eh].shadowUniformBuffersMemory, NULL);
    free(euclid.handle[eh].shadowUniformBuffersMapped);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Destroyed shadow data\n");
    for(uint32_t i = 0; i != euclid.mesize; i++){
        if(euclid.meshes[i].euclidid != eh){
            // This mesh belongs to a different, still-live engine handle — its Vulkan
            // objects were created against a different VkDevice, so skip it entirely here.
            continue;
        }
        if(euclid.meshes[i].usage == 0){
            vkDestroyPipeline(euclid.handle[eh].device, euclid.meshes[i].graphicsPipeline, NULL);
            vkDestroyDescriptorSetLayout(euclid.handle[eh].device, euclid.meshes[i].descriptorSetLayout, NULL);
            vkDestroyPipelineLayout(euclid.handle[eh].device, euclid.meshes[i].pipelineLayout, NULL);
            vkDestroyDescriptorPool(euclid.handle[eh].device, euclid.meshes[i].descriptorPool, NULL);
            free(euclid.meshes[i].descriptorSets);
        }
        if(euclid.meshes[i].usage == 1){
            vkDestroyPipeline(euclid.handle[eh].device, euclid.meshes[i].defferedPipeline, NULL);
            vkDestroyDescriptorSetLayout(euclid.handle[eh].device, euclid.meshes[i].defferedDescriptorSetLayout, NULL);
            vkDestroyPipelineLayout(euclid.handle[eh].device, euclid.meshes[i].defferedPipelineLayout, NULL);
            vkDestroyDescriptorPool(euclid.handle[eh].device, euclid.meshes[i].defferedDescriptorPool, NULL);
        }
        if(euclid.meshes[i].usage == 2){
            vkDestroyPipeline(euclid.handle[eh].device, euclid.meshes[i].shadowPipeline, NULL);
            vkDestroyDescriptorSetLayout(euclid.handle[eh].device, euclid.meshes[i].shadowDescriptorSetLayout, NULL);
            vkDestroyPipelineLayout(euclid.handle[eh].device, euclid.meshes[i].shadowPipelineLayout, NULL);
            vkDestroyDescriptorPool(euclid.handle[eh].device, euclid.meshes[i].shadowDescriptorPool, NULL);
        }
        if(euclid.meshes[i].usage == 3){
            vkDestroyPipeline(euclid.handle[eh].device, euclid.meshes[i].defferedPipeline, NULL);
            vkDestroyDescriptorSetLayout(euclid.handle[eh].device, euclid.meshes[i].defferedDescriptorSetLayout, NULL);
            vkDestroyPipelineLayout(euclid.handle[eh].device, euclid.meshes[i].defferedPipelineLayout, NULL);
            vkDestroyDescriptorPool(euclid.handle[eh].device, euclid.meshes[i].defferedDescriptorPool, NULL);
            vkDestroyPipeline(euclid.handle[eh].device, euclid.meshes[i].shadowPipeline, NULL);
            vkDestroyDescriptorSetLayout(euclid.handle[eh].device, euclid.meshes[i].shadowDescriptorSetLayout, NULL);
            vkDestroyPipelineLayout(euclid.handle[eh].device, euclid.meshes[i].shadowPipelineLayout, NULL);
            vkDestroyDescriptorPool(euclid.handle[eh].device, euclid.meshes[i].shadowDescriptorPool, NULL);
        }
        if(euclid.meshes[i].usage == 4){
            vkDestroyPipeline(euclid.handle[eh].device, euclid.meshes[i].graphicsPipeline, NULL);
            vkDestroyDescriptorSetLayout(euclid.handle[eh].device, euclid.meshes[i].lightingDescriptorSetLayout, NULL);
            vkFreeDescriptorSets(euclid.handle[eh].device, euclid.meshes[i].lightingDescriptorPool, 1, &euclid.meshes[i].lightingDescriptorSets);
            vkDestroyDescriptorPool(euclid.handle[eh].device, euclid.meshes[i].lightingDescriptorPool, NULL);
        }
        for(uint32_t j = 0; j != MAX_FRAMES_IN_FLIGHT; j++){
            vkDestroyBuffer(euclid.handle[eh].device, euclid.meshes[i].uniformBuffers[j], NULL);
            vkFreeMemory(euclid.handle[eh].device, euclid.meshes[i].uniformBuffersMemory[j], NULL);
        }
        free(euclid.meshes[i].uniformBuffers);
        free(euclid.meshes[i].uniformBuffersMemory);
        free(euclid.meshes[i].uniformBuffersMapped);
        free(euclid.meshes[i].savedtex);
    }
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Destroyed uniform buffers\n");
    for(uint32_t i = 0; i != euclid.tsize; i++){
        vkDestroyImageView(euclid.handle[eh].device, euclid.textures[i].textureImageView, NULL);
        vkDestroyImage(euclid.handle[eh].device, euclid.textures[i].texture, NULL);
        vkFreeMemory(euclid.handle[eh].device, euclid.textures[i].textureImageMemory, NULL);
    }
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Destroyed textures\n");
    for(uint32_t i = 0; i != euclid.mosize; i++){
        vkDestroyBuffer(euclid.handle[eh].device, euclid.models[i].vertexBuffer, NULL);
        vkFreeMemory(euclid.handle[eh].device, euclid.models[i].vertexBufferMemory, NULL);
    }
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Destroyed vertexbuffers\n");
    for(uint32_t i = 0; i != euclid.msize; i++){
        vkDestroyShaderModule(euclid.handle[eh].device, euclid.materials[i].fragModule, NULL);
        vkDestroyShaderModule(euclid.handle[eh].device, euclid.materials[i].vertModule, NULL);
    }
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Destroyed materials\n");
    for(int i = 0; i != MAX_FRAMES_IN_FLIGHT; i++){
        vkDestroySemaphore(euclid.handle[eh].device, euclid.handle[eh].imageAvailableSemaphores[i], NULL);
        vkDestroySemaphore(euclid.handle[eh].device, euclid.handle[eh].renderFinishedSemaphores[i], NULL);
        vkDestroyFence(euclid.handle[eh].device, euclid.handle[eh].inFlightFences[i], NULL);
    }
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Destroyed sync objects\n");
    for(int i = 0; i < 4; i++){
        for(int j = 0; j < MAX_FRAMES_IN_FLIGHT; j++){
            vkFreeCommandBuffers(euclid.handle[eh].device, euclid.handle[eh].commandPool[i], 1, &euclid.handle[eh].commandBuffers[j*4+i]);
        }
        vkDestroyCommandPool(euclid.handle[eh].device, euclid.handle[eh].commandPool[i], NULL);
    }
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Destroyed command pool and buffer\n");
    for(int i = 0; i != euclid.handle[eh].swapChainImageCount; i++){
        vkDestroyFramebuffer(euclid.handle[eh].device, euclid.handle[eh].swapChainFramebuffers[i], NULL);
    }
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Destroyed framebuffers\n");
    vkDestroyRenderPass(euclid.handle[eh].device, euclid.handle[eh].renderPass, NULL);
    vkDestroyRenderPass(euclid.handle[eh].device, euclid.handle[eh].shadowRenderPass, NULL);
    vkDestroyRenderPass(euclid.handle[eh].device, euclid.handle[eh].defferedRenderPass, NULL);
    vkDestroyRenderPass(euclid.handle[eh].device, euclid.handle[eh].lightingRenderPass, NULL);
    for(int i = 0; i != euclid.handle[eh].swapChainImageCount; i++){
        vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].swapChainImageViews[i], NULL);
    }
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Destroyed imageviews\n");
    vkDestroySwapchainKHR(euclid.handle[eh].device, euclid.handle[eh].swapChain, NULL);
    vkDestroySurfaceKHR(euclid.handle[eh].instance, euclid.handle[eh].surface, NULL);
    vkDestroyDevice(euclid.handle[eh].device, NULL);
    vkDestroyInstance(euclid.handle[eh].instance, NULL);
    free(euclid.handle[eh].swapChainFramebuffers);
    free(euclid.handle[eh].swapChainImageViews);
    free(euclid.handle[eh].swapChainImages);
    free(euclid.handle[eh].imageAvailableSemaphores);
    free(euclid.handle[eh].renderFinishedSemaphores);
    free(euclid.handle[eh].inFlightFences);
    free(euclid.handle[eh].commandBuffers);
    free(euclid.handle[eh].physicalDevices);
    if (euclid.handle[eh].debug == 1) printf("\e[1;36mEuclidVK\e[0;37m: Destroyed handle by id = %d\n", eh);
}