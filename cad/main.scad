
$fn = 64;
EPS = 1e-4;

size = [264, 98];

// A square with rounded corners.
// "size" defines the position of each corner's
// center of radius, not the final size of the shape.
module rounded_square(size, radius, center) {
    r = max(radius, EPS);
    minkowski() {
        square(size, center);
        circle(r);
    }
}

// Import "data/plate.stl".
// The new size is obtained by padding/cropping in
// the X and Y axes, and by scaling in the Z axis.
module plate(size, radius=3) {

    // IMPORTANT: UPDATE THIS PER STL FILE
    // OPENSCAD CANNOT GET SIZE FROM FILE
    stl_size = [271, 104.75, 1.5];

    intersection() {
        translate([0, 0, -1]) linear_extrude(size[2] + 2)
        rounded_square([size[0], size[1]], radius, center=true);
        union() {
            translate([-stl_size[0]/2, -stl_size[1]/2, 0])
            scale([1, 1, size[2]/stl_size[2]]) import("data/plate.stl");
            linear_extrude(size[2]) difference() {
                square([size[0] + 2*radius + 1, size[1] + 2*radius + 1], center=true);
                square([stl_size[0] - 1, stl_size[1] - 1], center=true);
            }
        }
    }
}

// A cherry MX switch.
module switch() {
    translate([-7.8, -7.8, 0]) // center
    translate([.45, .45, 2.5]) rotate([90, 0, 0])
    color([.5, 1, .5]) import("data/switch.stl");
}

// A cherry OEM keycap.
module keycap(pressed=false) {
    color([.9, .9, .9, .5])
    if (pressed) {
        translate([0, 0, 4]) import("data/keycap.stl");
    } else {
        translate([0, 0, 8]) import("data/keycap.stl");
    }
}

// The blackpill development board.
module blackpill() {
    color([.5, .5, .5]) import("data/blackpill.stl");
}

// A profile sufficient to pass the blackpill through.
module blackpill_hole() {
    square([2, 22], center=true);
    translate([-2.25, 0]) hull() {
        translate([0,-3, 0]) circle(2);
        translate([0, 3, 0]) circle(2);
    }
}

// The walls of the case.
// "size" defines the position of each top corner's
// center of radius, not the final size of the shape.
module shell(size, height=5, depth=15, angle=4) {
    difference() {
        hull() {
            translate([0, 0, height])
            linear_extrude(EPS) rounded_square(size, 5, center=true);
            translate([0, 0, -depth]) rotate([-angle, 0, 0])
            linear_extrude(EPS) rounded_square([size[0] - 2, size[1]], 8, center=true);
        }
        hull() {
            translate([0, 0, 1 + height])
            linear_extrude(EPS) rounded_square(size, 2, center=true);
            translate([0, 0, 3 - depth]) rotate([-angle, 0, 0])
            linear_extrude(EPS) rounded_square(size, 2, center=true);
        }
    }
}

// A volume used to separate the shell into top and bottom parts.
module top_region(size, radius) {
    linear_extrude(100) rounded_square(size, radius, center=true);
    translate([0, 0, 2]) linear_extrude(100)
    square([size[0] + 100, size[1] + 100], center=true);
}

// The lower half of the keyboard.
module bottom(size) {

    // shell
    difference() {
        shell(size);
        top_region(size, 4);
        translate([-140, 38, -13]) rotate([-4, 0, 0]) rotate([0, 90, 0])
        linear_extrude(10) blackpill_hole();
    }

    // blackpill mount
    difference() {
        translate([-85, 38, -13]) rotate([-4, 0, 0]) cube([4, 27, 6], center=true);
        translate([-87, 38, -15]) rotate([-4, 0, 0]) cube([4, 22, 6], center=true);
    }

    // plate supports
    translate([0, 7.1 - 19, -12])
    hull() {
        translate([-8, 0, 0]) cylinder(12, 2, 1);
        translate([ 8, 0, 0]) cylinder(12, 2, 1);
    }
    translate([0, 7.1 + 19, -14])
    hull() {
        translate([-8, 0, 0]) cylinder(14, 2, 1);
        translate([ 8, 0, 0]) cylinder(14, 2, 1);
    }
    translate([-70, 7.1, -13])
    hull() {
        translate([-8, 0, 0]) cylinder(13, 2, 1);
        translate([ 8, 0, 0]) cylinder(13, 2, 1);
    }
    translate([-70, 7.1 - 2*19, -10])
    hull() {
        translate([-8, 0, 0]) cylinder(10, 2, 1);
        translate([ 8, 0, 0]) cylinder(10, 2, 1);
    }
    translate([ 70, 7.1, -13])
    hull() {
        translate([-8, 0, 0]) cylinder(13, 2, 1);
        translate([ 8, 0, 0]) cylinder(13, 2, 1);
    }
    translate([ 70, 7.1 - 2*19, -10])
    hull() {
        translate([-8, 0, 0]) cylinder(10, 2, 1);
        translate([ 8, 0, 0]) cylinder(10, 2, 1);
    }
    translate([0, 7.1 - 2*19, -10])
    hull() {
        translate([-8, 0, 0]) cylinder(10, 2, 1);
        translate([ 8, 0, 0]) cylinder(10, 2, 1);
    }
}

// The upper half of the keyboard.
module top(size) {

    // shell
    intersection() {
        shell(size);
        top_region(size, 3.75);
    }

    // plate
    plate([size[0], size[1], 1.6]);

    // strengthening structures
    linear_extrude(3) for (i = [-2:0])
    translate([0, i * 19 + 7.1]) square([size[0] + 6, 2], center=true);
    linear_extrude(5)
    translate([0, 28.75]) square([size[0] + 6, 3], center=true);
}

// translate([-123.50, 40.4, 0]) { switch(); keycap(true); }
// translate([-121.15, -40.4, 0]) { switch(); keycap(); }

// translate([-112, 38, -14]) rotate([-4, 0, 0]) blackpill();
