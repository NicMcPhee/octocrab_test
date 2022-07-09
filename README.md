# `octocrab` test

This simple yew project compiles and runs fine, but if we uncomment the
[octocrab](https://github.com/XAMPPRocky/octocrab)
dependency in `Cargo.toml` and run `trunk serve --open` then the compilation fails,
and I have no idea why. It smells somewhat like a dependency issue, like perhaps
octocrab depends on a different version of some library and that's causing the problems,
but that's just a guess.
