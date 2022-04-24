use rand::prelude::*;
use flo_draw::*;
use flo_draw::canvas::*;
use flo_draw::binding::*;
use std::*;

use clap::*;

const COLOR: Color = Color::Rgba(1.0, 0.0, 0.8, 1.0);
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

    let depth_arg = Arg::new("depth")
        .long("depth") 
        .default_value("14")
        .help("factorial for branch iteration, default 10 i32");
    
    let angle_start_arg = Arg::new("angle")
        .long("angle") 
        .default_value("20.0")
        .help("change angle for branch iteration, default 20.0 float");

    let app = app.arg(angle_start_arg);
    let app = app.arg(depth_arg);

    let matches = app.get_matches();

    let mut tree: TreeProps = TreeProps {angle_change: 0.0, depth: 0};

        if let Ok(f) = matches.value_of("angle").unwrap().parse::<f32>() {
            tree.angle_change = f;
        }
    
        if let Ok(n) = matches.value_of("depth").unwrap().parse::<i32>() {
            tree.depth = n;
        }

    with_2d_graphics( move || {
        let mut window_properties = WindowProperties::from(&"Fractal Tree Generator");
        
        let (canvas, events) = create_canvas_window_with_events(window_properties);

        canvas.draw(|gc| {
            let SCR_HEIGHT = 100.0*tree.depth as f32;
          
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
    } else if n%3 == 0 {
        return Color::Rgba(0.0, 1.0, 1.0, 1.0);
    }
    
    Color::Rgba(1.0, 0.0, 0.0, 1.0) 
}

    /// Populates the branches of a tree. and draws tree
    pub fn draw_tree(gc: &mut CanvasGraphicsContext, x : f32 ,y : f32, n: i32, angle: f32, angle_change:f32) {
        let color = get_color(n);

        let x2 = x + angle.to_radians().sin() * n as f32 * 10.0;
        let y2 = y + angle.to_radians().cos() * n as f32 * 10.0;

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


