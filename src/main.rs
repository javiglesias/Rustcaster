extern crate sdl2;
extern crate gl;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, BlendMode};
use sdl2::rect::Rect;

const _SCREEN_W: i32 = 1200;
const _SCREEN_H: i32 = 600;
const _MAP_X: i32 = 8;
const _MAP_Y: i32 = 8;
const _MAP_S: i32 = 64;

const _CELL_W: i32 = 90;
const _CELL_H: i32 = 90;

const _PI: f32 = 3.14159;
const _2PI: f32 = 2.0*_PI;
const _PI2: f32 = _PI/2.0;
const _3PI2: f32 = 3.0*_PI/2.0;
const _DR: f32 = 0.01745329; // one degree in radians

fn main() {
    const _APPLES8:     u8      = 8;
    const _APPLES16:    u16     = 16;
    const _APPLES32:    u32     = 32;
    const _APPLES64:    u64     = 64;
    const _APPLES128:   u128    = 128;

    const _PEARS8:      i8      = 8;
    const _PEARS16:     i16     = 16;
    const _PEARS32:     i32     = 32;
    const _PEARS64:     i64     = 64;
    const _PEARS128:    i128    = 128;

    const _MELON32:     f32     = 32.0;
    const _MELON64:     f64     = 64.0;

    const _CHERRY:      bool    = false;

    const _ORANGE:      char    = 'ª';

    const _GRAPES:      (u8, i8, f32, char) = (255, 127, 3.0, '4');

    
    let _secret_number: u32 = rand::thread_rng()
        .gen_range(1..=100);
    let _banana = 1;
    // loop {
    //     let mut _guess = String::new();
    //     println!("Guess a number:");
    //     io::stdin()
    //         .read_line(&mut _guess)
    //         .expect("Failed the read.");
    //     let _guess: u32 = match _guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_)  => {
    //             println!("Guess ONLY number.");
    //             continue;
    //         }
    //     };
    //     match _guess.cmp(&_secret_number) {
    //         Ordering::Less => println!("Guess higher next time."),
    //         Ordering::Greater => println!("Guess Lower next time."),
    //         Ordering::Equal =>
    //         { 
    //             println!("Guessed correctly");
    //             break;
    //         }
    //     }
    // }
    raycaster();
}

