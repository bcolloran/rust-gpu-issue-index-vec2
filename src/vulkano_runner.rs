use std::sync::Arc;

use spirv_std::glam::Vec2;
use std::collections::BTreeMap;
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator,
        layout::{
            DescriptorSetLayout, DescriptorSetLayoutBinding, DescriptorSetLayoutCreateInfo,
            DescriptorType,
        },
        DescriptorSet, WriteDescriptorSet,
    },
    device::{Device, DeviceCreateInfo, DeviceFeatures, Queue, QueueCreateInfo},
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{
        compute::{ComputePipeline, ComputePipelineCreateInfo},
        layout::{PipelineLayout, PipelineLayoutCreateInfo},
        Pipeline, PipelineBindPoint, PipelineShaderStageCreateInfo,
    },
    shader::{ShaderModule, ShaderStages},
    sync::{self, GpuFuture},
    VulkanLibrary,
};

use crate::{KERNEL_ENTRY_POINT, WORKGROUP_SIZE};

pub struct VulkanoRunner {
    device: Arc<Device>,
    queue: Arc<Queue>,
    pipeline: Arc<ComputePipeline>,
    memory_allocator: Arc<StandardMemoryAllocator>,
    descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
}

impl VulkanoRunner {
    /// Create a new Vulkano runner
    pub fn new() -> Self {
        // 1. Load the Vulkan library
        let library = VulkanLibrary::new().unwrap();

        // 2. Create instance
        let instance = Instance::new(library, InstanceCreateInfo::default()).unwrap();

        // 3. Pick first physical device with a compute queue
        let physical = instance
            .enumerate_physical_devices()
            .unwrap()
            .next()
            .unwrap();

        // 4. Select a queue family that supports compute
        let (queue_family_index, _q_props) = physical
            .queue_family_properties()
            .iter()
            .enumerate()
            .find(|(_, q)| q.queue_flags.contains(vulkano::device::QueueFlags::COMPUTE))
            .map(|(i, q)| (i as u32, q.clone()))
            .unwrap();

        // 5. Create logical device + queue
        let (device, mut queues) = Device::new(
            physical,
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                enabled_features: DeviceFeatures {
                    vulkan_memory_model: true,

                    #[cfg(feature = "variable_pointers")]
                    variable_pointers: true,
                    #[cfg(feature = "variable_pointers_storage_buffer")]
                    variable_pointers_storage_buffer: true,

                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .unwrap();

        let queue = queues.next().unwrap();

        // 6. Memory allocator
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
            device.clone(),
            Default::default(),
        ));
        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            Default::default(),
        ));

        // 7. Create shader module from embedded SPIR-V
        let kernel_bytes = crate::KERNEL_SPIRV;
        // Convert SPIR-V bytes to words then create shader module
        let words = vulkano::shader::spirv::bytes_to_words(kernel_bytes).unwrap();
        let shader_module = unsafe {
            ShaderModule::new(
                device.clone(),
                vulkano::shader::ShaderModuleCreateInfo::new(&words),
            )
        }
        .unwrap();

        let entry_point = shader_module.entry_point(KERNEL_ENTRY_POINT).unwrap();

        // Descriptor set layout (binding 0: storage buffer)
        let mut binding0 =
            DescriptorSetLayoutBinding::descriptor_type(DescriptorType::StorageBuffer);
        binding0.stages = ShaderStages::COMPUTE;
        binding0.descriptor_count = 1;
        let mut bindings = BTreeMap::new();
        bindings.insert(0u32, binding0);
        let descriptor_set_layout = DescriptorSetLayout::new(
            device.clone(),
            DescriptorSetLayoutCreateInfo {
                bindings,
                ..Default::default()
            },
        )
        .unwrap();

        // Pipeline layout + push constants
        let pipeline_layout = PipelineLayout::new(
            device.clone(),
            PipelineLayoutCreateInfo {
                set_layouts: vec![descriptor_set_layout],

                ..Default::default()
            },
        )
        .unwrap();

        // Build stage and compute pipeline
        let stage = PipelineShaderStageCreateInfo::new(entry_point.clone());
        let pipeline_info = ComputePipelineCreateInfo::stage_layout(stage, pipeline_layout.clone());
        let pipeline = ComputePipeline::new(device.clone(), None, pipeline_info).unwrap();

        Self {
            device,
            queue,
            pipeline,
            memory_allocator,
            descriptor_set_allocator,
            command_buffer_allocator,
        }
    }

    pub fn run_pass(&self, data: &mut [Vec2]) {
        // Allocate a CPU visible buffer, copy input, run compute, read back
        let len = data.len();

        // Create buffer (HOST visible & coherent)
        let usage =
            BufferUsage::STORAGE_BUFFER | BufferUsage::TRANSFER_SRC | BufferUsage::TRANSFER_DST;
        let buffer: Subbuffer<[Vec2]> = Buffer::from_iter(
            self.memory_allocator.clone(),
            BufferCreateInfo {
                usage,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_RANDOM_ACCESS,
                ..Default::default()
            },
            data.iter().copied(),
        )
        .unwrap();

        // Create descriptor set (binding 0: storage buffer)
        let layout = self
            .pipeline
            .layout()
            .set_layouts()
            .get(0)
            .cloned()
            .unwrap();

        let set = DescriptorSet::new(
            self.descriptor_set_allocator.clone(),
            layout,
            [WriteDescriptorSet::buffer(0, buffer.clone())],
            [],
        )
        .unwrap();

        // Build command buffer
        let mut builder = AutoCommandBufferBuilder::primary(
            self.command_buffer_allocator.clone(),
            self.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .bind_pipeline_compute(self.pipeline.clone())
            .unwrap();
        builder
            .bind_descriptor_sets(
                PipelineBindPoint::Compute,
                self.pipeline.layout().clone(),
                0,
                set,
            )
            .unwrap();

        // Dispatch
        let num_workgroups = len.div_ceil(WORKGROUP_SIZE) as u32;
        unsafe {
            builder.dispatch([num_workgroups, 1, 1]).unwrap();
        }

        let command_buffer = builder.build().unwrap();

        // Execute + wait
        let future = sync::now(self.device.clone())
            .then_execute(self.queue.clone(), command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();
        future.wait(None).unwrap();

        // Read back results (buffer is host visible)
        let content = buffer.read().unwrap();
        data.copy_from_slice(&content[..len]);
    }
}
