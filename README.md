# Kantera
Video composition and rendering kit for Rust, depends on FFmpeg and cairo.

Kantera supports you create simple videos from few Rust code (or Web-UI).

This is my challenging project.💪😬

Process for make video from Rust code:
1. Prepare assets (images, videos) on memory if you need.
1. Build renderer with `render` trait.
1. Render it use `render_to_mp4` function.

![kantera-logo](https://github.com/carrotflakes/kantera/raw/master/out.jpg)

TODO:

- [x] Import videos
- [x] Import images
- [x] Text rendering with any font
- [x] Output video with audio
- [ ] Web-UI as GUI
- [ ] Sound composition
- [ ] DSL for building renderer
- [ ] Multi-thread rendering
- [ ] Realtime hosting via WebRTC
- [ ] Documentation...

## Concepts
- Orthogonal APIs

## Requirement

You need install them:

- FFmpeg
- cairo

## Usage
### Example of video generating

``` sh
$ cargo run --release --example demo
```

After a while, a video named `out.mp4` will be output to current directory.

### Web UI

``` sh
$ cd kantera-web-ui/front
$ yarn install
$ yarn build
$ cd ..
$ cargo run --release
```

Then open `localhost:8080` in your web browser.

e.g. https://twitter.com/carrotflakes/status/1213135191125872642

### Web UI (heroku)

[![Deploy](https://www.herokucdn.com/deploy/button.png)](https://heroku.com/deploy)

## Author

* carrotflakes (carrotflakes@gmail.com)

## Copyright

Copyright (c) 2020 carrotflakes (carrotflakes@gmail.com)

## License

Licensed under the MIT License.
