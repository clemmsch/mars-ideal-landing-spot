# The (Somewhat) Ideal Landing Spot on Mars (If we ignore scientists)

(Disclaimer: This is a repo for a school project that got out of scope)

## How this works

1. The Program first reads all of the images in img (ignoring subdirs)
2. The Program scans all of the targets pixels.
3. The Program uses euclidian distance to get the nearest of the 11 'major' colors defined by me (Yellow, Blue, Red, Violet, ...) of each pixel.
4. The Program uses an iterative approach to loop through all of the image's Colors (Vec<Color>) to get all pixels at which the major color is the same
5. The Program draws spots on a template representing these spots (x = pos % IMAGE_WIDTH, y = (pos - x) / IMAGE_WIDTH)

## Requirements of an image
The image must be 520x416 pixels tall (That is a size that fits the ratio of the NASA maps very well (orig - 200 on both sides))

### Why so small?
It is, in my opinion, not neccessary for these images to be that exact.

## Sources
All Sources for images and other important things are linked in `/cites`