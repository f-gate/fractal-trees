
use flo_draw::*;
use flo_draw::canvas::*;

const COLOR: Color = Color::Rgba(1.0, 0.0, 0.8, 1.0);
const ANGLE_START: f32 = 10.0;
const DEPTH: i32 = 15;
const LINE_WIDTH: f32 = 1.0;


fn main() {

    with_2d_graphics(|| {
        let canvas = create_canvas_window("Fractal Tree Generator");
        canvas.draw(|gc| {

            let mut t = Tree::default();
            gc.line_width(LINE_WIDTH);
            gc.stroke_color(COLOR);
            t.branch(gc, 500.0, 0.0, 0.0, DEPTH);
 

            //make canvas height based off tree generated.
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);

            gc.canvas_height(t.get_canvas_height() as f32);
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

    // get u32 canvas height
    pub fn get_canvas_height(&self) -> u32 {
        let mut y_values: Vec<u32> = vec![];
        self.branches.iter().map(|b|y_values.push(b.y.abs() as u32));
        println!("{:?}", y_values);
        match y_values.iter().max() {
            Some(v) => {
                println!("{}", v);
                *v
            } ,
            _ => {
                println!("{}", 0); 
                0
            }  
        }
    }

    /// Populates the branches of a tree.
    pub fn branch( &mut self, gc: &mut CanvasGraphicsContext, x : f32 ,y : f32, angle: f32, n: i32) {
        self.branches.push(Branch::new(COLOR, x, y));

        let x2 = x + angle.to_radians().sin() * n as f32 * 10.0;
        let y2 = y + angle.to_radians().cos() * n as f32 * 10.0;
        gc.new_path();
        gc.move_to(x, y);
        gc.line_to(x2 , y2);
        gc.close_path();
        gc.stroke();    


        if n == 0 {
          return;
         }
        self.branch(gc, x2, y2, angle - ANGLE_START, n-1);
        self.branch(gc, x2, y2, angle + ANGLE_START, n-1);
}

}
