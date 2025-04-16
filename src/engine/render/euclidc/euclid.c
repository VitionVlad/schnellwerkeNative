#include "euclid.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <vulkan/vulkan.h>
#include <GLFW/glfw3.h>
#include <GLFW/glfw3native.h>

const int MAX_FRAMES_IN_FLIGHT = 2;

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
    uint32_t resolutionX;
    uint32_t resolutionY;
    uint32_t oldx;
    uint32_t oldy;
    VkImageView *swapChainImageViews;
    VkRenderPass renderPass;
    VkImage depthImage;
    VkImageView depthImageView;
    VkDeviceMemory depthImageMemory;
    VkFramebuffer *swapChainFramebuffers;
    VkCommandBuffer *commandBuffers;
    VkCommandPool commandPool;
    VkSemaphore *imageAvailableSemaphores;
    VkSemaphore *renderFinishedSemaphores;
    VkFence *inFlightFences;
    uint32_t currentFrame;
    uint32_t imageIndex;
    uint32_t totalFrames;
    uint32_t shadowMapResolution;
} euclidh;

typedef struct euclidmaterial{
    VkShaderModule vertModule;
    VkShaderModule fragModule;
    uint32_t cullMode;
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
    VkPipeline graphicsPipeline;
    uint32_t modelId;
    VkBuffer *uniformBuffers;
    VkDeviceMemory *uniformBuffersMemory;
    void** uniformBuffersMapped;
    VkDescriptorPool descriptorPool;
    VkDescriptorSet *descriptorSets;
    float lub[24];
    uint32_t drawable;
    uint32_t texid;
} euclidmesh;

struct euclidVK{
    euclidh *handle;
    uint32_t size;
    euclidmaterial *materials;
    uint32_t msize;
    euclidmodel *models;
    uint32_t mosize;
    euclidmesh *meshes;
    uint32_t mesize;
    euclidtexture *textures;
    uint32_t tsize;
} euclid;

uint32_t findMemoryType(uint32_t typeFilter, VkMemoryPropertyFlags properties, uint32_t eh) {
    VkPhysicalDeviceMemoryProperties memProperties;
    vkGetPhysicalDeviceMemoryProperties(euclid.handle[eh].physicalDevices[euclid.handle[eh].chosenDevice], &memProperties);
    for (uint32_t i = 0; i < memProperties.memoryTypeCount; i++) {
        if ((typeFilter & (1 << i)) && (memProperties.memoryTypes[i].propertyFlags & properties) == properties) {
            return i;
        }
    }
    printf("\e[1;31mError\e[0;37m: Cant find suitable memory");
    exit(-1);
}

