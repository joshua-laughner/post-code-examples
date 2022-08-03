#ifndef GUARD_PHYSLIB
#define GUARD_PHYSLIB

#include <iostream>
#include <math.h>

struct Vec3d
{
    double x;
    double y;
    double z;

    Vec3d(double x, double y, double z) : x(x), y(y), z(z) {}

    double sqrMagnitude() const {
        return x*x + y*y + z*z;
    }

    double magnitude() const {
        double sqmag = sqrMagnitude();
        return sqrt(sqmag);
    }

    Vec3d unit() const {
        double len = magnitude();
        return Vec3d(x / len, y / len, z / len);
    }

    Vec3d operator+(Vec3d other) const {
        return Vec3d(x + other.x, y + other.y, z + other.z);
    }

    void operator+=(Vec3d other) {
        x += other.x;
        y += other.y;
        z += other.z;
    }

    Vec3d operator-() const {
        return Vec3d(-x, -y, -z);
    }

    Vec3d operator-(Vec3d other) const {
        return Vec3d(x - other.x, y - other.y, z - other.z);
    }

    void operator-=(Vec3d other) {
        x -= other.x;
        y -= other.y;
        z -= other.z;
    }


    Vec3d operator*(double other) const {
        return Vec3d(x * other, y * other, z * other);
    }

    Vec3d operator/(double other) const {
        return Vec3d(x / other, y / other, z / other);
    }
};

std::ostream& operator<<(std::ostream &out, const Vec3d &v) {
    out << "Vec3d(" << v.x << ", " << v.y << ", " << v.z << ")";
    return out;
}

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

#endif
