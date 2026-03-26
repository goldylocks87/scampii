#!/usr/bin/env python3
"""
Generate a PNG emoji from scampii's Frame 0 pixel data.
Outputs: scampii.png (32x26 native) and scampii_128.png (128x128 scaled, cropped/centered).
"""

from PIL import Image

# Palette: packed value -> RGBA
# 0 (NN) = transparent, 1-16 = hue variants
PALETTE = {
    0:  (0, 0, 0, 0),           # NN - transparent
    1:  (8, 7, 7, 255),         # S0
    2:  (26, 19, 17, 255),      # S1
    3:  (97, 39, 33, 255),      # S2
    4:  (176, 91, 44, 255),     # S3
    5:  (185, 69, 29, 255),     # S4
    6:  (211, 151, 65, 255),    # S5
    7:  (232, 138, 54, 255),    # S6
    8:  (241, 100, 31, 255),    # S7
    9:  (252, 165, 112, 255),   # S8
    10: (255, 240, 137, 255),   # S9
    11: (86, 79, 91, 255),      # EyePupil
    12: (210, 210, 210, 255),   # EyeIris
    13: (255, 255, 255, 255),   # EyeWhite
    14: (26, 19, 17, 255),      # Leg
    15: (20, 16, 16, 255),      # Antenna
    16: (14, 12, 12, 255),      # EyeRing
}

# Aliases matching the Rust code
NN = 0
_0, _1, _2, _3, _4, _5, _6, _7, _8, _9 = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
EP, EI, EW = 11, 12, 13
_L, _A, _E = 14, 15, 16

# Frame 0 — antennae sweep left (32x26)
FRAME_0 = [
    [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,NN,NN,NN,NN,_A,_A,_A,_A,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,NN,NN,NN,_A,NN,NN,NN,NN,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_A,_A,_A,NN,NN,NN,NN,NN,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_E,_E,_E,NN,_A,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,NN,NN,NN,_2,_2,_2,_2,_2,_2,_2,_2,_E,_E,EI,EW,_E,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,NN,NN,_2,_4,_8,_8,_6,_7,_4,_4,_4,_E,_E,EP,EI,_E,_2,_2,_2,_2,_2,_2,_2,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,NN,_2,_4,_8,_9,_9,_8,_8,_6,_6,_4,_E,_E,_E,_E,_E,_7,_4,_4,_2,_1,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,_2,_4,_6,_9,_8,_6,_6,_7,_7,_7,_7,_4,_E,_E,_E,_7,_3,_2,_1,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,_2,_4,_2,_8,_6,_6,_7,_7,_7,_7,_7,_7,_7,_7,_7,_3,_2,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,_2,_4,_5,_2,_6,_7,_7,_4,_7,_4,_7,_4,_7,_4,_3,_2,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,_2,_6,_9,_6,_2,_0,_3,_2,_3,_2,_3,_2,_3,_2,_0,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,_2,_7,_8,_6,_7,_4,_0,_L,_0,_0,_L,_0,_L,_L,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,_2,_6,_7,_4,_2,_L,NN,_L,NN,_L,NN,NN,_L,NN,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,_2,_5,_2,_2,_2,_1,NN,NN,_L,NN,NN,_L,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,_2,_7,_9,_6,_4,_4,_1,NN,NN,NN,NN,_L,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,_2,_4,_8,_6,_4,_1,NN,NN,NN,NN,NN,NN,_L,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,NN,_2,_4,_4,_2,_4,_1,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,NN,NN,_2,_2,_4,_7,_4,_4,_1,_1,_1,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_2,_2,_4,_2,_1,_2,_3,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_2,_1,_2,_1,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_1,_2,_1,_3,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_1,_3,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,_1,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
    [NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN,NN],
]

W, H = 32, 26

# --- Native resolution PNG ---
img = Image.new("RGBA", (W, H), (0, 0, 0, 0))
for y, row in enumerate(FRAME_0):
    for x, px in enumerate(row):
        img.putpixel((x, y), PALETTE[px])

img.save("/Users/alec.ellebracht/Desktop/scampii/scampii.png")
print("scampii.png (32x26)")

# --- Tight crop to content bounds ---
# Find bounding box of non-transparent pixels
min_x, min_y, max_x, max_y = W, H, 0, 0
for y, row in enumerate(FRAME_0):
    for x, px in enumerate(row):
        if px != 0:
            min_x = min(min_x, x)
            min_y = min(min_y, y)
            max_x = max(max_x, x)
            max_y = max(max_y, y)

cropped = img.crop((min_x, min_y, max_x + 1, max_y + 1))
cw, ch = cropped.size
print(f"  Content bounds: ({min_x},{min_y}) to ({max_x},{max_y}) = {cw}x{ch}")

# --- 128x128 emoji (centered, pixel-perfect upscale) ---
# Scale to fit 128x128 while maintaining pixel grid
scale = min(128 // cw, 128 // ch)
scaled = cropped.resize((cw * scale, ch * scale), Image.NEAREST)

# Center on 128x128 canvas
emoji = Image.new("RGBA", (128, 128), (0, 0, 0, 0))
ox = (128 - scaled.width) // 2
oy = (128 - scaled.height) // 2
emoji.paste(scaled, (ox, oy))
emoji.save("/Users/alec.ellebracht/Desktop/scampii/scampii_128.png")
print(f"scampii_128.png (128x128, {scale}x upscale)")

# --- Also generate all 3 standard emoji sizes ---
for size in [64, 256, 512]:
    scale_s = min(size // cw, size // ch)
    if scale_s < 1:
        scale_s = 1
    scaled_s = cropped.resize((cw * scale_s, ch * scale_s), Image.NEAREST)
    out = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    out.paste(scaled_s, ((size - scaled_s.width) // 2, (size - scaled_s.height) // 2))
    out.save(f"/Users/alec.ellebracht/Desktop/scampii/scampii_{size}.png")
    print(f"scampii_{size}.png ({size}x{size}, {scale_s}x upscale)")

print("\nDone!")
