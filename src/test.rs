use core::f32;
//use std::{intrinsics::{}, iter};

use geom::range;
use nannou::{draw, lyon::geom::euclid::{approxord::max, default}, prelude::*};
use rand::prelude::*;
const N_PARTICLES: usize = 10;

fn main() {
    nannou::sketch(view).run()
}

//let point_charges;

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let window = app.main_window();
    let win = window.rect();
    draw.background().rgb(1.0, 1.0, 1.0);

    let WIDTH = win.w();
    let HEIGHT = win.h();

    // 100-step and 10-step grids.
    //draw_grid(&draw, &win, 100.0, 1.0);
    //draw_grid(&draw, &win, 25.0, 0.5);

    // Crosshair.
    

    // Crosshair text.
    // let top = format!("{:.1}", win.top());
    // let bottom = format!("{:.1}", win.bottom());
    // let left = format!("{:.1}", win.left());
    // let right = format!("{:.1}", win.right());
    // let x_off = 30.0;
    // let y_off = 20.0;
    // draw.text("0.0")
    //     .x_y(15.0, 15.0)
    //     .color(crosshair_color)
    //     .font_size(14);
    // draw.text(&top)
    //     .h(win.h())
    //     .font_size(14)
    //     .align_text_top()
    //     .color(crosshair_color)
    //     .x(x_off);
    // draw.text(&bottom)
    //     .h(win.h())
    //     .font_size(14)
    //     .align_text_bottom()
    //     .color(crosshair_color)
    //     .x(x_off);
    // draw.text(&left)
    //     .w(win.w())
    //     .font_size(14)
    //     .left_justify()
    //     .color(crosshair_color)
    //     .y(y_off);
    // draw.text(&right)
    //     .w(win.w())
    //     .font_size(14)
    //     .right_justify()
    //     .color(crosshair_color)
    //     .y(y_off);

    // Window and monitor details.
    if let Some(monitor) = window.current_monitor() {
        let w_scale_factor = window.scale_factor();
        //let m_scale_factor = monitor.scale_factor();
        let mon_phys = monitor.size();
        let mon = mon_phys.to_logical(w_scale_factor as f64);
        let mon_w: f32 = mon.width;
        let mon_h: f32 = mon.height;
        draw.background().rgb(100.0, 100.0, 100.0);

        // let text = format!(
        //     "
        // Window size: [{:.0}, {:.0}]
        // Window ratio: {:.2}
        // Window scale factor: {:.2}
        // Monitor size: [{:.0}, {:.0}]
        // Monitor ratio: {:.2}
        // Monitor scale factor: {:.2}
        // ",
        //     win.w(),
        //     win.h(),
        //     win.w() / win.h(),
        //     w_scale_factor,
        //     mon_w,
        //     mon_h,
        //     mon_w / mon_h,
        //     m_scale_factor
        // );
        // let pad = 6.0;
        // draw.text(&text)
        //     .h(win.pad(pad).h())
        //     .w(win.pad(pad).w())
        //     .line_spacing(pad)
        //     .font_size(14)
        //     .align_text_bottom()
        //     .color(crosshair_color)
        //     .left_justify();

        // Ellipse at mouse.
        draw.ellipse().wh([5.0; 2].into()).xy(app.mouse.position());

     

        for x in 0..(WIDTH/20.0) as u16 {
            for y in 0..(HEIGHT/20.0) as u16 {

                let x_cast = x as f32;
                let y_cast = y as f32;

                let vec_x = field_simulate(x_cast, y_cast)[0]*15.0;
                let vec_y = field_simulate(x_cast, y_cast)[1]*15.0;

                let x2 = x_cast + vec_x;
                let y2 = y_cast + vec_y;

                //let offset = 2;

                let arrow_start = pt2(x_cast, y_cast);
                let arrow_end: Vec2 = pt2(x2, y2);

                draw.arrow().start(arrow_start).end(arrow_end);

            }
    
        }

        fluid_simulate(app, WIDTH, HEIGHT);

        // Mouse position text.
        let mouse = app.mouse.position();
        let pos = format!("[{:.1}, {:.1}]", mouse.x, mouse.y);
        draw.text(&pos)
            .xy(mouse + vec2(0.0, 20.0))
            .font_size(14)
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

// fn draw_grid(draw: &Draw, win: &Rect, step: f32, weight: f32) {
//     let step_by = || (0..).map(|i| i as f32 * step);
//     let r_iter = step_by().take_while(|&f| f < win.right());
//     let l_iter = step_by().map(|f| -f).take_while(|&f| f > win.left());
//     let x_iter = r_iter.chain(l_iter);
//     for x in x_iter {
//         draw.line()
//             .weight(weight)
//             .points(pt2(x, win.bottom()), pt2(x, win.top()));
//     }
//     let t_iter = step_by().take_while(|&f| f < win.top());
//     let b_iter = step_by().map(|f| -f).take_while(|&f| f > win.bottom());
//     let y_iter = t_iter.chain(b_iter);
//     for y in y_iter {
//         draw.line()
//             .weight(weight)
//             .points(pt2(win.left(), y), pt2(win.right(), y));
//     }
// }



fn field_simulate(x: f32, y: f32) -> Vec<f32> {
    let point_charges: Vec<Vec<f32>> = vec![vec![(rand::random::<f32>() - 0.5)*500.0, (rand::random::<f32>()-0.5)*500.0, (rand::random::<f32>()-0.5)*5.0]];

    let half: f32 = 0.5;
    //[x, y, charge]
    let mut vec_x: f32 = 0.0;
    let mut vec_y: f32 = 0.0;

    const K: f32= 70.0;

    for i in 0..point_charges.len() {
        let x_diff = x - point_charges[i][0];
        let y_diff = y - point_charges[i][1];
    
        let radius = f32::sqrt((x_diff*x_diff) + (y_diff*y_diff));
    

        let vector_magnitude = K * (point_charges[i][2] / (radius*radius));

        let angle = (y_diff / x_diff).atan();

        let anti_radius = 1.0 / radius;
        let point_vec_x: f32 = vector_magnitude * (signum(x_diff) * abs(f32::cos(angle))) * anti_radius;
        let point_vec_y: f32 = vector_magnitude * (signum(y_diff) * abs(f32::sin(angle))) * anti_radius;
    
    
        vec_x += point_vec_x;
        vec_y += point_vec_y;
        

    }
    let vec_out: Vec<f32> = vec![vec_x, vec_y];
    return vec_out;
}

fn fluid_simulate(app: &App, w: f32, h: f32) {
    let draw = app.draw();
    //draw.background().color(BLACK);
    //const N_PARTICLES: usize = 10;
    //const base_particle: Vec<f32> = vec![0.0; 6];


    let mut fluids_array: Vec<Vec<f32>> = vec![vec![(rand::random::<f32>() - 0.5)*w, (rand::random::<f32>() - 0.5)*h, 0.0, 0.0, 500.0, 0.0]; N_PARTICLES];
    //                                              [x, y, xVelo, yVelo, life length, lifetime tracker]

    for i in 0..fluids_array.len() {
        fluids_array[i][5] += 1.0;

        if fluids_array[i][5] > fluids_array[i][4] {
            fluids_array[i] =  vec![(rand::random::<f32>() - 0.5)*w, (rand::random::<f32>() - 0.5)*h, 0.0, 0.0, 0.0, 0.0]; 
        }

        let particle_x = fluids_array[i][0];        
        let particle_y = fluids_array[i][1];

        fluids_array[i][2] += field_simulate(particle_x, particle_y)[0]*0.1;
        fluids_array[i][3] += field_simulate(particle_x, particle_y)[1]*0.1;

        let x_velo: f32 = fluids_array[i][2];
        let y_velo: f32 = fluids_array[i][3];


        let max_speed: f32 = 4.0;

        if abs(x_velo) > max_speed {
            fluids_array[i][2] = signum(x_velo) * max_speed;
        }

        if abs(y_velo) > max_speed {
            fluids_array[i][3] = signum(y_velo) * max_speed;
        }

        fluids_array[i][0] += x_velo;
        fluids_array[i][1] += y_velo;

        let line_start = pt2(particle_x, particle_y);
        let line_end= pt2(particle_x-x_velo, particle_y-y_velo);

        draw.line().start(line_start).end(line_end).weight(2.0).color(GRAY);

        draw.ellipse().w(5.0).h(5.0).x(particle_x).y(particle_y);

    }
}