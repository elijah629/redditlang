# RedditLang

<picture>
  <source srcset="./assets/redditLang-full-dark.png" media="(prefers-color-scheme: dark)" width="450"/>
  <img src="./assets/redditLang-full-light.png" width="450"/>
</picture>

redditLang or PHL ( **P**rogramming **H**umor **L**anguage ) is a meme language devised by the r/ProgrammerHumor subreddit discord members

The spec is avaliable [here](./RedditLang%20Spec.md)

## Why Redditlang?

Redditlang is

- 🔥 blazingly fast
- 🎮 cross platform
- Redditlang is the most opinionated language out there, forcing only the best of practices. For example, if you use inline macros, the police will be called and informed a fire has broken out at your home.\*

So what are you waiting for? Please invest your life savings in this!
_[Check out the Official VSCode extension here](https://github.com/elijah629/redditlang-vscode)_

\*_Compiler\*\* is WIP, if you are looking for it. It will be here_  
\*\*_It may be Interpreted or JIT, but that is implementation specific_
**note**: We have realized that we have a younger audience, so we have translated the spec into Gen Z Slang "to make it bussin" [here](./RedditLang%20Spec%20GenZ.md)

## Requirements

LLVM 15.0.x

### Note

For some ubuntu users you might get an error about `Polly` not being found. Install it with `sudo apt install libpolly-15-dev`
See [this](https://gitlab.com/taricorp/llvm-sys.rs/-/issues/13) for more information
