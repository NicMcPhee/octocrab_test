# `octocrab` test

This simple yew project compiles and runs fine, but if we uncomment the
[octocrab](https://github.com/XAMPPRocky/octocrab)
dependency in `Cargo.toml` and run `trunk serve --open` then the compilation fails,
and I have no idea why. It smells somewhat like a dependency issue, like perhaps
octocrab depends on a different version of some library and that's causing the problems,
but that's just a guess.

Apparently the problem is [a fight between the `reqwest` package that Octocrab uses
and much of the rest of the world](https://github.com/XAMPPRocky/octocrab/issues/224#issuecomment-1179603852).
The plan over in Octocrab-land is to replace the dependency on `reqwest` with
`http` and `tower`, but it's not clear how quickly that will happen so it's not
entirely clear what our best bet is here.
