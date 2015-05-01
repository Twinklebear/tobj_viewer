tobj viewer
===
A simple Wavefront OBJ viewer that uses [tobj](https://github.com/Twinklebear/tobj) to load models and
[glium](https://github.com/tomaka/glium) to render them. The rendering code is essentially straight out of
the glium teapot demo but will also re-scale models to fit within a unit cube so it's easier to view a wide
variety of models at potentially very different scales.

Samples
---
The rendering quality is extremely basic, this program is mostly used to check that tobj is loading things properly
on some bigger scenes.

![Stanford Buddha](http://i.imgur.com/eUsqZd8.png)

![Rust Logo](http://i.imgur.com/uJbca2d.png)

The Buddha is from the [Stanford Scanning Repository](http://graphics.stanford.edu/data/3Dscanrep/) and the Rust logo
was modeled [Nylithius on BlenderArtists](http://blenderartists.org/forum/showthread.php?362836-Rust-language-3D-logo).

