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

            gc.new_path();
            gc.move_to(500.0, 100.0);
            gc.line_to(500.0, 200.0);
            gc.fill_color(Color::Rgba(1.0, 0.0, 0.8, 1.0));
            gc.fill();
        });
    
        branch(&canvas, branch_props);

    })
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


fn branch(canvas: &flo_draw::canvas::Canvas, branch: BranchProps) {
    
}

fn add(num :i32) {

}