void createInstance(uint32_t eh){
    VkApplicationInfo appinfo = {0};
    appinfo.apiVersion = VK_API_VERSION_1_3;
    appinfo.applicationVersion = VK_MAKE_VERSION(0, 1, 0);
    appinfo.engineVersion = VK_MAKE_VERSION(0, 1, 0);
    appinfo.pApplicationName = "Schnellwerke3n";
    appinfo.pEngineName = "euclidRender";
    appinfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
    appinfo.pNext = NULL;
    
    VkInstanceCreateInfo createInfo = {0};
    createInfo.enabledExtensionCount = 0;
    createInfo.ppEnabledExtensionNames = NULL;
    createInfo.enabledLayerCount = 0;
    createInfo.ppEnabledLayerNames = NULL;
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
    printf("\e[1;36mEuclidVK\e[0;37m: Enabled extensions count = %d\n", lrnm);
    for(int i = 0; i != lrnm; i++){
        printf("\e[1;36mEuclidVK\e[0;37m: Enabled extension %s\n", extprop[i].extensionName);
        extnms[i] = extprop[i].extensionName;
    }
    createInfo.ppEnabledExtensionNames = (const char *const *) extnms;

    VkResult result = vkCreateInstance(&createInfo, NULL, &euclid.handle[eh].instance);
    printf("\e[1;36mEuclidVK\e[0;37m: Vulkan instance created with result %d \n", result);
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
            printf("\e[1;36mEuclidVK\e[0;37m: Device id = %d\n", i);
            printf("\e[1;36mEuclidVK\e[0;37m: Device name = %s\n", deviceProperties.deviceName);
            printf("\e[1;36mEuclidVK\e[0;37m: Device api version = %d\n", deviceProperties.apiVersion);
            printf("\e[1;36mEuclidVK\e[0;37m: Device device type = %d\n", deviceProperties.deviceType);
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
            printf("\e[1;31mError\e[0;37m: Can not find a suitable device");
            exit(-1);
        }
    }
    printf("\e[1;36mEuclidVK\e[0;37m: chosen physical device id = %d\n", euclid.handle[eh].chosenDevice);
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

    VkPhysicalDeviceFeatures deviceFeatures;
    deviceFeatures.robustBufferAccess = VK_FALSE;
    deviceFeatures.fullDrawIndexUint32 = VK_FALSE;
    deviceFeatures.imageCubeArray = VK_FALSE;
    deviceFeatures.independentBlend = VK_FALSE;
    deviceFeatures.geometryShader = VK_FALSE;
    deviceFeatures.tessellationShader = VK_FALSE;
    deviceFeatures.sampleRateShading = VK_FALSE;
    deviceFeatures.dualSrcBlend = VK_FALSE;
    deviceFeatures.logicOp = VK_FALSE;
    deviceFeatures.multiDrawIndirect = VK_FALSE;
    deviceFeatures.drawIndirectFirstInstance = VK_FALSE;
    deviceFeatures.depthClamp = VK_FALSE;
    deviceFeatures.depthBiasClamp = VK_FALSE;
    deviceFeatures.fillModeNonSolid = VK_FALSE;
    deviceFeatures.depthBounds = VK_FALSE;
    deviceFeatures.wideLines = VK_FALSE;
    deviceFeatures.largePoints = VK_FALSE;
    deviceFeatures.alphaToOne = VK_FALSE;
    deviceFeatures.multiViewport = VK_FALSE;
    deviceFeatures.samplerAnisotropy = VK_FALSE;
    deviceFeatures.textureCompressionETC2 = VK_FALSE;
    deviceFeatures.textureCompressionASTC_LDR = VK_FALSE;
    deviceFeatures.textureCompressionBC = VK_FALSE;
    deviceFeatures.occlusionQueryPrecise = VK_FALSE;
    deviceFeatures.pipelineStatisticsQuery = VK_FALSE;
    deviceFeatures.vertexPipelineStoresAndAtomics = VK_FALSE;
    deviceFeatures.fragmentStoresAndAtomics = VK_FALSE;
    deviceFeatures.shaderTessellationAndGeometryPointSize = VK_FALSE;
    deviceFeatures.shaderImageGatherExtended = VK_FALSE;
    deviceFeatures.shaderStorageImageExtendedFormats = VK_FALSE;
    deviceFeatures.shaderStorageImageMultisample = VK_FALSE;
    deviceFeatures.shaderStorageImageReadWithoutFormat = VK_FALSE;
    deviceFeatures.shaderStorageImageWriteWithoutFormat = VK_FALSE;
    deviceFeatures.shaderUniformBufferArrayDynamicIndexing = VK_FALSE;
    deviceFeatures.shaderSampledImageArrayDynamicIndexing = VK_FALSE;
    deviceFeatures.shaderStorageBufferArrayDynamicIndexing = VK_FALSE;
    deviceFeatures.shaderStorageImageArrayDynamicIndexing = VK_FALSE;
    deviceFeatures.shaderClipDistance = VK_FALSE;
    deviceFeatures.shaderCullDistance = VK_FALSE;
    deviceFeatures.shaderFloat64 = VK_FALSE;
    deviceFeatures.shaderInt64 = VK_FALSE;
    deviceFeatures.shaderInt16 = VK_FALSE;
    deviceFeatures.shaderResourceResidency = VK_FALSE;
    deviceFeatures.shaderResourceMinLod = VK_FALSE;
    deviceFeatures.sparseBinding = VK_FALSE;
    deviceFeatures.sparseResidencyBuffer = VK_FALSE;
    deviceFeatures.sparseResidencyImage2D = VK_FALSE;
    deviceFeatures.sparseResidencyImage3D = VK_FALSE;
    deviceFeatures.sparseResidency2Samples = VK_FALSE;
    deviceFeatures.sparseResidency4Samples = VK_FALSE;
    deviceFeatures.sparseResidency8Samples = VK_FALSE;
    deviceFeatures.sparseResidency16Samples = VK_FALSE;
    deviceFeatures.sparseResidencyAliased = VK_FALSE;
    deviceFeatures.variableMultisampleRate = VK_FALSE;
    deviceFeatures.inheritedQueries = VK_FALSE;

    uint32_t extensionCount = 0;
    vkEnumerateDeviceExtensionProperties(euclid.handle[eh].physicalDevices[euclid.handle[eh].chosenDevice], NULL, &extensionCount, NULL);

    VkExtensionProperties *extprop = malloc(sizeof(VkExtensionProperties)*extensionCount);
    vkEnumerateDeviceExtensionProperties(euclid.handle[eh].physicalDevices[euclid.handle[eh].chosenDevice], NULL, &extensionCount, extprop);

    char** extnms = malloc(sizeof(char*)*extensionCount);
    printf("\e[1;36mEuclidVK\e[0;37m: Enabled device extensions count = %d\n", extensionCount);
    for(int i = 0; i != extensionCount; i++){
        printf("\e[1;36mEuclidVK\e[0;37m: Enabled device extension(%d) %s\n", i, extprop[i].extensionName);
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
    printf("\e[1;36mEuclidVK\e[0;37m: Device created with result = %d\n", result);
    vkGetDeviceQueue(euclid.handle[eh].device, euclid.handle[eh].chosenqueuefam, 0, &euclid.handle[eh].graphicsQueue);
    vkGetDeviceQueue(euclid.handle[eh].device, euclid.handle[eh].chosenpresentqueue, 0, &euclid.handle[eh].presentQueue);
    printf("\e[1;36mEuclidVK\e[0;37m: Chosen present queue = %d\n", euclid.handle[eh].chosenpresentqueue);
    printf("\e[1;36mEuclidVK\e[0;37m: Chosen queue family with result = %d\n", euclid.handle[eh].chosenqueuefam);
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
            printf("\e[1;36mEuclidVK\e[0;37m: Avaible format = %d avaible color space = %d\n", formats[i].format, formats[i].colorSpace);
            if (formats[i].format == VK_FORMAT_B8G8R8A8_SRGB && formats[i].colorSpace == VK_COLOR_SPACE_SRGB_NONLINEAR_KHR) {
                chosenFormat = i;
                break;
            }
        }
    }else{
        printf("\e[1;31mError\e[0;37m: No formats avaible");
        exit(-1);
    }

    uint32_t presentModeCount;
    VkPresentModeKHR *modes;
    vkGetPhysicalDeviceSurfacePresentModesKHR(euclid.handle[eh].physicalDevices[euclid.handle[eh].chosenDevice], euclid.handle[eh].surface, &presentModeCount, NULL);
    if (presentModeCount != 0) {
        modes = malloc(sizeof(VkPresentModeKHR)*presentModeCount);
        vkGetPhysicalDeviceSurfacePresentModesKHR(euclid.handle[eh].physicalDevices[euclid.handle[eh].chosenDevice], euclid.handle[eh].surface, &presentModeCount, modes);
        for(int i = 0; i != presentModeCount; i++){
            printf("\e[1;36mEuclidVK\e[0;37m: Present mode avaible = %d\n", modes[i]);
        }
    }else{
        printf("\e[1;31mError\e[0;37m: No present mode avaible");
        exit(-1);
    }

    uint32_t imageCount = capabilities.minImageCount+1;
    if (capabilities.maxImageCount > 0 && imageCount > capabilities.maxImageCount) {
        imageCount = capabilities.maxImageCount;
    }

    printf("\e[1;36mEuclidVK\e[0;37m: SwapChain image count = %d\n", imageCount);

    VkSwapchainCreateInfoKHR createInfo = {0};
    createInfo.sType = VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;
    createInfo.surface = euclid.handle[eh].surface;
    createInfo.minImageCount = imageCount;
    createInfo.imageFormat = formats[chosenFormat].format;
    createInfo.imageColorSpace = formats[chosenFormat].colorSpace;
    createInfo.imageExtent.width = euclid.handle[eh].resolutionX;
    createInfo.imageExtent.height = euclid.handle[eh].resolutionY;
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
    printf("\e[1;36mEuclidVK\e[0;37m: SwapChain created with result = %d\n", result);
    vkGetSwapchainImagesKHR(euclid.handle[eh].device, euclid.handle[eh].swapChain, &imageCount, NULL);
    euclid.handle[eh].swapChainImages = malloc(sizeof(VkImage)*imageCount);
    vkGetSwapchainImagesKHR(euclid.handle[eh].device, euclid.handle[eh].swapChain, &imageCount, euclid.handle[eh].swapChainImages);
    euclid.handle[eh].swapChainImageFormat = formats[chosenFormat].format;
    euclid.handle[eh].swapChainExtent = capabilities.currentExtent;
    euclid.handle[eh].swapChainImageCount = imageCount;
    euclid.handle[eh].oldx = euclid.handle[eh].resolutionX;
    euclid.handle[eh].oldy = euclid.handle[eh].resolutionY;
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
        printf("\e[1;36mEuclidVK\e[0;37m: SwapChain Image View %d created with result = %d\n", i, result);
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
    printf("\e[1;36mEuclidVK\e[0;37m: Renderpass created with result = %d\n", result);
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
    depthCreateInfo.extent.width = euclid.handle[eh].resolutionX;
    depthCreateInfo.extent.height = euclid.handle[eh].resolutionY;
    depthCreateInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
    depthCreateInfo.samples = VK_SAMPLE_COUNT_1_BIT;
    depthCreateInfo.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    VkResult result = vkCreateImage(euclid.handle[eh].device, &depthCreateInfo, NULL, &euclid.handle[eh].depthImage);
    printf("\e[1;36mEuclidVK\e[0;37m: depth image created with result = %d\n", result);

    VkMemoryRequirements memRequirements;
    vkGetImageMemoryRequirements(euclid.handle[eh].device, euclid.handle[eh].depthImage, &memRequirements);

    VkMemoryAllocateInfo allocInfo = {0};
    allocInfo.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    allocInfo.allocationSize = memRequirements.size;
    allocInfo.memoryTypeIndex = findMemoryType(memRequirements.memoryTypeBits, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT, eh);

    result = vkAllocateMemory(euclid.handle[eh].device, &allocInfo, NULL, &euclid.handle[eh].depthImageMemory);
    printf("\e[1;36mEuclidVK\e[0;37m: depth image memory allocated with result = %d\n", result);

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
    printf("\e[1;36mEuclidVK\e[0;37m: depth imageview created with result = %d\n", result);

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
        framebufferInfo.width = euclid.handle[eh].resolutionX;
        framebufferInfo.height = euclid.handle[eh].resolutionY;
        framebufferInfo.layers = 1;
    
        result = vkCreateFramebuffer(euclid.handle[eh].device, &framebufferInfo, NULL, &euclid.handle[eh].swapChainFramebuffers[i]);
        printf("\e[1;36mEuclidVK\e[0;37m: SwapChain framebuffers created with result = %d\n", result);
    }
}

