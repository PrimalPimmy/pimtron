#!/usr/bin/env python3
# Builds src/shaders/earth_mask.bin from Natural Earth's 110m land polygons.
# Run from the repo root: python3 scripts/gen_earth_mask.py
import json
import urllib.request

URL = "https://raw.githubusercontent.com/nvkelso/natural-earth-vector/master/geojson/ne_110m_land.geojson"
W, H = 256, 128
SS = 4  # supersampling per axis
OUT = "src/shaders/earth_mask.bin"

data = json.load(urllib.request.urlopen(URL))
edges = []
for feature in data["features"]:
    geom = feature["geometry"]
    polys = geom["coordinates"] if geom["type"] == "MultiPolygon" else [geom["coordinates"]]
    for poly in polys:
        for ring in poly:
            for (x0, y0), (x1, y1) in zip(ring, ring[1:] + ring[:1]):
                edges.append((x0, y0, x1, y1))

sw, sh = W * SS, H * SS
cover = [[0] * W for _ in range(H)]
for sy in range(sh):
    lat = 90.0 - (sy + 0.5) * 180.0 / sh
    xs = []
    for x0, y0, x1, y1 in edges:
        if (y0 > lat) != (y1 > lat):
            xs.append(x0 + (lat - y0) * (x1 - x0) / (y1 - y0))
    xs.sort()
    row = cover[sy // SS]
    for a, b in zip(xs[0::2], xs[1::2]):
        start = max(0, int((a + 180.0) * sw / 360.0 + 0.5))
        end = min(sw, int((b + 180.0) * sw / 360.0 + 0.5))
        for sx in range(start, end):
            row[sx // SS] += 1

with open(OUT, "wb") as f:
    f.write(bytes(min(255, c * 255 // (SS * SS)) for row in cover for c in row))
print(f"wrote {OUT} ({W}x{H})")
