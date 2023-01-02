use ash::{extensions::khr::Swapchain, vk, Device};
use egui::{epaint::ClippedShape, CtxRef};
use egui_winit::winit::window::Window;

use crate::{*, renderer::Renderer};

/// egui integration with winit and ash.
pub struct Integration<A: AllocatorTrait> {
    context: CtxRef,
    egui_winit: egui_winit::State,

    renderer: Renderer<A>
}
impl<A: AllocatorTrait> Integration<A> {
    /// Create an instance of the integration.
    pub fn new(
        window: &Window,
        physical_width: u32,
        physical_height: u32,
        scale_factor: f64,
        font_definitions: egui::FontDefinitions,
        style: egui::Style,
        device: Device,
        allocator: A,
        swapchain_loader: Swapchain,
        swapchain: vk::SwapchainKHR,
        surface_format: vk::SurfaceFormatKHR,
    ) -> Self {

        // Create context
        let context = CtxRef::default();
        context.set_fonts(font_definitions);
        context.set_style(style);

        let egui_winit = egui_winit::State::new(window);

        let renderer = Renderer::new(
            physical_width,
            physical_height,
            scale_factor,
            device,
            allocator,
            swapchain_loader,
            swapchain,
            surface_format
        );
        Self {
            context,
            egui_winit,
            renderer
        }
    }

 

    /// handling winit event.
    pub fn update(&mut self, winit_event: &egui_winit::winit::event::WindowEvent<'_>) -> bool {
        self.egui_winit.on_event(&self.context, winit_event)
    }

    /// begin frame.
    pub fn begin_frame(&mut self, window: &Window) {
        let raw_input = self.egui_winit.take_egui_input(window);
        self.context.begin_frame(raw_input);
    }

    /// end frame.
    pub fn end_frame(&mut self, window: &Window) -> (egui::Output, Vec<ClippedShape>) {
        let (output, clipped_shapes) = self.context.end_frame();

        self.egui_winit.handle_output(window, &self.context, output.clone());

        (output, clipped_shapes)
    }

    /// Get [`egui::CtxRef`].
    pub fn context(&self) -> CtxRef {
        self.context.clone()
    }

    /// Record paint commands.
    pub fn paint(
        &mut self,
        command_buffer: vk::CommandBuffer,
        swapchain_image_index: usize,
        clipped_meshes: Vec<egui::ClippedMesh>,
    ) {
        self.renderer.render(command_buffer, swapchain_image_index, clipped_meshes, &self.context.fonts().font_image());
    }

    pub fn resize(
        &mut self,
        physical_width: u32,
        physical_height: u32,
        swapchain: vk::SwapchainKHR,
        surface_format: vk::SurfaceFormatKHR) {
        self.renderer.update_swapchain(physical_width, physical_height, swapchain, surface_format);
    }

    pub fn register_user_texture(
        &mut self,
        image_view: vk::ImageView,
        sampler: vk::Sampler,
    ) -> egui::TextureId {
        self.renderer.register_user_texture(image_view, sampler)
    }

    pub fn unregister_user_texture(&mut self, texture_id: egui::TextureId) {
        self.renderer.unregister_user_texture(texture_id);
    }
    
    pub unsafe fn destroy(&mut self) {
        self.renderer.destroy();
    }
}