void createCommandPool(uint32_t eh){
    VkCommandPoolCreateInfo poolInfo = {0};
    poolInfo.sType = VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;
    poolInfo.flags = VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT;
    poolInfo.queueFamilyIndex = euclid.handle[eh].chosenqueuefam;

    VkResult result = vkCreateCommandPool( euclid.handle[eh].device, &poolInfo, NULL, & euclid.handle[eh].commandPool);
    printf("\e[1;36mEuclidVK\e[0;37m: Command pool created with result = %d\n", result);
}

void createCommandBuffer(uint32_t eh){
    euclid.handle[eh].commandBuffers = malloc(sizeof(VkCommandBuffer)*MAX_FRAMES_IN_FLIGHT);
    for(int i = 0; i != MAX_FRAMES_IN_FLIGHT; i++){
        VkCommandBufferAllocateInfo allocInfo = {0};
        allocInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
        allocInfo.commandPool =  euclid.handle[eh].commandPool;
        allocInfo.level = VK_COMMAND_BUFFER_LEVEL_PRIMARY;
        allocInfo.commandBufferCount = 1;

        VkResult result = vkAllocateCommandBuffers( euclid.handle[eh].device, &allocInfo, & euclid.handle[eh].commandBuffers[i]);
        printf("\e[1;36mEuclidVK\e[0;37m: Command buffer created with result = %d\n", result);
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
        printf("\e[1;36mEuclidVK\e[0;37m: imageAvailableSemaphore created with result = %d\n", result);
        result = vkCreateSemaphore(euclid.handle[eh].device, &semaphoreInfo, NULL, &euclid.handle[eh].renderFinishedSemaphores[i]);
        printf("\e[1;36mEuclidVK\e[0;37m: renderFinishedSemaphore created with result = %d\n", result);
        result = vkCreateFence(euclid.handle[eh].device, &fenceInfo, NULL, &euclid.handle[eh].inFlightFences[i]);
        printf("\e[1;36mEuclidVK\e[0;37m: inFlightFence created with result = %d\n", result);
    }
}

void recreateSwapChain(uint32_t eh){
    vkDeviceWaitIdle(euclid.handle[eh].device);
    for(int i = 0; i != euclid.handle[eh].swapChainImageCount; i++){
        vkDestroyFramebuffer(euclid.handle[eh].device, euclid.handle[eh].swapChainFramebuffers[i], NULL);
    }
    for(int i = 0; i != euclid.handle[eh].swapChainImageCount; i++){
        vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].swapChainImageViews[i], NULL);
    }
    free(euclid.handle[eh].swapChainFramebuffers);
    free(euclid.handle[eh].swapChainImageViews);
    free(euclid.handle[eh].swapChainImages);
    vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].depthImageView, NULL);
    vkDestroyImage(euclid.handle[eh].device, euclid.handle[eh].depthImage, NULL);
    vkFreeMemory(euclid.handle[eh].device, euclid.handle[eh].depthImageMemory, NULL);
    vkDestroySwapchainKHR(euclid.handle[eh].device, euclid.handle[eh].swapChain, NULL);
    createSwapChain(eh);
    createSwapChainImageViews(eh);
    createFrameBuffers(eh);
}

