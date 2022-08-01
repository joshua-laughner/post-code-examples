// Compile on Linux with: g++ -o physics_model physics_model.cpp
#include <iostream>
#include <math.h>
#include <vector>
#include "physlib.h"

// This is bad practice, only done here for brevity.
using namespace std;

const double G = 6.67408e-11; // m3 kg-1 s-2

struct Angles3d
{
    double theta;
    double phi;

    Angles3d(double theta, double phi) : theta(theta), phi(phi) {}

    Vec3d toUnitVector() const {
        double theta_rad = theta * M_PI / 180.0;
        double phi_rad = phi * M_PI / 180.0;
        double x = cos(theta_rad) * sin(phi_rad);
        double y = sin(theta_rad) * sin(phi_rad);
        double z = cos(phi_rad);
        return Vec3d(x, y, z);
    }
};

class PhysicsObject {
    public:

    double mass;
    Vec3d position;
    Vec3d velocity;

    PhysicsObject(double mass) : mass(mass), position(Vec3d(0.0, 0.0, 0.0)), velocity(Vec3d(0.0, 0.0, 0.0)) {}

    void print_status() const {
        cout << "Object is at " << position << " with velocity " << velocity << "\n";
    }

    virtual void update(double dt, vector<PhysicsObject>& all_objects) {
        Vec3d force = compute_force_vector(dt, all_objects);
        position += velocity * dt;
        velocity += force * dt / mass;
    }

    virtual Vec3d compute_force_vector(double dt, vector<PhysicsObject>& all_objects) {
        Vec3d force = Vec3d(0.0, 0.0, 0.0);
        for(auto obj : all_objects) {
            double r2 = (obj.position - position).sqrMagnitude();
            if (r2 > 1e-10) {
                Vec3d dir = (obj.position - position).unit();
                force += dir * G * obj.mass * mass / r2;
            }
        }
        return force;
    }
};


class Rocket : public PhysicsObject {
    public:

    double structural_mass;
    double fuel_mass;
    Angles3d rotation;
    double thrust; // units of fuel mass consumed per second

    Rocket(double structural_mass, double fuel_mass, double thrust) : 
        structural_mass(structural_mass),
        fuel_mass(fuel_mass),
        rotation(Angles3d(0.0, 0.0)),
        thrust(thrust),
        PhysicsObject(structural_mass + fuel_mass)
        {

        }

    void print_status() const {
        cout << "Rocket is at " << position << " with velocity " << velocity << " and remaining fuel " << fuel_mass << "\n";
    }

    Vec3d compute_force_vector(double dt, vector<PhysicsObject>& all_objects) override {
        Vec3d force = PhysicsObject::compute_force_vector(dt, all_objects);
        double fuel_consumed = thrust * dt;
        if (fuel_consumed > fuel_mass) {
            fuel_consumed = fuel_mass;
        }
        if (fuel_consumed < 0){
            fuel_consumed = 0.0;
            fuel_mass = 0.0;
        }
        fuel_mass -= fuel_consumed;
        mass -= fuel_consumed;
        force += rotation.toUnitVector() * fuel_consumed * 100;
        return force;
    }
};


int main() {
    Rocket rocket = Rocket(100.0, 10.0, 1.0);
    vector<PhysicsObject> all_objects = {rocket};
    double dt = 0.1;
    double t = 0.0;
    double print_timer = 99.0;
    while (t < 20.0) {
        if (print_timer >= 1.0) {
            cout << "[Time = " << t << "] ";
            rocket.print_status();
            print_timer = 0.0;
        }

        rocket.update(dt, all_objects);
        t += dt;
        print_timer += dt;
    }
    cout << "[Time = " << t << "] ";
    rocket.print_status();
}