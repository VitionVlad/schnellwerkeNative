#include "euclid.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <vulkan/vulkan.h>
#include <GLFW/glfw3.h>
#include <GLFW/glfw3native.h>

const int MAX_FRAMES_IN_FLIGHT = 2;

typedef struct euclidh{
    VkInstance instance;
    VkPhysicalDevice *physicalDevices;
    unsigned int chosenDevice;
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
    unsigned int usedPresentMode;
    unsigned int resolutionX;
    unsigned int resolutionY;
    unsigned int oldx;
    unsigned int oldy;
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
} euclidh;

struct euclidVK{
    euclidh *handle;
    int size;
} euclid;

uint32_t findMemoryType(uint32_t typeFilter, VkMemoryPropertyFlags properties, unsigned int eh) {
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

void createInstance(unsigned int eh){
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

void getDevice(unsigned int eh){
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

void getPresentFamily(unsigned int eh){
    VkBool32 presentSupport = VK_FALSE;
    for(int i = 0; i != euclid.handle[eh].queueFamilyCount; i++){
        vkGetPhysicalDeviceSurfaceSupportKHR(euclid.handle[eh].physicalDevices[euclid.handle[eh].chosenDevice], i, euclid.handle[eh].surface, &presentSupport);
        if(presentSupport == VK_TRUE){
            euclid.handle[eh].chosenpresentqueue = i;
            break;
        }
    }
}

void createDevice(unsigned int eh){
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

void createSwapChain(unsigned int eh){
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
    free(formats);
    free(modes);
}

void createSwapChainImageViews(unsigned int eh){
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

void createRenderPass(unsigned int eh){
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

void createFrameBuffers(unsigned int eh){
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

void createCommandPool(unsigned int eh){
    VkCommandPoolCreateInfo poolInfo = {0};
    poolInfo.sType = VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;
    poolInfo.flags = VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT;
    poolInfo.queueFamilyIndex = euclid.handle[eh].chosenqueuefam;

    VkResult result = vkCreateCommandPool( euclid.handle[eh].device, &poolInfo, NULL, & euclid.handle[eh].commandPool);
    printf("\e[1;36mEuclidVK\e[0;37m: Command pool created with result = %d\n", result);
}

void createCommandBuffer(unsigned int eh){
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

void createSyncObjects(unsigned int eh){
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

void recreateSwapChain(unsigned int eh){
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

void startrender(unsigned int eh){
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

void endrender(unsigned int eh){
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

unsigned int new(){
    unsigned int eh = euclid.size;
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

unsigned int loopcont(unsigned int eh){
    glfwGetFramebufferSize(euclid.handle[eh].window, &euclid.handle[eh].resolutionX, &euclid.handle[eh].resolutionY);
    startrender(eh);
    endrender(eh);
    glfwPollEvents();
    return !glfwWindowShouldClose(euclid.handle[eh].window);
}

void destroy(unsigned int eh){
    vkDeviceWaitIdle(euclid.handle[eh].device); 
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