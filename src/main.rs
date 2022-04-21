use rand::prelude::*;
use flo_draw::*;
use flo_draw::canvas::*;

const COLOR: Color = Color::Rgba(1.0, 0.0, 0.8, 1.0);
const ANGLE_START: f32 = 15.0;
const DEPTH: i32 = 13;
const LINE_WIDTH: f32 = 2.0;


fn main() {

    with_2d_graphics(|| {
        let canvas = create_canvas_window("Fractal Tree Generator");
        canvas.draw(|gc| {
            let SCR_HEIGHT = 100.0*DEPTH as f32;

            let mut t = Tree::default();
            gc.line_width(LINE_WIDTH);
            gc.stroke_color(COLOR);
            gc.canvas_height(SCR_HEIGHT);
            gc.center_region(0.0, 0., SCR_HEIGHT, SCR_HEIGHT);

            t.draw_tree(gc, SCR_HEIGHT/2.0, SCR_HEIGHT/10.0, 0.0, DEPTH);

            ()
        });
    

    })
}



#[derive(Debug)]
pub struct Branch {
    color: Color,
    x: f32,
    y: f32,
}
impl Branch {
    pub fn new(color: Color, x: f32, y:f32) -> Self {
         Self {
            color,
            x,
            y,
         }
    }
}

#[derive(Default, Debug)]
pub struct Tree {
    branches: Vec<Branch>,
    n: i32,
}

impl Tree {

    // get tree height
    pub fn get_tree_height(&self) -> f32 {
        let mut max_y = 0.0;
        for b in &self.branches {
            if b.y.abs() < max_y {
                continue;
            } else {
                max_y = b.y.abs();
            }
        }
        max_y as f32
    }

    /// Populates the branches of a tree. and draws tree
    pub fn draw_tree( &mut self, gc: &mut CanvasGraphicsContext, x : f32 ,y : f32, angle: f32, n: i32) {
        let mut rng = thread_rng();
        let color = Color::Rgba(rng.gen_range(0.5..=1.0), 0.0, rng.gen_range(0.5..=1.0), 1.0);
        self.branches.push(Branch::new(
            color, 
            x,
            y,
        ));

        let x2 = x + angle.to_radians().sin() * n as f32 * 10.0;
        let y2 = y + angle.to_radians().cos() * n as f32 * 10.0;
        gc.stroke_color(color);

        gc.new_path();
        gc.move_to(x, y);
        gc.line_to(x2 , y2);
        gc.close_path();    
        gc.stroke();

        if n == 0 {
          return;
         }
        self.draw_tree(gc, x2, y2, angle - ANGLE_START, n-1);
        self.draw_tree(gc, x2, y2, angle + ANGLE_START, n-1);


}

}
