# The (Somewhat) Ideal Landing Spot on Mars (If we ignore scientists)

(Disclaimer: This is a repo for a school project that got out of scope)

## How this works

1. The Program first reads all of the images in img (ignoring subdirs)
2. The Program scans all of the targets pixels.
3. The Program uses euclidian distance to get the nearest of the 11 'major' colors defined by me (Yellow, Blue, Red, Violet, ...)
4. The Program uses an iterative approach to get all pixels at which the major color is the same
5. The Program draws spots on a template representing these spots