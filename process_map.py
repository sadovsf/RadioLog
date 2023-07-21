#/bin/python3
# How to use: python3 process_map.py <path to your map data.txt>
# You can get map data from http://www.gnuplotting.org/plotting-the-world-revisited

import sys;

out = open("./src/ui/world_map/world.rs", "w")
out.write("/// [Source data](http://www.gnuplotting.org/plotting-the-world-revisited)\n\n")

count = 0

with open(sys.argv[1]) as f:
    lines = f.readlines()

    accum = ""
    for line in lines:
        parts = line.replace("\n", "").split(" ")
        if len(parts) >= 2:
            accum += "\t(" + parts[0] + ", " + parts[1] + "),\n"
            count = count + 1

out.write("pub static WORLD_HIGH_RESOLUTION: [(f64, f64); {}] = [\n".format(count))
out.write(accum)
out.write("];\n")