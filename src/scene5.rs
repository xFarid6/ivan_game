use crate::collisions::Rectangle;
use crate::{draw_quadtree_system, get_mouse_position, getwindowsize, Collider, Quadtree};
use crate::check_for_collisions;
use bevy::gizmos::circles;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::{gizmos::gizmos, prelude::*};
use bevy::color::palettes::{basic::*, css::*, tailwind::*};

#[derive(Debug, Clone, Component)]
pub struct VerletObject {
    position_current: Vec2,
    position_old: Vec2,
    acceleration: Vec2,
}

impl VerletObject {
    pub fn update_position(&mut self, dt: f32) {
        let velocity = self.position_current - self.position_old;
        // Save current position
        self.position_old = self.position_current;
        // Perform Verlet Integration
        self.position_current = self.position_current + velocity + self.acceleration * dt * dt;
        // Reset acceleration
        self.acceleration = Vec2::new(0., 0.);
    }

    pub fn accelerate(&mut self, acc: Vec2) {
        self.acceleration += acc;
    }
}

#[derive(Debug, Component)]
pub struct Solver {
    gravity: Vec2,
    objects: Vec<VerletObject>,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            gravity: Vec2 { x: 0.0, y: -1000. },
            objects: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let sub_steps = 8;
        let sub_dt = dt / sub_steps as f32;

        for i in 0..sub_steps {
            self.apply_gravity();
            self.apply_constraint();
            self.solve_collisions();
            self.update_positions(sub_dt);
        }
    }

    pub fn update_positions(&mut self, dt: f32) {
        for mut obj in self.objects.iter_mut() {
            obj.update_position(dt);
        }
    }

    pub fn apply_gravity(&mut self) {
        for mut obj in self.objects.iter_mut() {
            obj.accelerate(self.gravity);
        }
    }

    pub fn apply_constraint(&mut self) {
        let position = Vec2::new(0., 0.);
        let radius = 350.;
        for obj in self.objects.iter_mut() {
            let to_obj = obj.position_current - position;
            let dist = to_obj.length();
            // 15. is the default radius
            if dist > radius - 15. {
                let n = to_obj / dist;
                obj.position_current = position + n * (radius - 15.);
            }
        }
    }

    pub fn solve_collisions(&mut self) {
        let obj_count = self.objects.len();
        for i in 0..obj_count {
            // Split the slice at index `i + 1`, so `obj1` is the i-th element and `rest` contains the rest
            let (left, right) = self.objects.split_at_mut(i + 1);
            let obj1 = &mut left[i];  // Borrow obj1 mutably
            
            // Loop through the remaining objects
            for obj2 in right {
                let collision_axis = obj1.position_current - obj2.position_current;
                let dist = collision_axis.length();
                if dist < 30. {
                    let n = collision_axis / dist;
                    let delta = 30. - dist;
                    obj1.position_current += 0.5 * delta * n;
                    obj2.position_current -= 0.5 * delta * n;
                }
            }
        }
    }
}

// #[derive(Debug, Clone, Component)]
// pub struct Link {
//     obj1: &VerletObject,
//     obj2: &VerletObject,
//     target_dist: f32
// }

// impl Link {
//     fn new(obj1: &VerletObject, obj2: &VerletObject, td: f32) -> Self {
//         Self {
//             obj1,
//             obj2,
//             target_dist: td
//         }
//     }

//     fn apply(&self) {
//         let mut obj1 = *self.obj1;
//         let mut obj2 = *self.obj2;
//         let axis = obj1.position_current - obj2.position_current;
//         let dist = axis.length();
//         let n = axis / dist;
//         let delta = self.target_dist - dist;
//         obj1.position_current += 0.5 * delta * n;
//         obj2.position_current -= 0.5 * delta * n;
//     }
// }

pub fn setup_solver(mut commands: Commands) {
    
    let mut solver = Solver::new();
    commands.spawn(solver);
    
}

pub fn setup_scene5(
    mut commands: Commands,
    mut gizmos: Gizmos
) {
    let center = Vec2::new(0., 0.);
    let radius = 350.;
    gizmos.circle_2d(center, radius, Color::srgb(7.5, 0.0, 7.5));
}

pub fn spawn_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut solver: Query<&mut Solver>,
    time: Res<Time>,
    mut gizmos: Gizmos
) {
    let mouse_pos = get_mouse_position(camera_query, window).expect("Mouse not found");

    let dt = time.delta_seconds();
    let mut solver = solver.single_mut();

    if mouse_button_input.just_pressed(MouseButton::Left) {
        let v_obj = VerletObject {
            position_current: Vec2 { x: mouse_pos.x, y: mouse_pos.y },
            position_old: Vec2 { x: mouse_pos.x, y: mouse_pos.y },
            acceleration: Vec2::ZERO,
        };

        solver.objects.push(v_obj);
    }
    
    solver.update(dt);
    update_verletobj_texture(solver.objects.clone(), gizmos);
}

pub fn update_verletobj_texture(
    objects: Vec<VerletObject>,
    mut gizmos: Gizmos
) { 
    for obj in objects.iter() {
        gizmos.circle_2d(obj.position_current, 15., Color::srgb(6.25, 9.4, 9.1));
    }
}

pub fn rebuild_quadtree(
    window: Query<&Window>,
    objects: Query<(Entity, &Transform), With<Collider>>,
    mut gizmos: Gizmos,
) {
    // Building the tree
    let (w, h) = getwindowsize(window);
    let boundary = Rectangle { 
        x: -w/2., 
        y: -h/2., 
        width: w, 
        height: h 
    };
    let capacity = 4;
    let mut tree = Quadtree::new(boundary.clone(), capacity);

    // Inserting the objects
    for (entity, transform) in objects.iter() {
        let position = transform.translation.truncate();
        tree.insert(entity, position);
    }

    // Draw debug lines
    draw_quadtree_system(&tree, gizmos);

    // // Get an HashSet of the possible ongoing collisions
    // let found = check_for_collisions(&tree, boundary.clone());
    // // println!("Found is: {:?}", found);

}

fn circle_circle_collision(pos_a: Vec2, radius_a: f32, pos_b: Vec2, radius_b: f32) -> bool {
    let distance_squared = (pos_b - pos_a).length_squared();
    let radius_sum = radius_a + radius_b;
    distance_squared < radius_sum * radius_sum
}