void startrender(uint32_t eh){
    vkWaitForFences(euclid.handle[eh].device, 1, &euclid.handle[eh].inFlightFences[euclid.handle[eh].currentFrame], VK_TRUE, UINT64_MAX);
    vkResetFences(euclid.handle[eh].device, 1, &euclid.handle[eh].inFlightFences[euclid.handle[eh].currentFrame]);

    VkResult result = vkAcquireNextImageKHR(euclid.handle[eh].device, euclid.handle[eh].swapChain, UINT64_MAX, euclid.handle[eh].imageAvailableSemaphores[euclid.handle[eh].currentFrame], VK_NULL_HANDLE, &euclid.handle[eh].imageIndex);
    if (result == VK_ERROR_OUT_OF_DATE_KHR || euclid.handle[eh].oldx != euclid.handle[eh].resolutionX || euclid.handle[eh].oldy != euclid.handle[eh].resolutionY) {
        printf("\e[1;36mEuclidVk\e[0;37m: Resolution changed from %dx%d to %dx%d\n", euclid.handle[eh].oldx, euclid.handle[eh].oldy, euclid.handle[eh].resolutionX, euclid.handle[eh].resolutionY);
        recreateSwapChain(eh);
        euclid.handle[eh].oldx = euclid.handle[eh].resolutionX;
        euclid.handle[eh].oldy = euclid.handle[eh].resolutionY;
    }

    vkResetCommandBuffer(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame], 0);

    VkCommandBufferBeginInfo beginInfo = {0};
    beginInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
    beginInfo.flags = 0;
    beginInfo.pInheritanceInfo = NULL;
    vkBeginCommandBuffer(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame], &beginInfo);

    VkRenderPassBeginInfo renderPassInfo = {0};
    renderPassInfo.sType = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO;
    renderPassInfo.renderPass = euclid.handle[eh].renderPass;
    renderPassInfo.framebuffer = euclid.handle[eh].swapChainFramebuffers[euclid.handle[eh].imageIndex];
    renderPassInfo.renderArea.offset.x = 0;
    renderPassInfo.renderArea.offset.y = 0;
    renderPassInfo.renderArea.extent.width = euclid.handle[eh].resolutionX;
    renderPassInfo.renderArea.extent.height = euclid.handle[eh].resolutionY;
    VkClearValue clearValues[2] = {0};
    clearValues[0].color.float32[0] = 0.0;
    clearValues[0].color.float32[1] = 0.0;
    clearValues[0].color.float32[2] = 0.0;
    clearValues[0].color.float32[3] = 1.0;
    clearValues[1].depthStencil.depth = 1.0;
    clearValues[1].depthStencil.stencil = 0.0;    
    renderPassInfo.clearValueCount = 2;
    renderPassInfo.pClearValues = clearValues;
    vkCmdBeginRenderPass(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame], &renderPassInfo, VK_SUBPASS_CONTENTS_INLINE);
}

void endrender(uint32_t eh){
    vkCmdEndRenderPass(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame]);
    vkEndCommandBuffer(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame]);

    VkSubmitInfo submitInfo = {0};
    submitInfo.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO;

    VkPipelineStageFlags waitStages[] = {VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT};
    submitInfo.waitSemaphoreCount = 1;
    submitInfo.pWaitSemaphores = &euclid.handle[eh].imageAvailableSemaphores[euclid.handle[eh].currentFrame];
    submitInfo.pWaitDstStageMask = waitStages;

    submitInfo.commandBufferCount = 1;
    submitInfo.pCommandBuffers = &euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame];

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

uint32_t neweng(){
    uint32_t eh = euclid.size;
    if(euclid.size != 0){
        euclidh *tmp = malloc(sizeof(euclidh)*euclid.size);
        memcpy(tmp, euclid.handle, sizeof(euclidh)*euclid.size);
        free(euclid.handle);
        euclid.size++;
        euclid.handle = malloc(sizeof(euclidh)*euclid.size);
        memcpy(euclid.handle, tmp, sizeof(euclidh)*(euclid.size-1));
        free(tmp);
    }else{
        euclid.size++;
        euclid.handle = malloc(sizeof(euclidh)*euclid.size);
    }
    euclid.handle[eh].chosenDevice = -1;
    euclid.handle[eh].usedPresentMode = 0;
    createInstance(eh);
    getDevice(eh);
    glfwInit();
    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    euclid.handle[eh].window = glfwCreateWindow(800, 600, "Schnellwerke 3", NULL, NULL);
    glfwGetFramebufferSize(euclid.handle[eh].window, &euclid.handle[eh].resolutionX, &euclid.handle[eh].resolutionY);
    glfwCreateWindowSurface(euclid.handle[eh].instance, euclid.handle[eh].window, NULL, &euclid.handle[eh].surface);
    getPresentFamily(eh);
    createDevice(eh);
    createSwapChain(eh);
    createSwapChainImageViews(eh);
    createRenderPass(eh);
    createFrameBuffers(eh);
    createCommandPool(eh);
    createCommandBuffer(eh);
    createSyncObjects(eh);
    euclid.handle[eh].currentFrame = 0;
    euclid.handle[eh].imageIndex = 0;
    euclid.handle[eh].totalFrames = 0;
    return eh;
}

