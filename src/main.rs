// TODO: add collision between particles, we should probably create a world class or something, then only the particles added onto it can interact, we could add a gravity behaviour like world.add_particle(radius, color, pos).gravity = 10;, maybe the world class would contain a list with all of its objects? Also probably add a variable like collideable in the particle's constructor for increased control

mod physics;
mod utils;

use physics::{Constraint, Particle, World};
use piston::{UpdateEvent};
use piston_window::*;
use utils::distance;

fn main() {
    let width: u32 = 600;
    let height: u32 = 600;

    let mut world: World = World::new(width, height);
    world.set_caption("I am sleep'nt :)");

    let gravity = 100.;
    world.set_gravity(gravity);

    let number_to_iterate = 5;

    for y in 0..number_to_iterate{
        for x in 0..number_to_iterate{
            let mut point = Particle::new([0.2, 0.2, 0.2, 1.], [((x+1)*50) as f64, (y+1) as f64*50.], [((x+1)*50) as f64, (y+1) as f64*50.], 5.);
            if y == 0{
                point.color = [0., 0., 1., 1.];
            }
            world.add_particle(&point);
        }
    }
    
    for i in 0..number_to_iterate*number_to_iterate{
        if (i+1) % number_to_iterate != 0{
            let mut c1 = Constraint::new(i, i+1, 0.5, [0., 0., 0., 1.], 1.);
            c1.set_length(distance(world.get_particles()[i].position, world.get_particles()[i+1].position));
            world.add_constraint(&c1);
        }

        if i <= number_to_iterate*number_to_iterate - number_to_iterate - 1{
            let mut c2 = Constraint::new(i, i+number_to_iterate, 0.5, [0., 0., 0., 1.], 1.);
            c2.set_length(distance(world.get_particles()[i].position, world.get_particles()[i+number_to_iterate].position));
            world.add_constraint(&c2);
        }
    }

    while let Some(e) = world.next() {
        let e2 = e.clone();
        println!("{}", world.dt());
        world.iterate();
        world.draw_objects(e.clone());

        if let Some(button) = e2.press_args() {
            if let Button::Mouse(button) = button {
                let e3 = e2.clone();
                if distance(world.get_mouse_position(e2), world.get_particles()[23].position) <= world.get_particles()[23].radius && button == MouseButton::Left{
                    world.get_particles()[23].position = world.get_mouse_position(e3);
                }
            }
        }
    }
}
