# Shmup (by Kennedy Guerra)

Shmup is game wherein you destroy hordes of enemies with your spaceplane. It is a desktop game completely free of charge and dedicated to the public domain.

> [!NOTE]
> Shmup is at an early stage of development, barely a prototype.

![Animated gif](https://i.imgur.com/deye7dl.gif)

This project is part of the [Indie Smiths](https://github.com/IndieSmiths) project and has a [dedicated website](https://shmup.indiesmiths.com) where soon you'll be able to find more info about it.

It is made in [Rust](https://github.com/rust-lang/rust), with the Rust bindings to the SDL2 library ([crate](https://crates.io/crates/sdl2) | [GitHub repo](https://github.com/Rust-SDL2/rust-sdl2)), targeting desktop platforms where Rust is available like Windows, Mac and Linux.

This game was created by [Kennedy R. S. Guerra](https://kennedyrichard.com) (me), who also develops/maintains it. The code is public domain with [The Unlicense](https://unlicense.org/) license and the art are [CC0](https://creativecommons.org/publicdomain/zero/1.0/) assets provided for free by [Kenney](https://kenney.nl/assets).


## Support the game

Please, support the development of useful open-source apps and fun open-source games like Shmup in the Indie Smiths project by becoming our patron on [patreon][]. You can also make recurrent donations using [github sponsors][], [liberapay][] or [Ko-fi][].

Both [github sponsors][] and [Ko-fi][] also accept one-time donations.

Any amount is welcome and helps. Check the project's [donation page][] for all donation methods available.

Please, also [support the awesome work Kenney does for game creators](https://kenney.nl/donate).


## Installing & running the game


### If you want to install

Use the `cargo install shmup` command.


### If you want to use as a standalone program

Download the repository and run the `cargo run` command from it.


### Troubleshooting

The extra instructions below may not be necessary. They are just suggestions of things to try **in case** you can't run the shmup game right away.

#### Ensuring you have the sdl2 library

Regardless of whether you install the shmup game or not, you must make sure the `sdl2` library is installed on your machine, as well as other related libraries like `sdl_image`, `sdl_mixer`, `sdl_ttf` and `sdl_gfx`. You must also ensure that Rust can see those libraries on your system, although installing them may already take care of that.

I can't provide precise instructions because they vary from system to system, but a quick google search should point you to good instructions for your specific system.

On my (X)Ubuntu GNU/Linux machine, sdl2 and its related libraries can be installed easily from my command line. In the image below, the items marked with a green square are libs/components installed on my machine:

![libs/components installed on my machine](https://i.imgur.com/4cBVhQz.png)

Note that, as demonstrated in the image above you'll likely need not only the libs themselves, but also their accompanying development files (the versions maked with `-dev`, like `libsdl2-dev`).

This is probably a good place to start your search: https://wiki.libsdl.org/SDL2/Installation


#### Ensuring the sdl2 crate works fine

The shmup game is built on top of the [sdl2 crate](https://crates.io/crates/sdl2), so it is a good idea to ensure it is working properly with a simpler example first.

For instance, if you have problems trying to run the shmup game, first try executing this [simpler example](https://docs.rs/sdl2/latest/sdl2/) provided by the sdl2 crate documentation (this example only ensures base sdl2 is working).

To ensure the related libs (`sdl_image`, `sdl_mixer`, `sdl_ttf` and `sdl_gfx`) are working as well you'll have to try some of the other [available examples](https://github.com/Rust-SDL2/rust-sdl2/tree/master/examples).


## Controls

For now, there are only simple actions:

| Action | Key |
| --- | --- |
| Movement | w, a, s, d keys |
| Shoot | j |
| Quit | Escape |


## Semantic versioning for games

Since [semantic versioning](https://semver.org/spec/v2.0.0.html) doesn't map well to game project versions, this is the meaning I adopt here:

In a **X**.**Y**.**Z** version...

**X** is always either 0 or 1. If it is 0, the game isn't finished, that is, it isn't in the state the creator envisioned for it. If it is 1, then the game is in that state and thus we consider it finished.

Even when the game is finished (X is 1), the game may still get new features, fixes, content, etc., though, in much the same way finished games can still get patches or DLCs. Such additions/changes won't increment X though, only Y and Z, depending on the specific changes.

In semantic versioning, incrementign X indicates breaking changes, but since a game is just an executable, not a library with a public API, there's no reason to think of it like that here, so as we explained, we are only concerned with whether the game is finished or not.

**Y** and **Z** more closely resemble their meaning in semantic versioning. **Y** is incremented whenever new features or content are added to the game or when changes are made. And **Z** is incremented whenever a fix is introduced.


## Contributing

Keep in mind this is a game project, so it has a design and finite set of features defined by its creator (me, Kennedy Guerra) according to his vision. In this sense, it is not much different from a book project, which is usually not a very collaborative project, except for the editor's work.

In other words, as much as we love contributions in general in the Indie Smiths project, for this game project we would like the contributions to be limited to refactoring/optimizing/fixes, rather than changing the design/content of the game.

Additionally, when submitting pull requests (PRs), please, submit them to the `develop` branch, not the `main` branch. This way we can refine/complement the changes before merging them with `main`.

If in doubt, please [start a discussion](https://github.com/IndieSmiths/shmup-rs/discussions) first, in order to discuss what you would like to change.


## Issues

Issues are reserved for things that crash the game or otherwise prevent the user from progressing in the game. Please, if you're not certain, [start a discussion](https://github.com/IndieSmiths/shmup-rs/discussions) instead. It can always be converted into an issue later if needed.

## Contact

Contact me any time via [Bluesky](https://bsky.app/profile/kennedyrichard.com), [Twitter/X](https://twitter.com/KennedyRichard), [mastodon](https://fosstodon.org/KennedyRichard) or [email](mailto:kennedy@kennedyrichard.com).

You are also welcome on the Indie Smiths's [discord server](https://indiesmiths.com/discord).


## License

Shmup is dedicated to the public domain with [The Unlicense](https://unlicense.org/). The art are [CC0](https://creativecommons.org/publicdomain/zero/1.0/) assets provided for free by [Kenney](https://kenney.nl/assets).


## Why the name on game's title

Making games is arduous and honest work. Musicians, illustrators and many other professionals always sign their works. People who make games should not be afraid of doing so as well. Check [Bennett Foddy and Zach Gage's video](https://www.youtube.com/watch?v=N4UFC0y1tY0) to learn more about this.



<!-- More Links -->

[patreon]: https://patreon.com/KennedyRichard
[github sponsors]: https://github.com/sponsors/KennedyRichard
[liberapay]: https://liberapay.com/KennedyRichard
[Ko-fi]: https://ko-fi.com/kennedyrichard
[donation page]: https://indiesmiths.com/donate
