# line-tracer

2-dimensional light transport simulation between geometry consisting of lines, arcs, and circles. Fully simulates diffuse and reflective surfaces, as well as refractive volumes. 

This differs from the traditional per-pixel approach that 3D light simulations take, primarily because those real-time scenes are computing per-pixel values projected onto the surface of the relevant geometry. The idea of ray casting is a convenient abstraction,
but not actually represented in any information that gets sent to the rasterizer.

Using geometry shaders, a new rendering pipeline stage widely adopted starting in 2010, I am able to make the GPU calculate each ray as an execution unit, instead of each pixel - meaning that if you want to visualize light transport through a smoky medium, the rasterizer is working on a much larger unit and can get orders of magnitude more rendering done.

With a little bit of post processing, this technique lends itself incredibly well to real-time simulation of 2D scenes with physically accurate shading. There are many techniques for visualizing light through a scene in 2D, but to my knowledge this is the first time it's been done in this manner or as efficiently. 

This technique is limited to 2D scenes, unfortunately, as visualizing a 3D volume of full-color data is not possible in a useful manner for a static rendering. Thankfully, you never actually need a full 3D volume - most modern techniques simply render the portions visible from the camera, starting from a pixel and tracing through the scene. A full trace of the volume as shown in this project is overkill for 3D scenes.

The UI is immediate-mode UI from the OpenGL-based framework `egui` (https://github.com/emilk/egui), lending itself to fast prototyping and ease of use. I use it on top of my own OpenGL code, and I don't use any frameworks for the actual graphics handling - only for UI and text rendering.

<img width="817" alt="image" src="https://github.com/jearmstrong21/line-tracer/assets/20100635/e8b24aff-a9ef-4b90-8035-710f5f793dbd">
<img width="817" alt="image" src="https://github.com/jearmstrong21/line-tracer/assets/20100635/92080713-ed14-46bb-92e5-60d888020db8">
<img width="817" alt="image" src="https://github.com/jearmstrong21/line-tracer/assets/20100635/d8ae2673-9f49-4643-9af9-c827d77b1712">
<img width="265" alt="image" src="https://github.com/jearmstrong21/line-tracer/assets/20100635/a32dff52-6c78-4058-822d-d7eaebb344b6">
