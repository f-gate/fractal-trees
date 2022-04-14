use flo_draw::*;
use flo_draw::canvas::*;

fn main() {
    with_2d_graphics(|| {
        let canvas = create_canvas_window("hello world!");
    })
}
