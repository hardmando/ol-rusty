# ol-rusty
**ol-rusty** is a game engine developed in Rust with Vulkan support.

## Structure
**ol-rusty** consists of few modules, for now they are:
  - **ol-engine**: Main app functionality, connection of the rest of the modules
  - **ol-renderer**: Vulkan renderer made with ash
  - **ol-core**: Core game engine functionality
  - **ol-windowing**: Window creation and handling
  - *(ol-scene)*

## Why ol-rusty?
**ol-rusty** is developed with the idea of providing 3D game engine for Rust developers as well as creating one for my own needs.
ol-rusty will implement ECS paradigm and is being created with the machine-friendly design on the first place.
Big part of ol-rusty's future is data oriented design and easy way to implement ECS. With that in mind I hope to
create an game engine that will both be easy to use and provide easy and handy way to develope games in Rust.

## Plans
In the future I plan to add Spir-V directly in the engine, so there will be no need to recompile glsl shaders every time.
I also will try my best to add as much out-of-box shaders and technology support (i.e. ao, sss, particle systems, etc.) as I will be able to.
