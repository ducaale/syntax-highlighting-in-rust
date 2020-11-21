# Syntax higlighting in Rust

This repo contains my experiments with [syntect](https://github.com/trishume/syntect) +
custom syntaxes and themes. Initially, I faced some problems in converting from `.tmLanguage`
to `.sublime-syntax` due to a bug in Sublime Text. Luckily, there was a workaround which is
mentioned [here](https://forum.sublimetext.com/t/trying-to-convert-tmlanguage-syntax-to-sublime-syntax/53427).

## Syntaxes and themes used
- [Sublime-HTTP](https://github.com/samsalisbury/Sublime-HTTP)
- [json-kv](https://github.com/aurule/json-kv)
- [Sublime Packages](https://github.com/sublimehq/Packages/tree/fa6b8629c95041bf262d4c1dab95c456a0530122)
- [ansi-dark theme](https://github.com/sharkdp/bat/blob/master/assets/themes/ansi-dark.tmTheme)

## TODO:
- Figure out how convert assets into a dump file. See https://github.com/trishume/syntect/blob/master/examples/gendata.rs
- Investigate generating bin files as part of build.rs script. See https://doc.rust-lang.org/cargo/reference/build-scripts.html
- Include dump in the final binary. See https://stackoverflow.com/questions/61818515/
- Turn off the inclusion of default assets. See https://github.com/trishume/syntect/issues/278#issuecomment-576926815