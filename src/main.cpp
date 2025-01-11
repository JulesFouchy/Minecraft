#include <GLFW/glfw3.h>
#include <glfw3webgpu.h>
#include <webgpu/webgpu.h>
#include <cassert>
#include <iostream>
#include <vector>

// import std;

/**
 * Utility function to get a WebGPU adapter, so that
 *     WGPUAdapter adapter = requestAdapterSync(options);
 * is roughly equivalent to
 *     const adapter = await navigator.gpu.requestAdapter(options);
 */
WGPUAdapter requestAdapterSync(WGPUInstance instance, WGPURequestAdapterOptions const& options)
{
    // A simple structure holding the local information shared with the
    // onAdapterRequestEnded callback.
    struct UserData {
        WGPUAdapter adapter      = nullptr;
        bool        requestEnded = false;
    };
    UserData userData;

    // Callback called by wgpuInstanceRequestAdapter when the request returns
    // This is a C++ lambda function, but could be any function defined in the
    // global scope. It must be non-capturing (the brackets [] are empty) so
    // that it behaves like a regular C function pointer, which is what
    // wgpuInstanceRequestAdapter expects (WebGPU being a C API). The workaround
    // is to convey what we want to capture through the pUserData pointer,
    // provided as the last argument of wgpuInstanceRequestAdapter and received
    // by the callback as its last argument.
    auto onAdapterRequestEnded = [](WGPURequestAdapterStatus status, WGPUAdapter adapter, char const* message, void* pUserData) {
        UserData& userData = *reinterpret_cast<UserData*>(pUserData);
        if (status == WGPURequestAdapterStatus_Success)
        {
            userData.adapter = adapter;
        }
        else
        {
            std::cout << "Could not get WebGPU adapter: " << message << std::endl;
        }
        userData.requestEnded = true;
    };

    // Call to the WebGPU request adapter procedure
    wgpuInstanceRequestAdapter(
        instance /* equivalent of navigator.gpu */,
        &options,
        onAdapterRequestEnded,
        (void*)&userData
    );

#ifdef __EMSCRIPTEN__
    while (!userData.requestEnded)
    {
        emscripten_sleep(100);
    }
#endif // __EMSCRIPTEN__

    assert(userData.requestEnded);

    return userData.adapter;
}

/**
 * Utility function to get a WebGPU device, so that
 *     WGPUAdapter device = requestDeviceSync(adapter, options);
 * is roughly equivalent to
 *     const device = await adapter.requestDevice(descriptor);
 * It is very similar to requestAdapter
 */
WGPUDevice requestDeviceSync(WGPUAdapter adapter, WGPUDeviceDescriptor const* descriptor)
{
    struct UserData {
        WGPUDevice device       = nullptr;
        bool       requestEnded = false;
    };
    UserData userData;

    auto onDeviceRequestEnded = [](WGPURequestDeviceStatus status, WGPUDevice device, char const* message, void* pUserData) {
        UserData& userData = *reinterpret_cast<UserData*>(pUserData);
        if (status == WGPURequestDeviceStatus_Success)
        {
            userData.device = device;
        }
        else
        {
            std::cout << "Could not get WebGPU device: " << message << std::endl;
        }
        userData.requestEnded = true;
    };

    wgpuAdapterRequestDevice(
        adapter,
        descriptor,
        onDeviceRequestEnded,
        (void*)&userData
    );

#ifdef __EMSCRIPTEN__
    while (!userData.requestEnded)
    {
        emscripten_sleep(100);
    }
#endif // __EMSCRIPTEN__

    assert(userData.requestEnded);

    return userData.device;
}

auto main() -> int
{
    if (!glfwInit())
    {
        std::cerr << "Could not initialize GLFW!" << std::endl;
        return 1;
    }
    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE);
    GLFWwindow* window = glfwCreateWindow(640, 480, "Minecraft version améliorée", nullptr, nullptr);
    if (!window)
    {
        std::cerr << "Could not open window!" << std::endl;
        glfwTerminate();
        return 1;
    }

    WGPUInstanceDescriptor desc = {};
    desc.nextInChain            = nullptr;

