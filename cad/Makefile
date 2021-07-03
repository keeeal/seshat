all: top bottom

top:
	cat main.scad > top.scad
	echo "top(size);" >> top.scad
	openscad top.scad -o top.stl

bottom:
	cat main.scad > bottom.scad
	echo "bottom(size);" >> bottom.scad
	openscad bottom.scad -o bottom.stl

clean:
	rm top.scad top.stl bottom.scad bottom.stl