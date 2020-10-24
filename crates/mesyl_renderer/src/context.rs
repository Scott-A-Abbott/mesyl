use ash::vk;

pub struct VkContext {
    _entry: ash::Entry,
    device: ash::Device,
    instance: ash::Instance,
    physical_info: VkPhysicalDevice,
    surface_info: VkSurface,
    queues: VkQueues,
}

struct VkQueues {
    graphics: vk::Queue,
    present: vk::Queue,
}

struct VkSurface {
    inner: ash::extensions::khr::Surface,
    loader: vk::SurfaceKHR,
}

struct VkPhysicalDevice {
    inner: vk::PhysicalDevice,
    queue_family_indices: QueueFamilyIndices,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
}

struct QueueFamilyIndices {
    graphics_family: Option<u32>,
    present_family: Option<u32>,
}