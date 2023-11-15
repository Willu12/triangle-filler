use egui_sdl2_gl::gl::types::GLfloat;
use crate::triangle::Triangle;
use crate::triangle::*;

pub struct Grid {
    pub x_accuracy : u32,
    pub y_accuracy : u32,
    pub triangles : Vec<Triangle>
}

impl Grid {
    pub fn new(x: u32, y: u32) -> Self {
        Grid {x_accuracy : x, y_accuracy: y, triangles: vec![]}
    }

    pub fn draw(&mut self) {
        for triangle in self.triangles.iter() {
            triangle.draw();
        }
    }

    pub fn update_accuracy(&mut self, x: u32, y:u32) -> bool {
        let mut accuracy_changed = false;
        if self.x_accuracy != x || self.y_accuracy != y {accuracy_changed = true};
        self.x_accuracy = x;
        self.y_accuracy = y;

        accuracy_changed
    }

    pub fn update_triangles(&mut self) {
        let triangle_points = self.calculate_triangles_points();

        for triangle in triangle_points {
            let new_triangle = Triangle::new(triangle);
            self.triangles.push(new_triangle);
        }
    }

    fn calculate_triangles_points(&self) -> Vec<[MyVertex;3]> {
        let mut triangles: Vec<[MyVertex;3]> = vec![];
        let x_offset = 2.0 / self.x_accuracy as f32;
        let y_offset = 2.0  / self.y_accuracy as f32;

        // we create 2 triangles for each square of our grid
        for x in 0..self.x_accuracy {
            for y in 0..self.y_accuracy {
                let start: [GLfloat;3] = [-1.0 + x as f32 * x_offset, -1.0 + (y + 1) as f32 * y_offset,0.0];
                let end: [GLfloat;3] = [-1.0 + (x + 1) as f32 * x_offset,-1.0 + y as f32 * y_offset,0.0];

                let mid_upper: [GLfloat;3] = [-1.0 + x as f32 * x_offset, -1.0 + y as f32 * y_offset,0.0];
                let mid_lower: [GLfloat;3] = [-1.0 + (x + 1) as f32 * x_offset,-1.0 + (y + 1) as f32 * y_offset,0.0];

                //create upper triangle
                triangles.push([start,mid_upper,end]);
                triangles.push([start,mid_lower,end]);
            }
        }
       return triangles;
    }



}

//teraz chialbym podzielic ekran
