use flo_draw::*;
use flo_draw::canvas::*;



fn main() {
    let branch_props: BranchProps = BranchProps {
                        color: Color::Rgba(1.0, 0.2, 0.2, 1.0),
                        length: 500,
                        curve: 0.8};


    with_2d_graphics(|| {
        let canvas = create_canvas_window("Fractal Tree Generator");

        canvas.draw(|gc| {
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);
            gc.line_width(5.0);
            gc.stroke_color(Color::Rgba(1.0, 0.0, 0.8, 1.0));
           
            branch(gc, 500.0, 500.0, 1.2, 20);
        });
    

    })
}


fn branch(gc: &mut CanvasGraphicsContext, x : f32 ,y : f32, reduction_factor: f32,mut n: i32) {
            gc.new_path();
            gc.circle(x, y, n as f32 * 20.0);
           // gc.line_to(x , y);
           // gc.close_path();
            gc.stroke();

            println!("x: {} y: {}", x, y);
            if n == 0 {
              return;
             }
            n -= 1;
            branch(gc, x, y, reduction_factor, n )
    
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
