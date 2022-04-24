use rand::prelude::*;
use flo_draw::*;
use flo_draw::canvas::*;
use flo_draw::binding::*;
use std::*;

use clap::*;

const COLOR: Color = Color::Rgba(1.0, 0.0, 0.8, 1.0);
const ANGLE_START: f32 = 15.0;
const DEPTH: i32 = 15;
const LINE_WIDTH: f32 = 2.0;

struct TreeProps {
    angle_change: f32,
    depth: i32,
}

fn main() {
    let app = Command::new("hello-clap")
        .version("0.1")
        .about("draws fractal tree")
        .author("gatsey");

    let depth = Arg::new("depth")
        .long("depth") 
        .default_value("10")
        .help("factorial for branch iteration, default 10 i32")
        .required(true);
    
    let angle_start = Arg::new("angle")
        .long("angle") 
        .default_value("10.0")
        .help("change angle for branch iteration, default 20.0 float")
        .required(true);

    // now add in the argument we want to parse
    let app = app.arg(angle_start);
    let app = app.arg(depth);

    let matches = app.get_matches();

    // Extract the actual name
    if let Some(n) = matches.value_of("angle") {
        //let angle = n.parse::<f32>().expect("angle is not a valid float");
        println!("{:?}", n);
    }
    if let Some(i) = matches.value_of("depth") {
     //let depth = i.parse::<i32>().expect("depth is not a valid int");
         println!("{:?}", i);
    }

    let tree: TreeProps = TreeProps {
        angle_change: 20.0,
        depth: 15,
    };

    with_2d_graphics( move || {

        let mut window_properties       = WindowProperties::from(&"Fractal Tree Generator");
      //  window_properties.mouse_pointer = BindRef::from(bind(MousePointer::None));
        
        
        let (canvas, events) = create_canvas_window_with_events(window_properties);



        canvas.draw(|gc| {
            let SCR_HEIGHT = 100.0*DEPTH as f32;
            gc.stroke_color(Color::Rgba(1.0, 1.0, 1.0, 1.0));
           // gc.draw_rect();

            gc.line_width(LINE_WIDTH);
            
            gc.canvas_height(SCR_HEIGHT);
            gc.center_region(0.0, 0., SCR_HEIGHT, SCR_HEIGHT);
            
            draw_tree(gc, SCR_HEIGHT/2.0, SCR_HEIGHT/10.0,  tree.depth, 0.0, tree.angle_change);
           
            ()
        });

    })
}

fn get_color(n: i32) -> Color {
    if n%2 == 0 {
       return Color::Rgba(1.0, 0.0, 1.0, 1.0);
    } else if n% 7 == 0 {
        return Color::Rgba(0.0, 1.0, 1.0, 1.0); 
    } else if n%3 == 0 {
        return Color::Rgba(0.0, 1.0, 1.0, 1.0);
    }
    
    Color::Rgba(1.0, 1.0, 1.0, 1.0) 
}

    /// Populates the branches of a tree. and draws tree
    pub fn draw_tree(gc: &mut CanvasGraphicsContext, x : f32 ,y : f32, n: i32, angle: f32, angle_change:f32) {
        let mut color = get_color(n);

        let x2 = x + angle.to_radians().sin() * n as f32 * 10.0;
        let y2 = y + angle.to_radians().cos() * n as f32 * 10.0;

        //let bpb = BezierPathBuilder::start(Branch{x: x, y: y});
       // bpb.line_to(Branch{x: x2, y: y2});
       // let path = bpb.build()
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
        draw_tree(gc, x2, y2, n-1, angle - angle_change, angle_change);
        draw_tree(gc, x2, y2, n-1, angle + angle_change, angle_change);
}

struct Branch  {
    x: f64,
    y: f64,
}


