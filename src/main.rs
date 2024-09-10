use nannou::noise::{BasicMulti, Seedable};
use nannou::noise::NoiseFn;
use nannou::color::{IntoLinSrgba, LinSrgba};
use nannou::prelude::*; 

fn main() { 
	nannou::sketch(view).run();
} 

fn view(app: &App, frame: Frame) {
	let draw = app.draw(); 
        let rect = app.window_rect();

        app.set_loop_mode(LoopMode::loop_once());
        app.main_window().capture_frame("perlin_2d.png");

	draw.background().color(srgba8(230, 217, 162, 255)); 

        for j in ((rect.bottom() + 10.) as i32 ..(rect.top() - 10.) as i32).step_by(11) {
                let noise = BasicMulti::new().set_seed(random());
                let weight = 111.;
                let noise_h = 888.;
                let ld = random_range(0, 3);
                let pallete = vec![
                        //srgba8(230, 217, 162, 255).into_lin_srgba(),
                        srgba8(255, 218, 118, 255).into_lin_srgba(),
                        srgba8(255, 140, 158, 255).into_lin_srgba(),
                        srgba8(255, 78, 136, 255).into_lin_srgba()
                        ];

                let mut rnjesus: LinSrgba = pallete[random_range(0, 2)];
                
                let points = (0..=rect.w() as usize).map(|i| {

                    let x = i as f32 - (rect.w() / 2.);

                    let noise_y = noise.get([x as f64 / noise_h, 0.]) as f32;
                    let y = map_range(noise_y, -1., 1., rect.top() - 111., rect.bottom() + 111.);
                    
                    if ld == 0 {
                            let clr = lin_srgba(rnjesus.red + (noise_y/weight)
                                        , rnjesus.green + (noise_y/weight)
                                        , rnjesus.blue + (noise_y/weight)
                                        , rnjesus.alpha// + (noise_y/weight)
                                        );
                            rnjesus = clr;
                            
                            (pt2(x, y + j as f32), clr)
                   }else {
                            let clr = lin_srgba(rnjesus.red - (noise_y/weight)
                                        , rnjesus.green - (noise_y/weight)
                                        , rnjesus.blue - (noise_y/weight)
                                        , rnjesus.alpha// + (noise_y/weight)
                                        );
                            rnjesus = clr;

                            (pt2(x, y + j as f32), clr)
                   }
                });

                draw.polyline().weight(3.0).points_colored(points);    
        };

	draw.to_frame(app, &frame).unwrap(); 
}
