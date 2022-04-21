use flo_draw::*;
use flo_draw::canvas::*;

const COLOR: Color = Color::Rgba(1.0, 0.0, 0.8, 1.0);
const ANGLE: f32 = 10.0;
const DEPTH: i32 = 11;
const LINE_WIDTH: f32 = 1.0;


fn main() {
    let branch_props: BranchProps = BranchProps {
                        color: Color::Rgba(1.0, 0.2, 0.2, 1.0),
                        length: 500,
                        curve: 0.8};


    with_2d_graphics(|| {
        let canvas = create_canvas_window("Fractal Tree Generator");

        canvas.draw(|gc| {
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);
            gc.line_width(LINE_WIDTH);
            gc.stroke_color(COLOR);
           
          //  gc.new_path();
           // gc.move_to(500.0, 150.0);
           // gc.line_to(500.0 , 300.0);
          //  gc.close_path();
          //  gc.stroke();    

            branch(gc, 500.0, 300.0, 0.0, DEPTH);
        });
    

    })
}


fn branch(gc: &mut CanvasGraphicsContext, x : f32 ,y : f32, angle: f32, n: i32) {
            let x2 = x + angle.to_radians().sin() * n as f32 * 15.0;
            let y2 = x - angle.to_radians().cos() * n as f32 * 15.0;
            gc.new_path();
            gc.move_to(x, y);
            gc.line_to(x2 , y2);
            gc.close_path();
            gc.stroke();    

            println!("x: {} y: {}", x, y);
            if n == 0 {
              return;
             }
            branch(gc, x2, y2, angle - ANGLE, n-1);
            branch(gc, x2, y2, angle + ANGLE, n-1);
}


struct BranchProps {
    color: Color,
    length: i32,
    curve: f32,
}

struct Tree {
    branches: Vec<BranchProps>,
    n: i32,
}
