use nannou::{event::ElementState, prelude::*};
use rand::prelude::*;
use real::Real;
use std::{sync::{Once, ONCE_INIT}, vec};

const N_PARTICLES: usize = 10;
fn main() {
    nannou::sketch(view).run()
}


static INIT: Once = ONCE_INIT;
static mut point_charges_init: Option<Vec<Vec<f32>>> = None;
static mut fluids_init: Option<Vec<Vec<f32>>> = None;

fn initialize_charges() -> Vec<Vec<f32>> {
    INIT.call_once(|| {
        unsafe {
            point_charges_init = Some(vec![
                /*(vec![(rand::random::<f32>() - 0.5)*500.0, (rand::random::<f32>()-0.5)*500.0, (rand::random::<f32>()-0.5)*5.0]), 
                (vec![(rand::random::<f32>() - 0.5)*500.0, (rand::random::<f32>()-0.5)*500.0, (rand::random::<f32>()-0.5)*5.0]), 
                (vec![(rand::random::<f32>() - 0.5)*500.0, (rand::random::<f32>()-0.5)*500.0, (rand::random::<f32>()-0.5)*5.0])*/
                (vec![300.0 as f32, 300.0 as f32, 2.0 as f32])
                ]
            );
        }
    });
    unsafe {
        point_charges_init.clone().unwrap() // Access the initialized variable
    }
}

// fn initialize_fluids() -> Vec<Vec<f32>> {
//     INIT.call_once(|| {
//         unsafe {
//             fluids_init = Some(vec![vec![(rand::random::<f32>() - 0.5)*500.0, (rand::random::<f32>() - 0.5)*500.0, 0.0, 0.0, 500.0, 0.0]; N_PARTICLES]);
//         }
//     });
//     unsafe {
//         fluids_init.clone().unwrap() // Access the initialized variable
//     }
// }

//let mut fluids_array: Vec<Vec<f32>> = vec![vec![(rand::random::<f32>() - 0.5)*w, (rand::random::<f32>() - 0.5)*h, 0.0, 0.0, 500.0, 0.0]; N_PARTICLES];


