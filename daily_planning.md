Daily planner for the continuation of the project:

- [x] Make a daily planner
- [x] Add boundaries for the ball (screen)
- [x] Reposition ball on mouse-click position
- [x] Add new ball with keyboard key A at random position
        Balls added this way are deleted on contact with window border
- [x] Add gravity for making all balls fall
- [x] Add static shapes in the space to test collision with the ball
- [x] Move constants to a different file
- [x] Change gravity with arrow keys
- [x] Have clarity on [when to use Entities](https://taintedcoders.com/bevy/entities/)
- [x] Add bevy inspector
- [x] Understand Events in Bevy [(EventWriter and EventReader)](https://bevy-cheatbook.github.io/programming/events.html)
- [x] Switch to absolute position on screen (instead of window) of the ball
- [x] Function to convert [from window to world coordinates](https://stackoverflow.com/questions/64714076/how-do-i-convert-screen-space-to-world-space-coords-in-bevy-using-camera2dcompon)
- [-] ~~Control ball with keyboard arrow keys~~ (WASD + key to turn off gravity)
- [ ] Test collisions with Triangles and Squares [(add collisions)](https://kishimotostudios.com/articles/aabb_collision/)
- [x] Check out Tiled level editor (found [a better alternative](https://www.spritefusion.com/editor))
- [x] Get started working on the player (his file, structs, functions, sprites, ...)
- [x] On [using bundles](https://bevy-cheatbook.github.io/programming/bundle.html)
- [ ] Test out some particle systems
- [x] Make a navigation for the app (currently implemented as State-switching)
- [ ] Get started on some UI elements (possibly reusables) 
- [x] Learn how to [add bloom to shapes](https://bevyengine.org/examples/2d-rendering/bloom-2d/)
- [ ] Make various types of particles 
- [x] Make a world for a game (I'd go pixel art, 2D, lateral movement) as a tilemap
- [x] Import it with the bevy_ecs_tilemap plugin
- [x] Add a "Scene" stack
- [ ] Add parallax scrolling
- [x] Setup camera for pixel perfect rendering
- [x] Serve the game on the web?! (lib.rs file)
- [ ] Make another world for the game
- [ ] Add satisfying jigsaw particle effect (particles scatter on screen and follow mouse + explosion on click)
        - expel force between particles
        - random wandering vector
        - attraction force to mouse position
        - repulsion explosion on mouse click
- [x] Adding buttons
- [ ] Adding shaders
- [x] Tilemap loading and rendering system
- [ ] Implement new Scene with Graph drawing simulation
- [x] Add animated character
- [x] Implement cleanup functions for each level
- [x] Add personal ID storage for tilemaps
- [x] .set(ImagePlugin::default_nearest()) DONE
- [x] Add circle-filling timer
- [ ] Develop button adding api
- [ ] Fix github Actions
- [x] Add client
- [x] Add server
- [ ] Integrate client with the game
- [ ] Integrate server with the game
- [ ] Add a Scene to play Snake
- [ ] Add a Scene to play Space Shooter
- [ ] Add a connect four
 
----


Useful links:

- [Color picker fore srgb](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_colors/Color_picker_tool) (bloom effect enabled)
- [Rust macro for enum stringyfication](https://stackoverflow.com/questions/32710187/how-do-i-get-an-enum-as-a-string)
- [Undestanding the Bevy UI](https://taintedcoders.com/bevy/ui)
- [Understanding time and timers](https://bevy-cheatbook.github.io/fundamentals/time.html)
- Here's where we're actually going to [MAKE THE MAPS](https://www.spritefusion.com/editor)

**Meshes and Materials: What are they?**
In the context of 3D graphics and game development, particularly in engines like Bevy, meshes and materials serve different but complementary purposes. Here’s a breakdown of the differences between them:

Meshes
Definition: A mesh is a collection of vertices, edges, and faces that define the shape of a 3D object in space. It represents the geometric structure of an object.

Components:

Vertices: Points in 3D space that define the corners of the mesh.
Faces: Flat surfaces that are formed by connecting vertices. These can be triangles or quadrilaterals.
Normals: Vectors perpendicular to the surface of the mesh that are used for lighting calculations.
UV Coordinates: Coordinates that map points on the mesh to points on a texture, allowing for detailed surface texturing.
Purpose: Meshes define the shape and structure of 3D models. For example, a character model, an environment prop, or any object in a game is represented by a mesh.

Materials
Definition: A material defines the surface properties of a mesh, such as its color, texture, reflectivity, transparency, and how it interacts with light.

Components:

Texture: An image that is applied to the surface of a mesh to give it detail and color.
Color: Base color information that can be applied to the mesh.
Shader: Programs that dictate how the material should respond to lighting, shadows, and other visual effects. Shaders can implement various rendering techniques (like diffuse, specular, or normal mapping).
Purpose: Materials determine the appearance of a mesh. They define how light interacts with the surface, the surface's texture, and its overall color, making the mesh look realistic or stylized in a game environment.

Relationship Between Meshes and Materials
Combination: In a typical rendering pipeline, you will have a mesh (the geometry) that is paired with a material (the appearance). When you render an object, the graphics engine will use both the mesh and its associated material to produce the final image on the screen.

Reusability: You can use the same material with different meshes to maintain a consistent look across different objects. Conversely, you can apply different materials to the same mesh to achieve various visual effects.

Rendering: During rendering, the graphics engine will first draw the mesh to the screen and then apply the material properties to determine how that mesh looks based on lighting and other scene factors.
