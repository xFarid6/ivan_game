use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Component)]
pub struct Collider;

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    fn contains(&self, point: Vec2) -> bool {
        point.x >= self.x && point.x <= self.x + self.width &&
        point.y >= self.y && point.y <= self.y + self.height
    }

    fn intersects(&self, other: &Rectangle) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }

    pub fn min_x(&self) -> f32 {
        self.x
    }

    pub fn max_x(&self) -> f32 {
        self.x + self.width
    }

    pub fn min_y(&self) -> f32 {
        self.y
    }

    pub fn max_y(&self) -> f32 {
        self.y + self.height
    }
}

#[derive(Debug, Resource)]
pub struct Quadtree {
    boundary: Rectangle,
    capacity: usize,
    objects: HashSet<Entity>,  // The entities within the quadtree region
    subdivided: bool,
    north_west: Option<Box<Quadtree>>,
    north_east: Option<Box<Quadtree>>,
    south_west: Option<Box<Quadtree>>,
    south_east: Option<Box<Quadtree>>,
}

impl Quadtree {
    pub fn new(boundary: Rectangle, capacity: usize) -> Self {
        Quadtree {
            boundary,
            capacity,
            objects: HashSet::new(),
            subdivided: false,
            north_west: None,
            north_east: None,
            south_west: None,
            south_east: None,
        }
    }

    fn subdivide(&mut self) {
        let half_width = self.boundary.width / 2.0;
        let half_height = self.boundary.height / 2.0;
        let x = self.boundary.x;
        let y = self.boundary.y;

        let nw = Rectangle { x, y, width: half_width, height: half_height };
        let ne = Rectangle { x: x + half_width, y, width: half_width, height: half_height };
        let sw = Rectangle { x, y: y + half_height, width: half_width, height: half_height };
        let se = Rectangle { x: x + half_width, y: y + half_height, width: half_width, height: half_height };

        self.north_west = Some(Box::new(Quadtree::new(nw, self.capacity)));
        self.north_east = Some(Box::new(Quadtree::new(ne, self.capacity)));
        self.south_west = Some(Box::new(Quadtree::new(sw, self.capacity)));
        self.south_east = Some(Box::new(Quadtree::new(se, self.capacity)));

        self.subdivided = true;
    }

    pub fn insert(&mut self, entity: Entity, position: Vec2) {
        if !self.boundary.contains(position) {
            return;  // This object is out of bounds
        }

        if self.objects.len() < self.capacity {
            self.objects.insert(entity);
        } else {
            if !self.subdivided {
                self.subdivide();
            }

            // Recursively insert into the subtrees
            if let Some(ref mut nw) = self.north_west {
                nw.insert(entity, position);
            }
            if let Some(ref mut ne) = self.north_east {
                ne.insert(entity, position);
            }
            if let Some(ref mut sw) = self.south_west {
                sw.insert(entity, position);
            }
            if let Some(ref mut se) = self.south_east {
                se.insert(entity, position);
            }
        }
    }

    pub fn query(&self, range: &Rectangle, found: &mut HashSet<Entity>) {
        if !self.boundary.intersects(range) {
            return;
        }

        for &entity in &self.objects {
            found.insert(entity);
        }

        if self.subdivided {
            if let Some(ref nw) = self.north_west {
                nw.query(range, found);
            }
            if let Some(ref ne) = self.north_east {
                ne.query(range, found);
            }
            if let Some(ref sw) = self.south_west {
                sw.query(range, found);
            }
            if let Some(ref se) = self.south_east {
                se.query(range, found);
            }
        }
    }

    pub fn query_bounding_boxes(
        &self, 
        range: &Rectangle, 
        found: &mut HashSet<Entity>
    ) {
        if !self.boundary.intersects(range) {
            return;
        }
    
        // For each entity in this Quadtree region, check if it intersects with the query range
        for &entity in &self.objects {
            // Example check, assuming entities are rectangular and have a bounding box
            // If your entity shapes are more complex (e.g., circles or polygons), adjust accordingly.
            let entity_bb = get_entity_bounding_box(entity); // Get the entity's bounding box
            if entity_bb.intersects(range) {
                found.insert(entity);
            }
        }
    
        // Recurse into subtrees if subdivided
        if self.subdivided {
            if let Some(ref nw) = self.north_west {
                nw.query_bounding_boxes(range, found);
            }
            if let Some(ref ne) = self.north_east {
                ne.query_bounding_boxes(range, found);
            }
            if let Some(ref sw) = self.south_west {
                sw.query_bounding_boxes(range, found);
            }
            if let Some(ref se) = self.south_east {
                se.query_bounding_boxes(range, found);
            }
        }
    }

