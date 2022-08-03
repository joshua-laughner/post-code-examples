mod physlib;
use physlib::{Vec3, Angles3d};

const G: f64 = 6.67304e-11;

trait PhysicsObject {
    fn update(&mut self, dt: f64, all_objects: &[impl PhysicsObject]) {
        let force = self.compute_force_vector(dt, all_objects);
        let pos = self.get_position() + dt * self.get_velocity();
        let vel = self.get_velocity() + dt * force / self.get_mass();
        self.set_position(pos);
        self.set_velocity(vel);
    }

    fn print_status(&self) {
        println!("Object is at {:.2} with velocity {:.2}", self.get_position(), self.get_velocity())
    }

    fn compute_gravitational_force(me: &impl PhysicsObject, all_objects: &[impl PhysicsObject]) -> Vec3 {
        let mut force = Vec3::new_zero();
        for obj in all_objects.iter() {
            let r2 = (me.get_position() - obj.get_position()).sqr_magnitude();
            if r2 > 1e-10 {
                let dir = (me.get_position() - obj.get_position()).unit();
                force += dir * G * me.get_mass() * obj.get_mass() / r2;
            }
        }
        return force;
    }

    fn get_position(&self) -> Vec3;
    fn set_position(&mut self, pos: Vec3);
    fn get_velocity(&self) -> Vec3;
    fn set_velocity(&mut self, vel: Vec3);
    fn get_mass(&self) -> f64;
    fn compute_force_vector(&mut self, dt: f64, all_objects: &[impl PhysicsObject]) -> Vec3;
}

struct PassiveObject {
    position: Vec3,
    velocity: Vec3,
    mass: f64
}

impl PhysicsObject for PassiveObject {
    fn get_position(&self) -> Vec3 { self.position }
    fn set_position(&mut self, pos: Vec3) { self.position = pos }
    fn get_velocity(&self) -> Vec3 { self.velocity }
    fn set_velocity(&mut self, vel: Vec3) { self.velocity = vel }
    fn get_mass(&self) -> f64 { self.mass }

    fn compute_force_vector(&mut self, _: f64, all_objects: &[impl PhysicsObject]) -> Vec3 {
        return Self::compute_gravitational_force(self, all_objects);
    }
}

struct Rocket {
    position: Vec3,
    rotation: Angles3d,
    velocity: Vec3,
    structural_mass: f64,
    fuel_mass: f64,
    thrust: f64
}

impl PhysicsObject for Rocket {
    fn get_position(&self) -> Vec3 { self.position }
    fn set_position(&mut self, pos: Vec3) { self.position = pos }
    fn get_velocity(&self) -> Vec3 { self.velocity }
    fn set_velocity(&mut self, vel: Vec3) { self.velocity = vel }
    fn get_mass(&self) -> f64 { self.structural_mass + self.fuel_mass }

    fn compute_force_vector(&mut self, dt: f64, all_objects: &[impl PhysicsObject]) -> Vec3 {
        let mut force = Self::compute_gravitational_force(self, all_objects);
        let mut fuel_consumed = self.thrust * dt;
        if fuel_consumed > self.fuel_mass {
            fuel_consumed = self.fuel_mass;
        }
        if fuel_consumed < 0.0 {
            fuel_consumed = 0.0;
            self.fuel_mass = 0.0;
        }
        self.fuel_mass -= fuel_consumed;
        force += self.rotation.to_unit_vector() * fuel_consumed * 100.0;
        return force;
    }

    fn print_status(&self) {
        println!("Object is at {:.2} with velocity {:.2} and remaining fuel {:.2}", self.get_position(), self.get_velocity(), self.fuel_mass)
    }
}


fn main() {
    let mut rocket = Rocket{
        position: Vec3::new_zero(),
        rotation: Angles3d::new_zero(),
        velocity: Vec3::new_zero(),
        structural_mass: 100.0,
        fuel_mass: 10.0,
        thrust: 1.0
    };

    let objects: Vec<PassiveObject> = vec![];

    let dt = 0.1;
    let mut t = 0.0;
    let mut print_timer = 99.0;
    while t < 20.0 {
        if print_timer >= 1.0 {
            print!("[Time {t:.2}] ");
            rocket.print_status();
            print_timer = 0.0;
        }

        rocket.update(dt, &objects);
        t += dt;
        print_timer += dt;
    }
}
