# Voronoi
A toy program to visualize Voronoi diagrams

# Controls

click in the window -> Puts a point in the clicked place

s -> Saves the current image to a png file

r -> Erases all the points in the image

d -> Removes/adds the dots that mark where the points are

left/right arrow -> Changes the distance algorithm in the order of Euclidean/Manhattan/Chebyshev

# Running

With Rust nightly, it should be as easy as:

```bash
$ cargo run
```

Although I would probably recommend adding the --release flag because the diagram is calculated by calculating each individual point

```bash
$ cargo run --release
```