    pub fn update_entity_position(
        &mut self, 
        entity: Entity, 
        old_position: Vec2, 
        new_position: Vec2
    ) {
        // Remove from old position if necessary
        if !self.boundary.contains(old_position) {
            self.remove(entity, old_position);
        }
        
        // Insert into new position
        self.insert(entity, new_position);
    }
    
    pub fn remove(&mut self, entity: Entity, position: Vec2) {
        if !self.boundary.contains(position) {
            return;  // This entity is out of bounds of this node
        }

        // Try to remove the entity if it's in this node
        if self.objects.remove(&entity) {
            return;
        }

        // If the quadtree is subdivided, recursively attempt to remove the entity from sub-nodes
        if self.subdivided {
            if let Some(ref mut nw) = self.north_west {
                nw.remove(entity, position);
            }
            if let Some(ref mut ne) = self.north_east {
                ne.remove(entity, position);
            }
            if let Some(ref mut sw) = self.south_west {
                sw.remove(entity, position);
            }
            if let Some(ref mut se) = self.south_east {
                se.remove(entity, position);
            }
        }
    }

    // Function to recursively draw the quadtree nodes as rectangles using gizmos
    pub fn draw_gizmos(&self, gizmos: &mut Gizmos, color: Color) {
        // Draw the current node boundary
        self.draw_rectangle(gizmos, self.boundary.clone(), color);

        // If subdivided, recursively draw the child nodes
        if self.subdivided {
            if let Some(ref nw) = self.north_west {
                nw.draw_gizmos(gizmos, color);
            }
            if let Some(ref ne) = self.north_east {
                ne.draw_gizmos(gizmos, color);
            }
            if let Some(ref sw) = self.south_west {
                sw.draw_gizmos(gizmos, color);
            }
            if let Some(ref se) = self.south_east {
                se.draw_gizmos(gizmos, color);
            }
        }
    }

    // Helper function to draw a rectangle
    fn draw_rectangle(&self, gizmos: &mut Gizmos, rect: Rectangle, color: Color) {
        let min_x = rect.min_x();
        let max_x = rect.max_x();
        let min_y = rect.min_y();
        let max_y = rect.max_y();

        // Draw the four edges of the rectangle
        gizmos.line(Vec3::new(min_x, min_y, 0.0), Vec3::new(max_x, min_y, 0.0), color);
        gizmos.line(Vec3::new(max_x, min_y, 0.0), Vec3::new(max_x, max_y, 0.0), color);
        gizmos.line(Vec3::new(max_x, max_y, 0.0), Vec3::new(min_x, max_y, 0.0), color);
        gizmos.line(Vec3::new(min_x, max_y, 0.0), Vec3::new(min_x, min_y, 0.0), color);
    }
}

pub fn check_for_collisions(
    quadtree: &Quadtree,
    query_range: Rectangle, // Define the region to query
) -> HashSet<Entity> {
    let mut found = HashSet::new();
    quadtree.query(&query_range, &mut found);
    found
}

fn get_entity_bounding_box(entity: Entity) -> Rectangle {
    // Placeholder logic
    // You'll need to replace this with however you're managing entity components or bounding boxes.
    // This might involve querying Bevy components to get the size/position of the entity.
    
    // Example: if your entities store position and size:
    let position = get_entity_position(entity);  // Fetch the position from the entity
    let size = get_entity_size(entity);  // Fetch the size from the entity

    Rectangle {
        x: position.x,
        y: position.y,
        width: size.x,
        height: size.y,
    }
}

fn get_entity_position(entity: Entity) -> Vec2 {
    // Placeholder: replace this with actual logic to retrieve the entity's position
    Vec2::new(0.0, 0.0)
}

fn get_entity_size(entity: Entity) -> Vec2 {
    // Placeholder: replace this with actual logic to retrieve the entity's size
    Vec2::new(10.0, 10.0)
}

pub fn draw_quadtree_system(
    quadtree: &Quadtree, // Assuming you stored the quadtree in a resource
    mut gizmos: Gizmos,
) {
    quadtree.draw_gizmos(&mut gizmos, Color::WHITE);
}