#if defined(WEBGPU_BACKEND_DAWN) && DEBUG
    // Make sure the uncaptured error callback is called as soon as an error
    // occurs rather than at the next call to "wgpuDeviceTick".
    WGPUDawnTogglesDescriptor toggles;
    toggles.chain.next          = nullptr;
    toggles.chain.sType         = WGPUSType_DawnTogglesDescriptor;
    toggles.disabledToggleCount = 0;
    toggles.enabledToggleCount  = 1;
    const char* toggleName      = "enable_immediate_error_handling";
    toggles.enabledToggles      = &toggleName;

    desc.nextInChain = &toggles.chain;
#endif // WEBGPU_BACKEND_DAWN

    WGPUInstance instance = wgpuCreateInstance(&desc);

    // We can check whether there is actually an instance created
    if (!instance)
    {
        std::cerr << "Could not initialize WebGPU!" << std::endl;
        return 1;
    }

    // Display the object (WGPUInstance is a simple pointer, it may be
    // copied around without worrying about its size).
    std::cout << "WGPU instance: " << instance << std::endl;

    std::cout << "Requesting adapter..." << std::endl;

    auto                      surface     = glfwGetWGPUSurface(instance, window);
    WGPURequestAdapterOptions adapterOpts = {};
    adapterOpts.nextInChain               = nullptr;
    adapterOpts.compatibleSurface         = surface;
    // adapterOpts.
    WGPUAdapter adapter = requestAdapterSync(instance, adapterOpts);

    std::cout << "Got adapter: " << adapter << std::endl;
    wgpuInstanceRelease(instance);
    std::cout << "Requesting device..." << std::endl;

    WGPUDeviceDescriptor deviceDesc     = {};
    deviceDesc.nextInChain              = nullptr;
    deviceDesc.label                    = "WebGPU Device"; // anything works here, that's your call
    deviceDesc.requiredFeatureCount     = 0;               // we do not require any specific feature
    deviceDesc.requiredLimits           = nullptr;         // we do not require any specific limit
    deviceDesc.defaultQueue.nextInChain = nullptr;
    deviceDesc.defaultQueue.label       = "Default queue";
    // Null for now, see below
    // A function that is invoked whenever the device stops being available.
    deviceDesc.deviceLostCallback = [](WGPUDeviceLostReason reason, char const* message, void* /* pUserData */) {
        std::cout << "Device lost: reason " << reason;
        if (message)
            std::cout << " (" << message << ")";
        std::cout << std::endl;
    };
    WGPUDevice device = requestDeviceSync(adapter, &deviceDesc);

    WGPUSurfaceConfiguration config = {};
    config.nextInChain              = nullptr;
    // Configuration of the textures created for the underlying swap chain
    config.width                    = 640;
    config.height                   = 480;
    WGPUTextureFormat surfaceFormat = wgpuSurfaceGetPreferredFormat(surface, adapter);
    config.format                   = surfaceFormat;
    // And we do not need any particular view format:
    config.viewFormatCount = 0;
    config.viewFormats     = nullptr;
    config.usage           = WGPUTextureUsage_RenderAttachment;
    config.device          = device;
    config.presentMode     = WGPUPresentMode_Fifo;
    config.alphaMode       = WGPUCompositeAlphaMode_Auto;

    wgpuSurfaceConfigure(surface, &config);

    wgpuAdapterRelease(adapter);

    auto onDeviceError = [](WGPUErrorType type, char const* message, void* /* pUserData */) {
        std::cout << "Uncaptured device error: type " << type;
        if (message)
            std::cout << " (" << message << ")";
        std::cout << std::endl;
    };
    wgpuDeviceSetUncapturedErrorCallback(device, onDeviceError, nullptr /* pUserData */);

    std::cout << "Got device: " << device << std::endl;

    WGPUQueue queue = wgpuDeviceGetQueue(device);

    auto const GetNextSurfaceTextureView = [&]() -> WGPUTextureView {
        WGPUSurfaceTexture surfaceTexture;
        wgpuSurfaceGetCurrentTexture(surface, &surfaceTexture);
        if (surfaceTexture.status != WGPUSurfaceGetCurrentTextureStatus_Success)
        {
            return nullptr;
        }
        WGPUTextureViewDescriptor viewDescriptor;
        viewDescriptor.nextInChain     = nullptr;
        viewDescriptor.label           = "Surface texture view";
        viewDescriptor.format          = wgpuTextureGetFormat(surfaceTexture.texture);
        viewDescriptor.dimension       = WGPUTextureViewDimension_2D;
        viewDescriptor.baseMipLevel    = 0;
        viewDescriptor.mipLevelCount   = 1;
        viewDescriptor.baseArrayLayer  = 0;
        viewDescriptor.arrayLayerCount = 1;
        viewDescriptor.aspect          = WGPUTextureAspect_All;
        WGPUTextureView targetView     = wgpuTextureCreateView(surfaceTexture.texture, &viewDescriptor);
#ifndef WEBGPU_BACKEND_WGPU
        // We no longer need the texture, only its view
        // (NB: with wgpu-native, surface textures must not be manually released)
        wgpuTextureRelease(surfaceTexture.texture);
#endif // WEBGPU_BACKEND_WGPU
        return targetView;
    };

    while (!glfwWindowShouldClose(window))
    {
        // Check whether the user clicked on the close button (and any other
        // mouse/key event, which we don't use so far)
        glfwPollEvents();

        // Get the next target texture view
        WGPUTextureView targetView = GetNextSurfaceTextureView();
        if (!targetView)
            return 1;

        // Create a command encoder for the draw call
        WGPUCommandEncoderDescriptor encoderDesc = {};
        encoderDesc.nextInChain                  = nullptr;
        encoderDesc.label                        = "My command encoder";
        WGPUCommandEncoder encoder               = wgpuDeviceCreateCommandEncoder(device, &encoderDesc);

        WGPURenderPassDescriptor renderPassDesc = {};
        renderPassDesc.nextInChain              = nullptr;

        WGPURenderPassColorAttachment renderPassColorAttachment = {};
        renderPassColorAttachment.view                          = targetView;
        renderPassColorAttachment.resolveTarget                 = nullptr;
        renderPassColorAttachment.loadOp                        = WGPULoadOp_Clear;
        renderPassColorAttachment.storeOp                       = WGPUStoreOp_Store;
        renderPassColorAttachment.clearValue                    = WGPUColor{1., 0., 0., 1.0};
#ifndef WEBGPU_BACKEND_WGPU
        renderPassColorAttachment.depthSlice = WGPU_DEPTH_SLICE_UNDEFINED;
#endif // NOT WEBGPU_BACKEND_WGPU
        renderPassDesc.depthStencilAttachment = nullptr;
        renderPassDesc.timestampWrites        = nullptr;

        renderPassDesc.colorAttachmentCount = 1;
        renderPassDesc.colorAttachments     = &renderPassColorAttachment;

        // Create the render pass and end it immediately (we only clear the screen but do not draw anything)
        WGPURenderPassEncoder renderPass = wgpuCommandEncoderBeginRenderPass(encoder, &renderPassDesc);
        wgpuRenderPassEncoderEnd(renderPass);
        wgpuRenderPassEncoderRelease(renderPass);

        // Finally encode and submit the render pass
        WGPUCommandBufferDescriptor cmdBufferDescriptor = {};
        cmdBufferDescriptor.nextInChain                 = nullptr;
        cmdBufferDescriptor.label                       = "Command buffer";
        WGPUCommandBuffer command                       = wgpuCommandEncoderFinish(encoder, &cmdBufferDescriptor);
        wgpuCommandEncoderRelease(encoder);

        std::cout << "Submitting command..." << std::endl;
        wgpuQueueSubmit(queue, 1, &command);
        wgpuCommandBufferRelease(command);
        std::cout << "Command submitted." << std::endl;

        // At the end of the frame
        wgpuTextureViewRelease(targetView);
#ifndef __EMSCRIPTEN__
        wgpuSurfacePresent(surface);
#endif
#if defined(WEBGPU_BACKEND_DAWN)
        wgpuDeviceTick(device);
#elif defined(WEBGPU_BACKEND_WGPU)
        wgpuDevicePoll(device, false, nullptr);
#endif
    }

    for (int i = 0; i < 5; ++i)
    {
        std::cout << "Tick/Poll device..." << std::endl;
#if defined(WEBGPU_BACKEND_DAWN)
        wgpuDeviceTick(device);
#elif defined(WEBGPU_BACKEND_WGPU)
        wgpuDevicePoll(device, false, nullptr);
#endif
    }

    //////
    //
    // Cleanup
    //// At the end
    wgpuSurfaceUnconfigure(surface);
    wgpuSurfaceRelease(surface);
    glfwDestroyWindow(window);
    glfwTerminate();
    wgpuQueueRelease(queue);
    wgpuDeviceRelease(device);
}