void draw(uint32_t eh, uint32_t eme){
    vkCmdBindPipeline(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame], VK_PIPELINE_BIND_POINT_GRAPHICS, euclid.meshes[eme].graphicsPipeline);

    VkDeviceSize offsets[] = {0};
    vkCmdBindVertexBuffers(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame], 0, 1, &euclid.models[euclid.meshes[eme].modelId].vertexBuffer, offsets);

    VkViewport viewport = {0};
    viewport.x = 0.0f;
    viewport.y = 0.0f;
    viewport.width = euclid.handle[eh].resolutionX;
    viewport.height = euclid.handle[eh].resolutionY;
    viewport.minDepth = 0.0f;
    viewport.maxDepth = 1.0f;
    vkCmdSetViewport(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame], 0, 1, &viewport);

    VkRect2D scissor = {0};
    scissor.offset.x = 0;
    scissor.offset.y = 0;
    scissor.extent = euclid.handle[eh].swapChainExtent;
    vkCmdSetScissor(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame], 0, 1, &scissor);

    euclid.meshes[eme].lub[0] = (float) euclid.handle[eh].resolutionX;
    euclid.meshes[eme].lub[1] = (float) euclid.handle[eh].resolutionY;
    euclid.meshes[eme].lub[2] = (float) euclid.handle[eh].shadowMapResolution;
    euclid.meshes[eme].lub[3] = (float) euclid.handle[eh].totalFrames;
    memcpy(euclid.meshes[eme].uniformBuffersMapped[euclid.handle[eh].currentFrame], euclid.meshes[eme].lub, sizeof(euclid.meshes[eme].lub));
    vkCmdBindDescriptorSets(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame], VK_PIPELINE_BIND_POINT_GRAPHICS, euclid.meshes[eme].pipelineLayout, 0, 1, &euclid.meshes[eme].descriptorSets[euclid.handle[eh].currentFrame], 0, NULL);

    vkCmdDraw(euclid.handle[eh].commandBuffers[euclid.handle[eh].currentFrame], euclid.models[euclid.meshes[eme].modelId].vertnum, 1, 0, 0);
}

uint32_t newmaterial(uint32_t eh, uint32_t *vert, uint32_t *frag, uint32_t svert, uint32_t sfrag, uint32_t cullmode){
    uint32_t em = euclid.msize;
    if(euclid.msize != 0){
        euclidh *tmp = malloc(sizeof(euclidmaterial)*euclid.msize);
        memcpy(tmp, euclid.materials, sizeof(euclidmaterial)*euclid.msize);
        free(euclid.materials);
        euclid.msize++;
        euclid.materials = malloc(sizeof(euclidmaterial)*euclid.msize);
        memcpy(euclid.materials, tmp, sizeof(euclidmaterial)*(euclid.msize-1));
        free(tmp);
    }else{
        euclid.msize++;
        euclid.materials = malloc(sizeof(euclidmaterial)*euclid.msize);
    }

    VkShaderModuleCreateInfo vcreateInfo = {0};
    vcreateInfo.sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
    vcreateInfo.codeSize = svert;
    vcreateInfo.pCode = (uint32_t*) &vert[0];
    VkResult result = vkCreateShaderModule(euclid.handle[eh].device, &vcreateInfo, NULL, &euclid.materials[em].vertModule);
    printf("\e[1;36mEuclidMT\e[0;37m: Vertex shader module created with result = %d\n", result);

    VkShaderModuleCreateInfo fcreateInfo = {0};
    fcreateInfo.sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
    fcreateInfo.codeSize = sfrag;
    fcreateInfo.pCode = (uint32_t*) &frag[0];
    result = vkCreateShaderModule(euclid.handle[eh].device, &fcreateInfo, NULL, &euclid.materials[em].fragModule);
    printf("\e[1;36mEuclidMT\e[0;37m: Fragment shader module created with result = %d\n", result);

    euclid.materials[em].cullMode = cullmode;
    euclid.materials[em].polygonMode = VK_POLYGON_MODE_FILL;
    euclid.materials[em].lineWidth = 1.0;

    printf("\e[1;36mEuclidMT\e[0;37m: Shader module created by id = %d\n", em);
    return em;
}

