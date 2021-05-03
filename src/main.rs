use minifb::{Key, Scale, Window, WindowOptions};

const DIM: usize = 320;
const N_ITERATIONS: u32 = 100;
const INFIN: f64 = 16.0;

struct Observer {
    x: f64,
    y: f64,
    zoom:f64,
    speed:f64,
    zoom_speed:f64
}

impl Observer {
    fn vert_move(&mut self, dir: i8){
        self.y+= DIM as f64 * (f64::powf(2.0,-(self.zoom + 1.0))) * self.speed as f64 * dir as f64;
    }
    fn horz_move(&mut self, dir: i8){
        self.x += DIM as f64 * (f64::powf(2.0,-(self.zoom + 1.0))) * self.speed as f64 * dir as f64;
    }
    fn change_zoom(&mut self, mag: i8){
        self.zoom += self.zoom_speed * mag as f64;
    }
    fn get_bounds(&self) -> (f64, f64, f64){
        let gap = 8.0 * (f64::powf(2.0,-(self.zoom + 1.0)));
        let (min_x,min_y) = (self.x - gap, self.y - gap);
        let interval = (2.0 * gap) / DIM as f64;
        (min_x, min_y, interval)     
    }
}

pub struct Color {
    frac: f32,
    r: u8,
    g: u8,
    b: u8
}

fn main() {

    println!("Use WASD to move around, LShift to zoom in, LCtrl to zoom out.");

    let mut buffer: Vec<u32> = vec![0; DIM * DIM];

    let mut window = Window::new(
        "Mandelbrot Explorer",
        DIM,DIM,
        WindowOptions {
            resize: false,
            scale: Scale::X2,
            ..WindowOptions::default()
        }
    ).expect("Error in opening window.");

    window.limit_update_rate(Some(std::time::Duration::from_micros(41500))); // 24 fps

    let gradient:Vec<Color> = vec![
        Color {frac: 0.0   , r: 0  , g: 7  , b: 100},
        Color {frac: 0.16  , r: 32 , g: 107, b: 203},
        Color {frac: 0.42  , r: 237, g: 255, b: 255},
        Color {frac: 0.6425, r: 255, g: 170, b: 0  },
        Color {frac: 0.8575, r: 0  , g: 70 , b: 0  },
        Color {frac: 1.0   , r: 0  , g: 7  , b: 100}
    ];

    let mut observer = Observer {
        x:-1.0,
        y:0.0,
        zoom:1.0,
        speed:7e-4,
        zoom_speed: 6e-2
    };
    let mut is_changed = true;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        check_input(&mut window, &mut observer, &mut is_changed);
        if is_changed{
            is_changed = false;
            let (min_x, min_y, interval) = observer.get_bounds();
            for j in 0..DIM{
                let y = min_y + j as f64 *interval;
                for i in 0..DIM{
                    let x = min_x + i as f64 * interval;

                    let mut n = 1;
                    let (mut temp_real, mut temp_complex) = (x,y);
                    while n < N_ITERATIONS{
                        let temp = temp_real;
                        temp_real = temp_real * temp_real - temp_complex * temp_complex + x;
                        temp_complex = 2.0 * temp * temp_complex + y;
                        if temp_real + temp_complex > INFIN{
                            break;
                        }
                        n += 1;
                    }
                    buffer[j*DIM + i] = color_map(n,N_ITERATIONS, &gradient);
                }       
            }
        } 
        window.update_with_buffer(&buffer, DIM, DIM).unwrap();  
    }
}

fn check_input(window: &mut Window, observer: &mut Observer, is_changed: &mut bool){
    if window.is_key_down(Key::W){
        observer.vert_move(-1);
        *is_changed = true;
    }
    if window.is_key_down(Key::S){
        observer.vert_move(1);
        *is_changed = true;
    }
    if window.is_key_down(Key::A){
        observer.horz_move(-1);
        *is_changed = true;
    }
    if window.is_key_down(Key::D){
        observer.horz_move(1);
        *is_changed = true;
    }
    if window.is_key_down(Key::LeftShift){
        observer.change_zoom(1);
        *is_changed = true;
    }
    if window.is_key_down(Key::LeftCtrl){
        observer.change_zoom(-1);
        *is_changed = true;
    }
}

pub fn color_map(n: u32, total: u32, gradient: &Vec<Color>) -> u32{
    let frac = n as f32 / total as f32;
    for i in 1..gradient.len()-1{
        if gradient[i].frac >= frac{
            let prev_color = &gradient[i-1];
            let next_color = &gradient[i];
            let lerp_frac = (frac - prev_color.frac) as f32 / (next_color.frac - prev_color.frac) as f32;
            let r = (prev_color.r as f32 + ((next_color.r as i32 - prev_color.r as i32) as f32 * lerp_frac)) as u8;
            let g = (prev_color.g as f32 + ((next_color.g  as i32 - prev_color.g as i32) as f32 * lerp_frac)) as u8;
            let b = (prev_color.b as f32 + ((next_color.b  as i32 - prev_color.b as i32) as f32 * lerp_frac)) as u8;
            return as_u32_rgb(r, g, b);
        }
    }
    return as_u32_rgb(0, 0, 0);
}

fn as_u32_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b // returns 0x00rrggbb
}
