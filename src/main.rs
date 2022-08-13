extern crate sdl2;
extern crate gl;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, BlendMode};
use sdl2::rect::Rect;

const _SCREEN_W: i32 = 1200;
const _SCREEN_H: i32 = 600;

const _CELL_W: i32 = 90;
const _CELL_H: i32 = 90;

const _PI: f32 = 3.14159;
const _2PI: f32 = 2.0*_PI;
const _PI2: f32 = _PI/2.0;
const _3PI2: f32 = 3.0*_PI/2.0;
const _DR: f32 = 0.01745329; // one degree in radians

fn main() {
    raycaster();
}

fn raycaster()
{
    let mut _MAP_X: i32 = 8;
    let mut _MAP_Y: i32 = 8;
    let mut _MAP_S: i32 = _MAP_X * _MAP_Y;
    let mut bin_depth = 1; //Quadtree
    let mut map_file = File::open("resources\\map.txt").unwrap();
    let mut map_data = String::new();
    map_file.read_to_string(&mut map_data);
    let map = map_data.split(",");
    let mut _MAP: Vec<i32> = vec![];
    map.for_each(|x|_MAP.push(x.parse().unwrap()));
    _MAP_S = _MAP.len() as i32;
    //println!("map file: {:?}", map);
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
                Event::KeyDown { keycode : Some(Keycode::E), ..} => {
                    // QUADTREE
                    bin_depth += 2;
                    _MAP_S = _MAP_S / bin_depth;
                },
                Event::KeyDown { keycode : Some(Keycode::Q), ..} => {
                    // QUADTREE
                    bin_depth -= 2;
                    if bin_depth < 0 {bin_depth = 1;}
                    _MAP_S = _MAP_S * 2;
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
        let mut q = 1;
        // loop {// se ejecuta bin_depth veces
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
                        let xo = j * _MAP_S * q;
                        let yo = i * _MAP_S * q;
                        _canvas.fill_rect(Rect::new(
                            (xo + j + q) as i32, 
                            (yo + i + q) as i32,
                            (_MAP_S) as u32, 
                            (_MAP_S) as u32));
                        if j >= _MAP_X-1 { break; } // brekeamos el momento
                        j+=1;
                    }
                    if i >= _MAP_Y-1 { break; }
                    i+=1;
                }
            // q += 1;
            // if q >= bin_depth {break;}
        // }
        // --DRAW MAP
        //Draw Player
        _canvas.set_draw_color(Color::RGB(255, 0, 0));
        _canvas.fill_rect(Rect::new(px as i32, py as i32, 8, 8));
        _canvas.draw_line(sdl2::rect::Point::new((px+4.0) as i32, (py+4.0) as i32),
            sdl2::rect::Point::new((px + pdx + 5.0) as i32, (py + pdy + 5.0) as i32));
        _canvas.set_draw_color(Color::RGB(255, 200, 0));
        //--Draw Player
        // Draw 3d Rays
        // check Horizontal lines
        let mut ra:f32 = pa - 30.0*_DR;
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
            disT = disT*ca.cos();
            let mut lineH = (_MAP_S*_SCREEN_H) as f32/disT;
            if lineH > _SCREEN_H as f32 {
                lineH = _SCREEN_H as f32;
            } 
            _canvas.fill_rect(Rect::new((z*_MAP_X+_SCREEN_H) as i32, 0 as i32, 8, lineH as u32));
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
}

fn dist(ax: f32, ay: f32, bx: f32, by: f32, ang: f32) -> f32 {
    let mut dist = 0.0;
    dist = (bx-ax)*(bx-ax) + (by-ay)*(by-ay);
    dist = dist.sqrt(); 
    return dist;
}