uint32_t newmodel(uint32_t eh, float *vertices, float *uv, float *normals, uint32_t size){
    uint32_t em = euclid.mosize;
    if(euclid.mosize != 0){
        euclidh *tmp = malloc(sizeof(euclidmodel)*euclid.mosize);
        memcpy(tmp, euclid.models, sizeof(euclidmodel)*euclid.mosize);
        free(euclid.models);
        euclid.mosize++;
        euclid.models = malloc(sizeof(euclidmodel)*euclid.mosize);
        memcpy(euclid.models, tmp, sizeof(euclidmodel)*(euclid.mosize-1));
        free(tmp);
    }else{
        euclid.mosize++;
        euclid.models = malloc(sizeof(euclidmodel)*euclid.mosize);
    }

    float *tg = malloc(sizeof(float)*size*3);
    float *ctg = malloc(sizeof(float)*size*3);
    for (uint32_t i = 0, u = 0; i < size*3; i+=9, u+=6){
        float v0[] = { vertices[i], vertices[i+1], vertices[i+2] };
        float v1[] = { vertices[i+3], vertices[i+4], vertices[i+5] };
        float v2[] = { vertices[i+6], vertices[i+7], vertices[i+8] };
        float uv0[] = { uv[u], uv[u+1]+1.0f };
        float uv1[] = { uv[u+2], uv[u+3]+1.0f };
        float uv2[] = { uv[u+4], uv[u+5]+1.0f };
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
    float *res = malloc(sizeof(float)*size*14);
    for(uint32_t i = 0, v = 0, u = 0; i < size*14; i+=14, v+=3, u+=2){
        res[i] = vertices[v];
        res[i+1] = vertices[v+1];
        res[i+2] = vertices[v+2];
        res[i+3] = uv[u];
        res[i+4] = uv[u+1];
        res[i+5] = normals[v];
        res[i+6] = normals[v+1];
        res[i+7] = normals[v+2];
        res[i+8] = tg[v];
        res[i+9] = tg[v+1];
        res[i+10] = tg[v+2];
        res[i+11] = ctg[v];
        res[i+12] = ctg[v+1];
        res[i+13] = ctg[v+2];
    }

    euclid.models[em].vertnum = size;
    VkBufferCreateInfo bufferInfo = {0};
    bufferInfo.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
    bufferInfo.size = sizeof(res[0]) * size*14;
    bufferInfo.usage = VK_BUFFER_USAGE_VERTEX_BUFFER_BIT;
    bufferInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
    VkResult result = vkCreateBuffer(euclid.handle[eh].device, &bufferInfo, NULL, &euclid.models[em].vertexBuffer);
    printf("\e[1;36mEuclidMD\e[0;37m: Vertex buffer created with result = %d\n", result);
    VkMemoryRequirements memRequirements;
    vkGetBufferMemoryRequirements(euclid.handle[eh].device, euclid.models[em].vertexBuffer, &memRequirements);

    VkMemoryAllocateInfo allocInfo = {0};
    allocInfo.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    allocInfo.allocationSize = memRequirements.size;
    allocInfo.memoryTypeIndex = findMemoryType(memRequirements.memoryTypeBits, VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT, eh);

    vkAllocateMemory(euclid.handle[eh].device, &allocInfo, NULL, &euclid.models[em].vertexBufferMemory);
    vkBindBufferMemory(euclid.handle[eh].device, euclid.models[em].vertexBuffer, euclid.models[em].vertexBufferMemory, 0);
    void* data;
    vkMapMemory(euclid.handle[eh].device, euclid.models[em].vertexBufferMemory, 0, bufferInfo.size, 0, &data);
        memcpy(data, res, (size_t) bufferInfo.size);
    vkUnmapMemory(euclid.handle[eh].device, euclid.models[em].vertexBufferMemory);
    free(tg);
    free(ctg);
    free(res);
    return em;
}

void createDescriptorSetLayout(uint32_t eh, uint32_t eme) {
    VkDescriptorSetLayoutBinding uboLayoutBinding[2] = {0};
    uboLayoutBinding[0].binding = 0;
    uboLayoutBinding[0].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    uboLayoutBinding[0].descriptorCount = 1;
    uboLayoutBinding[0].stageFlags = VK_SHADER_STAGE_VERTEX_BIT | VK_SHADER_STAGE_FRAGMENT_BIT;
    uboLayoutBinding[0].pImmutableSamplers = NULL;

    uboLayoutBinding[1].binding = 1;
    uboLayoutBinding[1].descriptorType = VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER;
    uboLayoutBinding[1].descriptorCount = 1;
    uboLayoutBinding[1].stageFlags = VK_SHADER_STAGE_FRAGMENT_BIT;
    uboLayoutBinding[1].pImmutableSamplers = NULL;

    VkDescriptorSetLayoutCreateInfo layoutInfo = {0};
    layoutInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO;
    layoutInfo.bindingCount = 2;
    layoutInfo.pBindings = uboLayoutBinding;
    VkResult result = vkCreateDescriptorSetLayout(euclid.handle[eh].device, &layoutInfo, NULL, &euclid.meshes[eme].descriptorSetLayout);
    printf("\e[1;36mEuclidMS\e[0;37m: Descriptor set layout created with result = %d\n", result);
}

void createUniformBuffer(uint32_t eh, uint32_t eme){
    VkDeviceSize bufferSize = 96;

    euclid.meshes[eme].uniformBuffers = malloc(sizeof(VkBuffer)*MAX_FRAMES_IN_FLIGHT);
    euclid.meshes[eme].uniformBuffersMemory = malloc(sizeof(VkDeviceMemory)*MAX_FRAMES_IN_FLIGHT);
    euclid.meshes[eme].uniformBuffersMapped = malloc(sizeof(void*)*MAX_FRAMES_IN_FLIGHT);
    for (size_t i = 0; i < MAX_FRAMES_IN_FLIGHT; i++) {
        VkBufferCreateInfo bufferInfo = {0};
        bufferInfo.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        bufferInfo.size = bufferSize;
        bufferInfo.usage = VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT;
        bufferInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
        VkResult result = vkCreateBuffer(euclid.handle[eh].device, &bufferInfo, NULL, &euclid.meshes[eme].uniformBuffers[i]);
        printf("\e[1;36mEuclidMD\e[0;37m: Uniform buffer created with result = %d\n", result);
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
    VkDescriptorPoolSize poolSize[2] = {0};
    poolSize[0].type = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    poolSize[0].descriptorCount = MAX_FRAMES_IN_FLIGHT;

    poolSize[1].type = VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER;
    poolSize[1].descriptorCount = MAX_FRAMES_IN_FLIGHT;

    VkDescriptorPoolCreateInfo poolInfo = {0};
    poolInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO;
    poolInfo.poolSizeCount = 2;
    poolInfo.pPoolSizes = poolSize;
    poolInfo.maxSets = MAX_FRAMES_IN_FLIGHT;

    VkResult result = vkCreateDescriptorPool(euclid.handle[eh].device, &poolInfo, NULL, &euclid.meshes[eme].descriptorPool);
    printf("\e[1;36mEuclidMS\e[0;37m: Descriptor pool created with result = %d\n", result);
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
    printf("\e[1;36mEuclidMS\e[0;37m: Descriptor sets allocated with result = %d\n", result);

    for (size_t i = 0; i < MAX_FRAMES_IN_FLIGHT; i++) {
        VkDescriptorBufferInfo bufferInfo = {0};
        bufferInfo.buffer = euclid.meshes[eme].uniformBuffers[i];
        bufferInfo.offset = 0;
        bufferInfo.range = 96;

        VkDescriptorImageInfo imageInfo = {0};
        imageInfo.imageLayout = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
        imageInfo.imageView = euclid.textures[euclid.meshes[eme].texid].textureImageView;
        imageInfo.sampler = euclid.textures[euclid.meshes[eme].texid].sampler;

        VkWriteDescriptorSet descriptorWrite[2] = {0};
        descriptorWrite[0].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[0].dstSet = euclid.meshes[eme].descriptorSets[i];
        descriptorWrite[0].dstBinding = 0;
        descriptorWrite[0].dstArrayElement = 0;
        descriptorWrite[0].descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
        descriptorWrite[0].descriptorCount = 1;
        descriptorWrite[0].pBufferInfo = &bufferInfo;
        descriptorWrite[0].pImageInfo = NULL;
        descriptorWrite[0].pTexelBufferView = NULL;

        descriptorWrite[1].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite[1].dstSet = euclid.meshes[eme].descriptorSets[i];
        descriptorWrite[1].dstBinding = 1;
        descriptorWrite[1].dstArrayElement = 0;
        descriptorWrite[1].descriptorType = VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER;
        descriptorWrite[1].descriptorCount = 1;
        descriptorWrite[1].pBufferInfo = NULL;
        descriptorWrite[1].pImageInfo = &imageInfo;
        descriptorWrite[1].pTexelBufferView = NULL;

        vkUpdateDescriptorSets(euclid.handle[eh].device, 2, descriptorWrite, 0, NULL);
    }
    free(ldcs);
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
    viewport.width = (float) euclid.handle[eh].resolutionX;
    viewport.height = (float) euclid.handle[eh].resolutionY;
    viewport.minDepth = 0.0f;
    viewport.maxDepth = 1.0f;

    VkRect2D scissor = {0};
    scissor.offset.x = 0;
    scissor.offset.y = 0;
    scissor.extent.width = euclid.handle[eh].resolutionX;
    scissor.extent.height = euclid.handle[eh].resolutionY;

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
    colorBlendAttachment.dstColorBlendFactor = VK_BLEND_FACTOR_ZERO;
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
    printf("\e[1;36mEuclidMS\e[0;37m: Pipeline layout created with result = %d\n", result);

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
    printf("\e[1;36mEuclidMS\e[0;37m: Pipeline created with result = %d\n", result);
}

uint32_t newmesh(uint32_t eh, uint32_t es, uint32_t em, uint32_t te){
    uint32_t eme = euclid.mesize;
    if(euclid.mesize != 0){
        euclidh *tmp = malloc(sizeof(euclidmesh)*euclid.mesize);
        memcpy(tmp, euclid.meshes, sizeof(euclidmesh)*euclid.mesize);
        free(euclid.meshes);
        euclid.mesize++;
        euclid.meshes = malloc(sizeof(euclidmesh)*euclid.mesize);
        memcpy(euclid.meshes, tmp, sizeof(euclidmesh)*(euclid.mesize-1));
        free(tmp);
    }else{
        euclid.mesize++;
        euclid.meshes = malloc(sizeof(euclidmesh)*euclid.mesize);
    }
    euclid.meshes[eme].euclidid = eh;
    euclid.meshes[eme].drawable = 1;
    euclid.meshes[eme].texid = te;
    createUniformBuffer(eh, eme);
    createDescriptorPool(eh, eme);
    createDescriptorSetLayout(eh, eme);
    createDescriptorSets(eh, eme);
    createPipeline(eh, eme, es, em);
    return eme;
}

void setmeshbuf(uint32_t eme, uint32_t i, float val){
    euclid.meshes[eme].lub[i+4] = val;
}

uint32_t loopcont(uint32_t eh){
    glfwGetFramebufferSize(euclid.handle[eh].window, &euclid.handle[eh].resolutionX, &euclid.handle[eh].resolutionY);
    startrender(eh);
    for(uint32_t i = 0; i != euclid.mesize; i++){
        if(euclid.meshes[i].euclidid == eh && euclid.meshes[i].drawable == 1){
            draw(eh, i);
        }
    }
    endrender(eh);
    glfwPollEvents();
    return !glfwWindowShouldClose(euclid.handle[eh].window);
}

VkCommandBuffer beginSingleTimeCommands(uint32_t eh) {
    VkCommandBufferAllocateInfo allocInfo = {0};
    allocInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
    allocInfo.level = VK_COMMAND_BUFFER_LEVEL_PRIMARY;
    allocInfo.commandPool = euclid.handle[eh].commandPool;
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

    vkFreeCommandBuffers(euclid.handle[eh].device, euclid.handle[eh].commandPool, 1, &commandBuffer);
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
            blit.srcSubresource.baseArrayLayer = 0;
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

uint32_t newtexture(uint32_t eh, uint32_t xsize, uint32_t ysize, uint32_t zsize, char *pixels){
    uint32_t te = euclid.tsize;
    if(euclid.tsize != 0){
        euclidh *tmp = malloc(sizeof(euclidtexture)*euclid.tsize);
        memcpy(tmp, euclid.textures, sizeof(euclidtexture)*euclid.tsize);
        free(euclid.textures);
        euclid.tsize++;
        euclid.textures = malloc(sizeof(euclidtexture)*euclid.tsize);
        memcpy(euclid.textures, tmp, sizeof(euclidtexture)*(euclid.tsize-1));
        free(tmp);
    }else{
        euclid.tsize++;
        euclid.textures = malloc(sizeof(euclidtexture)*euclid.tsize);
    }

    VkBuffer stagingBuffer;
    VkDeviceMemory stagingBufferMemory;

    VkDeviceSize imageSize = xsize * ysize * zsize * 4;

    VkBufferCreateInfo bufferInfo = {0};
    bufferInfo.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
    bufferInfo.size = imageSize;
    bufferInfo.usage = VK_BUFFER_USAGE_TRANSFER_SRC_BIT;
    bufferInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
    VkResult result = vkCreateBuffer(euclid.handle[eh].device, &bufferInfo, NULL, &stagingBuffer);
    printf("\e[1;36mEuclidTEX\e[0;37m: Staging buffer created with result = %d\n", result);
    VkMemoryRequirements memRequirements;
    vkGetBufferMemoryRequirements(euclid.handle[eh].device, stagingBuffer, &memRequirements);
    VkMemoryAllocateInfo allocInfo = {0};
    allocInfo.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    allocInfo.allocationSize = memRequirements.size;
    allocInfo.memoryTypeIndex = findMemoryType(memRequirements.memoryTypeBits, VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT, eh);
    vkAllocateMemory(euclid.handle[eh].device, &allocInfo, NULL, &stagingBufferMemory);

    void* data;
    vkMapMemory(euclid.handle[eh].device, stagingBufferMemory, 0, imageSize, 0, &data);
        memcpy(data, pixels, imageSize);
    vkUnmapMemory(euclid.handle[eh].device, stagingBufferMemory);

    vkBindBufferMemory(euclid.handle[eh].device, stagingBuffer, stagingBufferMemory, 0);

    euclid.textures[te].mipLevels = floor(log2(fmaxf(xsize, ysize)))+1;

    VkImageCreateInfo imageInfo = {0};
    imageInfo.sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
    imageInfo.imageType = VK_IMAGE_TYPE_2D;
    imageInfo.extent.width = xsize;
    imageInfo.extent.height = ysize;
    imageInfo.extent.depth = 1;
    imageInfo.mipLevels = euclid.textures[te].mipLevels;
    imageInfo.arrayLayers = zsize;
    imageInfo.format = VK_FORMAT_R8G8B8A8_SRGB;
    imageInfo.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    imageInfo.usage = VK_IMAGE_USAGE_TRANSFER_DST_BIT | VK_IMAGE_USAGE_SAMPLED_BIT;
    imageInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
    imageInfo.samples = VK_SAMPLE_COUNT_1_BIT;
    imageInfo.flags = 0;
    result = vkCreateImage(euclid.handle[eh].device, &imageInfo, NULL, &euclid.textures[te].texture);
    printf("\e[1;36mEuclidTEX\e[0;37m: Texture created with result = %d\n", result);

    VkMemoryRequirements memRequirementsi;
    vkGetImageMemoryRequirements(euclid.handle[eh].device, euclid.textures[te].texture, &memRequirementsi);
    
    VkMemoryAllocateInfo allocInfoi = {0};
    allocInfoi.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    allocInfoi.allocationSize = memRequirementsi.size;
    allocInfoi.memoryTypeIndex = findMemoryType(memRequirementsi.memoryTypeBits, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT, eh);
    
    result = vkAllocateMemory(euclid.handle[eh].device, &allocInfoi, NULL, &euclid.textures[te].textureImageMemory);
    printf("\e[1;36mEuclidTEX\e[0;37m: Texture memory alocated with result = %d\n", result);

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
    barrier.subresourceRange.layerCount = zsize;
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
    region.imageSubresource.layerCount = zsize;

    region.imageOffset.x = 0;
    region.imageOffset.y = 0;
    region.imageOffset.z = 0;
    region.imageExtent.width = xsize;
    region.imageExtent.height = ysize;
    region.imageExtent.depth = 1;

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

    generateMipmaps(euclid.textures[te].texture, xsize, ysize, euclid.textures[te].mipLevels, zsize, eh);

    VkImageViewCreateInfo viewInfo = {0};
    viewInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
    viewInfo.image = euclid.textures[te].texture;
    viewInfo.viewType = VK_IMAGE_VIEW_TYPE_2D_ARRAY;
    viewInfo.format = VK_FORMAT_R8G8B8A8_SRGB;
    viewInfo.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
    viewInfo.subresourceRange.baseMipLevel = 0;
    viewInfo.subresourceRange.levelCount = euclid.textures[te].mipLevels;
    viewInfo.subresourceRange.baseArrayLayer = 0;
    viewInfo.subresourceRange.layerCount = zsize;

    result = vkCreateImageView(euclid.handle[eh].device, &viewInfo, NULL, &euclid.textures[te].textureImageView);
    printf("\e[1;36mEuclidTEX\e[0;37m: Texture view created with result = %d\n", result);

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
    samplerInfo.maxLod = 0.0f;

    result = vkCreateSampler(euclid.handle[eh].device, &samplerInfo, NULL, &euclid.textures[te].sampler);
    printf("\e[1;36mEuclidTEX\e[0;37m: Sampler created with result = %d\n", result);

    return te;
}

void destroy(uint32_t eh){
    vkDeviceWaitIdle(euclid.handle[eh].device);
    for(uint32_t i = 0; i != euclid.mesize; i++){
        vkDestroyPipeline(euclid.handle[eh].device, euclid.meshes[i].graphicsPipeline, NULL);
        vkDestroyDescriptorSetLayout(euclid.handle[eh].device, euclid.meshes[i].descriptorSetLayout, NULL);
        vkDestroyPipelineLayout(euclid.handle[eh].device, euclid.meshes[i].pipelineLayout, NULL);
        for(uint32_t j = 0; j != MAX_FRAMES_IN_FLIGHT; j++){
            vkDestroyBuffer(euclid.handle[eh].device, euclid.meshes[i].uniformBuffers[j], NULL);
            vkFreeMemory(euclid.handle[eh].device, euclid.meshes[i].uniformBuffersMemory[j], NULL);
        }
        free(euclid.meshes[i].uniformBuffers);
        free(euclid.meshes[i].uniformBuffersMemory);
        free(euclid.meshes[i].uniformBuffersMapped);
        vkDestroyDescriptorPool(euclid.handle[eh].device, euclid.meshes[i].descriptorPool, NULL);
        free(euclid.meshes[i].descriptorSets);
    }
    for(uint32_t i = 0; i != euclid.tsize; i++){
        vkDestroyImageView(euclid.handle[eh].device, euclid.textures[i].textureImageView, NULL);
        vkDestroyImage(euclid.handle[eh].device, euclid.textures[i].texture, NULL);
        vkFreeMemory(euclid.handle[eh].device, euclid.textures[i].textureImageMemory, NULL);
    }
    for(uint32_t i = 0; i != euclid.mosize; i++){
        vkDestroyBuffer(euclid.handle[eh].device, euclid.models[i].vertexBuffer, NULL);
        vkFreeMemory(euclid.handle[eh].device, euclid.models[i].vertexBufferMemory, NULL);
    }
    for(uint32_t i = 0; i != euclid.msize; i++){
        vkDestroyShaderModule(euclid.handle[eh].device, euclid.materials[i].fragModule, NULL);
        vkDestroyShaderModule(euclid.handle[eh].device, euclid.materials[i].vertModule, NULL);
    }
    for(int i = 0; i != MAX_FRAMES_IN_FLIGHT; i++){
        vkDestroySemaphore(euclid.handle[eh].device, euclid.handle[eh].imageAvailableSemaphores[i], NULL);
        vkDestroySemaphore(euclid.handle[eh].device, euclid.handle[eh].renderFinishedSemaphores[i], NULL);
        vkDestroyFence(euclid.handle[eh].device, euclid.handle[eh].inFlightFences[i], NULL);
    }
    vkFreeCommandBuffers(euclid.handle[eh].device, euclid.handle[eh].commandPool, MAX_FRAMES_IN_FLIGHT, euclid.handle[eh].commandBuffers);
    vkDestroyCommandPool(euclid.handle[eh].device, euclid.handle[eh].commandPool, NULL);
    for(int i = 0; i != euclid.handle[eh].swapChainImageCount; i++){
        vkDestroyFramebuffer(euclid.handle[eh].device, euclid.handle[eh].swapChainFramebuffers[i], NULL);
    }
    vkDestroyRenderPass(euclid.handle[eh].device, euclid.handle[eh].renderPass, NULL);
    for(int i = 0; i != euclid.handle[eh].swapChainImageCount; i++){
        vkDestroyImageView(euclid.handle[eh].device, euclid.handle[eh].swapChainImageViews[i], NULL);
    }
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
    printf("\e[1;36mEuclidVK\e[0;37m: Destroyed handle by id = %d\n", eh);
}