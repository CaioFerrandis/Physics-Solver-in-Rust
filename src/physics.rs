use piston_window::*;
use crate::utils::*;
use std::{time::{Duration, Instant}};

pub struct World{
    particles: Vec<Particle>,
    constraints: Vec<Constraint>,
    pub width: u32,
    pub height: u32,
    window: PistonWindow,
    gravity: f64,
    dt: f64,
    last_mouse_position: [f64; 2],
    last_time: Instant,
}

impl World{
    pub fn new(width: u32, height: u32) -> Self {
        let window: PistonWindow = WindowSettings::new("Window", [width, height])
            .exit_on_esc(true)
            .build()
            .expect("Failed to create Piston window");

        World {
            particles: Vec::new(),
            constraints: Vec::new(),
            width,
            height,
            window,
            gravity: 0.,
            dt: 1.,
            last_mouse_position: [-1., -1.],
            last_time: Instant::now(),
        }
    }

    pub fn set_gravity(&mut self, value: f64){
        self.gravity = value;
    }

    pub fn add_particle(&mut self, particle: &Particle){
        self.particles.push(*particle);
    }

    pub fn add_constraint(&mut self, constraint: &Constraint){
        self.constraints.push(*constraint);
    }

    pub fn draw_objects(&mut self, e: Event){
        self.window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            for particle in &self.particles{
                particle.draw(c.transform, g);
            }
            for constraint in &self.constraints{
                constraint.draw(c.transform, g, &self.particles)
            }
        });
    }

    pub fn next(&mut self) -> Option<Event>{
        return self.window.next();
    }

    pub fn iterate(&mut self){
        self.dt = self.dt();

        for particle in self.particles.iter_mut(){
            particle.update(self.gravity, self.dt);
            particle.bound(self.width, self.height);
        }
        for constraint in self.constraints.iter_mut(){
            constraint.update(&mut self.particles);
        }
    }

    pub fn set_caption(&mut self, caption: &str){
        self.window.set_title(caption.to_string());
    }

    pub fn get_particles(&mut self) -> &mut Vec<Particle>{
        return &mut self.particles;
    }

    pub fn dt(&mut self) -> f64{
        let mut dt = self.last_time.elapsed().as_micros() as f64;
        dt /= f64::powf(10., 6.);
        self.last_time = Instant::now();
        return dt;
    }

    pub fn get_constraints(&mut self) -> &mut Vec<Constraint>{
        return &mut self.constraints;
    }

    pub fn get_mouse_position(&mut self, e: Event) -> [f64; 2]{
        if let Some(pos) = e.mouse_cursor_args(){
            self.last_mouse_position = pos;
            return pos;
        }
        else{
            return self.last_mouse_position;
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Particle{
    pub color: [f32; 4],
    pub old_position: [f64; 2],
    pub position: [f64; 2],
    velocity: [f64; 2],
    pub radius: f64,
    pub update: bool,
}

impl Particle{
    pub fn new(color: [f32; 4], old_position: [f64; 2], position: [f64; 2], radius: f64) -> Self{
        let mut velocity: [f64; 2] = [0., 0.];
        velocity[0] = position[1] - old_position[1];
        velocity[1] = position[1] - old_position[1];
        let update = true;
        Particle { color, old_position, position, velocity, radius, update }
    }

    pub fn draw<G: piston_window::Graphics>(&self, transform: [[f64; 3];2], g: &mut G){
        ellipse(
            self.color,
            [self.position[0] - self.radius, self.position[1] - self.radius, self.radius * 2.0, self.radius * 2.0],
            transform,
            g,
        );
    }

    pub fn bound(&mut self, width: u32, height: u32){
        if self.position[0] < self.radius{
            self.position[0] = self.radius;
            self.old_position[0] = self.position[0] + self.velocity[0];
        }
        else if self.position[0] > width as f64 - self.radius{
            self.position[0] = width as f64 - self.radius;
            self.old_position[0] = self.position[0] + self.velocity[0];
        }
        if self.position[1] < self.radius{
            self.position[1] = self.radius;
            self.old_position[1] = self.position[1] + self.velocity[1];
        }
        else if self.position[1] > height as f64 - self.radius{
            self.position[1] = height as f64 - self.radius;
            self.old_position[1] = self.position[1] + self.velocity[1];
        }
    }

    pub fn update(&mut self, gravity: f64, dt: f64){
        if self.update{
            for i in 0..10{
                self.velocity[0] = self.position[0] - self.old_position[0];
                self.velocity[1] = self.position[1] - self.old_position[1] + gravity;
        
                self.old_position = self.position.clone();
        
                self.position[0] += self.velocity[0]*dt;
                self.position[1] += self.velocity[1]*dt;
            }
        }
    }

    pub fn is_physics(&mut self, enabled: bool){
        self.update = enabled;
    }

    pub fn set_position(&mut self, position: [f64; 2]){
        self.position = position;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Constraint{
    pub point1: usize,
    pub point2: usize,
    start: [f64; 2],
    end: [f64;2],
    length: f64,
    pub strength: f64,
    pub color: [f32; 4],
    pub radius: f64,
}

impl Constraint{
    pub fn new(point1: usize, point2: usize, strength: f64, color: [f32; 4], radius: f64) -> Self{
        let start = [0.; 2];
        let end = [0.; 2];
        let length = distance(start, end);
        Constraint{ point1, point2, start, end, length, strength, color, radius }
    }

    pub fn draw<G: piston_window::Graphics>(&self, transform: [[f64; 3];2], g: &mut G, particles: &Vec<Particle>){
        line(self.color, self.radius, 
            [particles[self.point1].position[0], particles[self.point1].position[1],
            particles[self.point2].position[0], particles[self.point2].position[1]],
             transform, g);
    }

    pub fn update(&mut self, particles: &mut Vec<Particle>){
        self.start = particles[self.point1].position;
        self.end = particles[self.point2].position;

        if distance(particles[self.point1].position, particles[self.point2].position) != self.length{
            let dist: f64 = distance(particles[self.point1].position, particles[self.point2].position);
            let offset = dist - self.length;
            let dx = self.start[0] - self.end[0];
            let dy = self.start[1] - self.end[1];

            let percent = offset / dist / 2. * self.strength;
            
            if particles[self.point1].update{
                particles[self.point1].position[0] -= dx*percent;
                particles[self.point1].position[1] -= dy*percent;
            }
            if particles[self.point2].update{
                particles[self.point2].position[0] += dx*percent;
                particles[self.point2].position[1] += dy*percent;
            }
        }
    }

    pub fn set_radius(&mut self, radius: f64){
        self.radius = radius;
    }

    pub fn get_length(&self) -> f64{
        return self.length;
    }

    pub fn set_length(&mut self, length: f64){
        self.length = length;
    }
}
