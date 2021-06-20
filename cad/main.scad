
$fn = 64;
EPS = 1e-4;

module load_dxf(file, bounds) {
    w = bounds[2] - bounds[0];
    h = bounds[3] - bounds[1];
    translate([-w/2 - bounds[0], -h/2 - bounds[1]])
    import(file);
}

module rounded_square(size, radius, center) {
    r = max(radius, EPS);
    minkowski() {
        square([size[0], size[1]], center);
        circle(r);
    }
}

module plate(size, radius=0) {

    dxf_bounds = [
        0.0950846,
        0.434082,
        266.795,
        100.446
    ];

    dx = dxf_bounds[2] - dxf_bounds[0];
    dy = dxf_bounds[3] - dxf_bounds[1];

    l = max(size[0], ceil(dx));
    h = max(size[1], ceil(dy));

    linear_extrude(size[2]) {
        load_dxf("data/plate.dxf", dxf_bounds);
        difference() {
            rounded_square([l, h], radius, true);
            square([floor(dx), floor(dy)], true);
        }
    }

    linear_extrude(size[2] + 1)
    for (i = [-2:0]) {
        translate([0, i * 19.05 + 7.1]) square([l, 2], true);
    }

    linear_extrude(5)
    translate([0, 28.75]) square([l, 3], true);
}

module switch() {
    translate([-7.8, -7.8, 0]) // center
    translate([.45, .45, 2.5]) rotate([90, 0, 0])
    color([.5, 1, .5]) import("data/switch.stl");
}

module keycap(pressed=false) {
    if (pressed) {
        translate([0, 0, 4])
        color([.9, .9, .9, .5]) import("data/keycap.stl");
    } else {
        translate([0, 0, 8])
        color([.9, .9, .9, .5]) import("data/keycap.stl");
    }
}

module blackpill() {
    color([.5, .5, .5]) import("data/blackpill.stl");
}

module blackpill_hole() {
    translate([-54/2, 0, 3.25]) rotate([0, 90, 0])
    linear_extrude(54) hull() {
        translate([0,-3, 0]) circle(2);
        translate([0, 3, 0])circle(2);
    }
    translate([0, 0, 1])
    cube([54, 22, 2], true);
}

module case() {
    difference() {
        hull() {
            translate([0, 0, -15])
            rotate([-4, 0, 0])
            linear_extrude(EPS)
            rounded_square([264, 98], 8, true);

            translate([0, 0, 5])
            linear_extrude(EPS)
            rounded_square([266, 98], 5, true);
        }

        hull() {
            translate([0, 0, -12])
            rotate([-4, 0, 0])
            linear_extrude(EPS)
            rounded_square([266, 98], 2, true);

            translate([0, 0, 6])
            linear_extrude(EPS)
            rounded_square([266, 98], 2, true);
        }

        translate([-120, 38, -14])
        rotate([-4, 0, 0]) blackpill_hole();
    }

    difference() {
        translate([-86, 38, -13])
        rotate([-4, 0, 0]) cube([4, 27, 6], true);
        translate([-88, 38, -15])
        rotate([-4, 0, 0]) cube([4, 22, 6], true);
    }
}

module bottom() {
    difference() {
        case();
        union() {
            linear_extrude(100)
            rounded_square([266, 98], 4, true);
            translate([0, 0, 2])
            linear_extrude(100)
            rounded_square([300, 200], 0, true);
        }
    }

    translate([0, 7.1 - 19.05, -12])
    hull() {
        translate([-8, 0, 0]) cylinder(12, 2, 1);
        translate([ 8, 0, 0]) cylinder(12, 2, 1);
    }
    translate([0, 7.1 + 19.05, -14])
    hull() {
        translate([-8, 0, 0]) cylinder(14, 2, 1);
        translate([ 8, 0, 0]) cylinder(14, 2, 1);
    }
    translate([-70, 7.1, -13])
    hull() {
        translate([-8, 0, 0]) cylinder(13, 2, 1);
        translate([ 8, 0, 0]) cylinder(13, 2, 1);
    }
    translate([-70, 7.1 - 2*19.05, -10])
    hull() {
        translate([-8, 0, 0]) cylinder(10, 2, 1);
        translate([ 8, 0, 0]) cylinder(10, 2, 1);
    }
    translate([ 70, 7.1, -13])
    hull() {
        translate([-8, 0, 0]) cylinder(13, 2, 1);
        translate([ 8, 0, 0]) cylinder(13, 2, 1);
    }
    translate([ 70, 7.1 - 2*19.05, -10])
    hull() {
        translate([-8, 0, 0]) cylinder(10, 2, 1);
        translate([ 8, 0, 0]) cylinder(10, 2, 1);
    }
    translate([0, 7.1 - 2*19.05, -10])
    hull() {
        translate([-8, 0, 0]) cylinder(10, 2, 1);
        translate([ 8, 0, 0]) cylinder(10, 2, 1);
    }
}

module top() {
    intersection() {
        case();
        union() {
            linear_extrude(100)
            rounded_square([266, 98], 3.75, true);
            translate([0, 0, 2])
            linear_extrude(100)
            rounded_square([300, 200], 0, true);
        }
    }

    plate([271, 103, 1.5]);
}

// translate([-123.85, 40.5, 0]) { switch(); keycap(true); }
// translate([-121.45, -40.5, 0]) { switch(); keycap(); }

// translate([-113, 38, -14])
// rotate([-4, 0, 0])
// blackpill();

top();
bottom();