fn view(app: &App, frame: Frame) {
    let point_charges = initialize_charges();
    // let mut fluid_particles = initialize_fluids();
    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(CORNFLOWERBLUE);

    // Draw a purple triangle in the top left half of the window.
    let win = app.window_rect();
    // draw.tri()
    //     .points(win.bottom_left(), win.top_left(), win.top_right())
    //     .color(VIOLET);

    // Draw an ellipse to follow the mouse.
    let t = app.time;

    //if t < 1.0 {
        
    
    
    // draw.ellipse()
    //     .x_y(app.mouse.x * t.cos(), app.mouse.y)
    //     .radius(win.w() * 0.125 * t.sin())
    //     .color(RED);

    // Draw a line!
    // draw.line()
    //     .weight(10.0 + (t.sin() * 0.5 + 0.5) * 90.0)
    //     .caps_round()
    //     .color(PALEGOLDENROD)
    //     .points(win.top_left() * t.sin(), win.bottom_right() * t.cos());

    // Draw a quad that follows the inverse of the ellipse.
    // draw.quad()
    //     .x_y(-app.mouse.x, app.mouse.y)
    //     .color(DARKGREEN)
    //     .rotate(t);

    // Draw a rect that follows a different inverse of the ellipse.
    // draw.rect()
    //     .x_y(app.mouse.y, app.mouse.x)
    //     .w(app.mouse.x * 0.25)
    //     .hsv(t, 1.0, 1.0);

        // let WIDTH = win.w();
        // let HEIGHT = win.h();

        draw.ellipse().wh([5.0; 2].into()).xy(app.mouse.position());

     

        for x in (-win.w()/2.0/20.0) as i16..(win.w()/2.0/20.0) as i16 {
            for y in (-win.h()/2.0/20.0) as i16..(win.h()/2.0/20.0) as i16  {

                let x_cast = x as f32 * 20.0;
                let y_cast = y as f32 * 20.0;

                let field_sim: Vec<f32> = field_simulate(x_cast, y_cast, &point_charges);

                let mut vec_x = field_sim[0]*150.0;
                let mut vec_y = field_sim[1]*150.0;
                let biggest_influence: usize = field_sim[2] as usize;

                // if abs(vec_x) > abs(x_cast - point_charges[biggest_influence][0]) {
                //     vec_x = signum(vec_x) * abs(x_cast - point_charges[biggest_influence][0]) * f32::cos((field_sim[4]/field_sim[3]).atan());
                // }
                // if abs(vec_y) > abs(y_cast - point_charges[biggest_influence][1]) {
                //     vec_y = signum(vec_y) * abs(y_cast - point_charges[biggest_influence][1]) * f32::sin((field_sim[4]/field_sim[3]).atan());
                // }


                // if abs(vec_y) > abs(y_cast - point_charges[biggest_influence][1]) {
                //     vec_y = signum(vec_y) * abs(y_cast - point_charges[biggest_influence][1]);
                // }
                let mut resize_vector: Vec<f32> = field_sim;

                while resize_vector.len() > 2 {
                    resize_vector.pop();
                }

                if abs(vec_x) > abs(x_cast - point_charges[biggest_influence][0]) || abs(vec_y) > abs(y_cast - point_charges[biggest_influence][1]){
                    let mut ratio: f32 = 0.0;
                    if vec_x > vec_y {
                        ratio =  (x_cast - point_charges[biggest_influence][0]) / abs(vec_x);
                        //resize_vector.resize(new_len, value)
                        vec_x *= ratio;
                        vec_y *= ratio;
                    }
                    else {
                        ratio = (y_cast - point_charges[biggest_influence][1]) / abs(vec_y);
                        vec_x *= ratio;
                        vec_y *= ratio;
                    }

                    
                }

                let x2 = x_cast + vec_x;
                let y2 = y_cast + vec_y;

                //let offset = 2;

                let arrow_start = pt2(x_cast, y_cast);
                let mut arrow_end: Vec2 = pt2(x2, y2);

                // if abs(vec_x) > abs(x_cast - point_charges[biggest_influence][0]) || abs(vec_y) > abs(y_cast - point_charges[biggest_influence][1]){
                //     let mut ratio: f32 = 0.0;
                //     if vec_x > vec_y {
                //         ratio =  (x_cast - point_charges[biggest_influence][0]) / vec_x;
                //         arrow_end = pt2(point_charges[biggest_influence][0], (y_cast+vec_y)*ratio);
                //     }
                //     else {
                //         ratio = (y_cast - point_charges[biggest_influence][1]) / vec_y;

                //         arrow_end = pt2((x_cast+vec_x)*ratio, point_charges[biggest_influence][1] );

                //     }

                    
                // }

                

                draw.arrow().start(arrow_start).end(arrow_end);

                //draw.ellipse().x(x_cast).y(y_cast).w(2.0).h(2.0);

            }
    
        }

        for i in 0..point_charges.len() {
            draw.ellipse().x_y(point_charges[i][0], point_charges[i][1]).w_h(10.0, 10.0);
        }

         draw.ellipse().x(-win.w()/2.0).y(0.0).w_h(4.0, 4.0);

        // fluid_simulate(win.w(), win.h(), &point_charges, fluid_particles);

        // Mouse position text.
        let mouse = app.mouse.position();
        let end_point: Vec2 = 
        pt2(
            (
                mouse.x as f32 + 
                field_simulate(
                    mouse.x, 
                    mouse.y, 
                    &point_charges)[0]*150.0
                ) , 
            (
                mouse.y as f32 + 
                field_simulate(
                    mouse.x, 
                    mouse.y, 
                    &point_charges)[1]*150.0
                )
            );

        draw.arrow().start(mouse).end(end_point);

        let pos = format!("[{:.1}, {:.1}]", mouse.x, mouse.y);
        draw.text(&pos)
            .xy(mouse + vec2(0.0, 20.0))
            .font_size(14)
            .color(WHITE);


    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
/* 
fn field_init(x: f32, y: f32, point_charges: &Vec<Vec<f32>>, t: f32) -> Vec<f32> {
    // if t < 1.0 {
    //     let point_charges: Vec<Vec<f32>> = vec![vec![(rand::random::<f32>() - 0.5)*500.0, (rand::random::<f32>()-0.5)*500.0, (rand::random::<f32>()-0.5)*5.0]];
    // }
    // else {
    //     let point_charges: Vec<Vec<f32>> = input_points;
    // }

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
    //let vec_out_init: Vec<f32>;
    let vec_out: Vec<f32>;

    vec_out = vec![vec_x, vec_y];
    

    return vec_out;
}
*/

fn field_simulate(x: f32, y: f32, point_charges: &Vec<Vec<f32>>) -> Vec<f32> {
    // if t < 1.0 {
    //     let point_charges: Vec<Vec<f32>> = vec![vec![(rand::random::<f32>() - 0.5)*500.0, (rand::random::<f32>()-0.5)*500.0, (rand::random::<f32>()-0.5)*5.0]];
    // }
    // else {
    //     let point_charges: Vec<Vec<f32>> = input_points;
    // }

    let half: f32 = 0.5;
    //[x, y, charge]
    let mut vec_x: f32 = 0.0;
    let mut vec_y: f32 = 0.0;
    let mut greatest_magnitude: f32 = 0.0;
    let mut greatest_magnitude_index: f32 = 0.0;
    let mut great_inf_x_diff: f32 = 0.0;
    let mut great_inf_y_diff: f32 = 0.0;


    const K: f32= 70.0;

    for i in 0..point_charges.len() {
        let x_diff = x - point_charges[i][0];
        let y_diff = y - point_charges[i][1];
    
        let radius = f32::sqrt((x_diff*x_diff) + (y_diff*y_diff));
    

        let vector_magnitude = K * (point_charges[i][2] / (radius*radius));

        let angle = (y_diff / x_diff).atan();

        let anti_radius = radius / 10.0;
        let point_vec_x: f32 = vector_magnitude * (signum(x_diff) * abs(f32::cos(angle))) * anti_radius;
        let point_vec_y: f32 = vector_magnitude * (signum(y_diff) * abs(f32::sin(angle))) * anti_radius;
    
    
        vec_x += point_vec_x;
        vec_y += point_vec_y;
        
        if vector_magnitude < greatest_magnitude {
            greatest_magnitude = vector_magnitude;
            greatest_magnitude_index = i as f32;
            great_inf_x_diff = x_diff;
            great_inf_y_diff = y_diff;
        }

    }
    let vec_out: Vec<f32>;
    
        vec_out = vec![vec_x, vec_y, greatest_magnitude_index, great_inf_x_diff, great_inf_y_diff];
    

    return vec_out;
}




fn fluid_simulate(w: f32, h: f32, charges: &Vec<Vec<f32>>, mut fluids_array: Vec<Vec<f32>>){
    //let draw = app.draw();
    //draw.background().color(BLACK);
    //const N_PARTICLES: usize = 10;
    //const base_particle: Vec<f32> = vec![0.0; 6];

    //                                      fluids_arr -> [x, y, xVelo, yVelo, life length, lifetime tracker]

    for i in 0..fluids_array.len() {
        fluids_array[i][5] += 1.0;

        if fluids_array[i][5] > fluids_array[i][4] {
            fluids_array[i] =  vec![(rand::random::<f32>() - 0.5)*w, (rand::random::<f32>() - 0.5)*h, 0.0, 0.0, 0.0, 0.0]; 
        }

        let particle_x = fluids_array[i][0];        
        let particle_y = fluids_array[i][1];

        fluids_array[i][2] += field_simulate(particle_x, particle_y, &charges)[0]*0.1;
        fluids_array[i][3] += field_simulate(particle_x, particle_y, &charges)[1]*0.1;

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

        //draw.line().start(line_start).end(line_end).weight(2.0).color(GRAY);

        //draw.ellipse().w(5.0).h(5.0).x(particle_x).y(particle_y);
        
        
    }
    
}