fn raycaster() -> bool
{
    let _MAP: Vec<i32> = vec![
        1,1,1,1,1,1,1,1,
        1,0,0,0,0,0,0,1,
        1,0,0,0,1,0,0,1,
        1,0,0,0,1,0,0,1,
        1,0,0,0,1,1,1,1,
        1,0,1,0,0,0,0,1,
        1,0,1,0,0,0,0,1,
        1,1,1,1,1,1,1,1
    ];
    let _context = sdl2::init().unwrap();
    let _video_subsys = _context.video().unwrap();
    let _window = _video_subsys.window("rustcaster", 
        _SCREEN_W as u32, _SCREEN_H as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut _canvas = _window.into_canvas()
        .present_vsync()    
        .build()
        .unwrap();
    let mut _event_pump = _context.event_pump().unwrap();
    
    let mut px: f32 = 70.0; // position player x
    let mut py: f32 = px; // player y
    let mut pa: f32 = 0.0; // player angle
    let mut pdx: f32 = pa.cos() * 5.0; // player delta x
    let mut pdy: f32 = pa.sin() * 5.0; // delta y

    'running : loop {
        for event in _event_pump.poll_iter() {
            match event {
                Event::Quit {..} | 
                Event::KeyDown { keycode : Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _ => {}
            }
            match event {
                Event::KeyDown { keycode : Some(Keycode::A), ..} => {
                    pa -= 0.1;
                    if pa < 0.0 {
                        pa += _2PI;
                    }
                    pdx = pa.cos()*5.0;
                    pdy = pa.sin()*5.0;
                },
                Event::KeyDown { keycode : Some(Keycode::D), ..} => {
                    pa += 0.1;
                    if pa > (_2PI) {
                        pa -= _2PI;
                    }
                    pdx = pa.cos()*5.0;
                    pdy = pa.sin()*5.0;
                },
                _ => {}
            }
            match event {
                Event::KeyDown { keycode : Some(Keycode::W), ..} => {
                    px += pdx;
                    py += pdy;
                },
                Event::KeyDown { keycode : Some(Keycode::S), ..} => {
                    px -= pdx;
                    py -= pdy;
                },
                _ => {}
            }
        }
        _canvas.set_draw_color(Color::RGB(0, 0, 10));
        _canvas.clear();
        // DRAW MAP
        let mut i: i32 = 0;
        loop { // HEIGHT
            let mut j: i32 = 0;
            loop {// WIDTH
                if _MAP[(i*_MAP_X+j) as usize] == 1 {
                    _canvas.set_draw_color(Color::RGB(100, 0, 255));
                } 
                if _MAP[(i*_MAP_X+j) as usize] == 0 {
                    _canvas.set_draw_color(Color::RGB(0, 255, 255));
                }
                if _MAP[(i*_MAP_X+j) as usize] > 1 {
                    _canvas.set_draw_color(Color::RGB(255, 255, 0));
                }
                let xo = j*_MAP_S;
                let yo = i*_MAP_S;
                _canvas.fill_rect(Rect::new(
                    xo as i32, 
                    yo as i32,
                    (_MAP_S + 1) as u32, 
                    (_MAP_S + 1) as u32));
                if j >= _MAP_X-1 { break; } // brekeamos el momento
                j+=1;
            }
            if i >= _MAP_Y-1 { break; }
            i+=1;
        }
        // --DRAW MAP
        //Draw Player
        //_canvas.set_draw_color(Color::RGB(255, 0, 0));
        //_canvas.fill_rect(Rect::new(px as i32, py as i32, 8, 8));
        // _canvas.draw_line(sdl2::rect::Point::new((px+4.0) as i32, (py+4.0) as i32),
        //     sdl2::rect::Point::new((px + pdx + 5.0) as i32, (py + pdy + 5.0) as i32));
        //_canvas.set_draw_color(Color::RGB(255, 200, 0));
        //--Draw Player
        // Draw 3d Rays
        // check Horizontal lines
        let mut ra:f32 = pa;
        let mut z = 0;
        let mut disT: f32 = 1.0;
        //_canvas.set_draw_color(Color::RGB(0, 200, 2));
        loop {
            if ra < 0.0 {
                ra += _2PI;
            }
            if ra > (_2PI) {
                ra -= _2PI;
            }
            let mut aTan = -1.0/ra.tan();
            let mut ry: f32 = 0.0;
            let mut rx: f32 = 0.0;
            let mut yo: f32 = 0.0;
            let mut xo: f32 = 0.0;
            let mut dof = 0;
            let mut mx: i32 = 0;
            let mut my: i32 = 0;
            let mut mp: i32 = 0;
            let mut r: i32 = 0;
            let mut distH: f32 = 1000000.0;
            let mut hx = px;
            let mut hy = py;
            if ra > _PI { // looking down
                ry = (py * _MAP_S as f32) / _MAP_S as f32 - 0.0001;
                rx = (py-ry) * aTan + px;
                yo = -_MAP_S as f32;
                xo = -yo*aTan;
            }
            if ra < _PI { // looking UP
                ry = (py * _MAP_S as f32) / _MAP_S as f32 + _MAP_S as f32;
                rx = (py-ry) * aTan + px;
                yo = _MAP_S as f32;
                xo = -yo*aTan;
            }
            if ra == 0.0 || ra == _PI {
                rx = px;
                ry = py;
                dof = 8;
            }
            loop {
                mx = (rx/_MAP_S as f32) as i32;
                my = (ry/_MAP_S as f32) as i32;
                mp = my * _MAP_X + mx;

                if mp >= 0 && mp < _MAP_X*_MAP_Y && _MAP[mp as usize] == 1 {
                    hx = rx;
                    hy = ry;
                    distH = dist(px, py, hx, hy, ra);
                    dof = 8; // hit wall
                } else {
                    rx += xo;
                    ry += yo;
                    dof += 1;
                }
                if dof >= 8 {
                    break;
                }
            }
            // _canvas.draw_line(sdl2::rect::Point::new((px+4.0) as i32, (py+4.0) as i32),
            //     sdl2::rect::Point::new(rx as i32, ry as i32));
            // check vertical lines
            ry = 0.0;
            rx = 0.0;
            yo = 0.0;
            xo = 0.0;
            mx = 0;
            my = 0;
            mp = 0;
            r = 0;
            let mut distV: f32 = 1000000.0;
            let mut vx = px;
            let mut vy = py;
            let mut nTan: f32 = -(ra.tan());
            if ra > _PI2 && ra < _3PI2 { // looking left
                rx = (px * _MAP_S as f32) / _MAP_S as f32 - 0.0001;
                ry = (px-rx) * nTan + py;
                xo = -64.0;
                yo = -xo*nTan;
            }
            if ra < _PI2 || ra > _3PI2 { // looking right
                rx = (px * _MAP_S as f32) / _MAP_S as f32 + 64.0;
                ry = (px-rx) * nTan + py;
                xo = 64.0;
                yo = -xo*nTan;
            }
            if ra == 0.0 || ra == _PI {
                rx = px;
                ry = py;
                dof = 8;
            }
            loop {
                mx = (rx/_MAP_S as f32) as i32;
                my = (ry/_MAP_S as f32) as i32;
                mp = my * _MAP_X + mx;

                if mp > 0 && mp < _MAP_X*_MAP_Y && _MAP[mp as usize] > 0 {
                    vx = rx;
                    vy = ry;
                    distV = dist(px, py, vx, vy, ra);
                    dof = 8; // hit wall
                } else {
                    rx += xo;
                    ry += yo;
                    dof += 1;
                }
                if dof >= 8 {
                    break;
                }
            }
            if distV < distH {
                rx = vx;
                ry = vy;
                disT = distV;
            } else if distV > distH {
                rx = hx;
                ry = hy;
                disT = distH;
            }
            _canvas.draw_line(sdl2::rect::Point::new(px as i32, py as i32),
                sdl2::rect::Point::new(rx as i32, ry as i32));
            // Draw 3d Walls
            let mut ca = pa-ra;
            if ca < 0.0 {
                ca += _2PI;
                _canvas.set_draw_color(Color::RGB(0, 200, 0));
            }
            if ca > (_2PI) {
                ca -= _2PI;
                _canvas.set_draw_color(Color::RGB(0, 100, 0));
            }
            //disT = disT*ca.cos();
            let mut lineH = (_MAP_S*_SCREEN_H) as f32/disT;
            if lineH > _SCREEN_H as f32 {
                lineH = _SCREEN_H as f32;
            }
            _canvas.draw_line(sdl2::rect::Point::new((z*_MAP_X+_SCREEN_H) as i32, (0) as i32),
                sdl2::rect::Point::new((z*_MAP_X+_SCREEN_H) as i32, lineH as i32)); 
            // --Draw 3d Walls
            if z >= 60 {break;}
            else {
                ra += _DR;
                z += 1;
            }
        }
        // --Draw 3d Rays
        _canvas.present();
    }
    return true;
}

fn dist(ax: f32, ay: f32, bx: f32, by: f32, ang: f32) -> f32 {
    let mut dist = 0.0;
    dist = (bx-ax)*(bx-ax) + (by-ay)*(by-ay);
    dist = dist.sqrt(); 
    return dist;
}
