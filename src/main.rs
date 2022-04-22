use rand::prelude::*;
//use flo_curves::bezier::*;
use flo_draw::*;
use flo_draw::canvas::*;
use flo_draw::binding::*;

const COLOR: Color = Color::Rgba(1.0, 0.0, 0.8, 1.0);
const ANGLE_START: f32 = 60.0;
const DEPTH: i32 = 11;
const LINE_WIDTH: f32 = 2.5;



fn main() {

    with_2d_graphics(|| {

        let mut window_properties       = WindowProperties::from(&"Fractal Tree Generator");
        //window_properties.mouse_pointer = BindRef::from(bind(MousePointer::None));
        
        let (canvas, events) = create_canvas_window_with_events(window_properties);

        canvas.draw(|gc| {
            let SCR_HEIGHT = 100.0*DEPTH as f32;
            gc.line_width(1.0);
            gc.stroke_color(Color::Rgba(1.0, 1.0, 1.0, 1.0));
           // gc.draw_rect();



            gc.line_width(LINE_WIDTH);
            gc.stroke_color(COLOR);
            gc.canvas_height(SCR_HEIGHT);
            gc.center_region(0.0, 0., SCR_HEIGHT, SCR_HEIGHT);
            
            draw_tree(gc, SCR_HEIGHT/2.0, SCR_HEIGHT/2.0, 0.0, DEPTH);
           
            ()
        });

    })
}

    /// Populates the branches of a tree. and draws tree
    pub fn draw_tree(gc: &mut CanvasGraphicsContext, x : f32 ,y : f32, angle: f32, n: i32) {
        let mut rng = thread_rng();
        let color = Color::Rgba(rng.gen_range(0.1..=1.0), rng.gen_range(0.1..=1.0), rng.gen_range(0.1..=1.0), 1.0);

        let x2 = x + angle.to_radians().sin() * n as f32 * 10.0;
        let y2 = y + angle.to_radians().cos() * n as f32 * 10.0;

       // let bpb = BezierPathBuilder::start(Branch{x: x, y: y});
        //bpb.line_to(Branch{x: x2, y: y2});
        //let path = bpb.build();
        //slower than having stroking just once but this allows customisation of branches
        //gc.draw_bzier_curve();

        gc.new_path();
        gc.stroke_color(color);
        gc.move_to(x, y);
        gc.line_to(x2 , y2);
        gc.stroke();

        if n == 0 {
          return;
         }
        draw_tree(gc, x2, y2, angle - ANGLE_START, n-1);
        draw_tree(gc, x2, y2, angle + ANGLE_START, n-1);
}

