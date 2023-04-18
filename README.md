## red4ext-rs example project
This is a template RED4ext plugin written in Rust.
It contains a CI job that produces a ZIP archive with your code nicely packaged as a Cyberpunk2077 mod.

### requirements
- [RED4ext](https://github.com/WopsS/RED4ext.SDK) (for deployment in the game)
- [redscript](https://github.com/jac3km4/redscript) (for deployment in the game)
  - Only required for the native function definition.

### how to quickly try it out?

1. install [Just](https://github.com/casey/just#installation) command runner
   ⚠️ if you haven't installed Cyberpunk with Steam in the default folder,
      write an `.env` file with:
      ```.env
      GAME_DIR="PATH\\TO\\CYBERPUNK\\GAME\\FOLDER"
      ```
2. run `just install`: it will copy the files to your game folder
3. once in-game, try it out by launching the methods through CET console, as explained in [reds/debug.reds](./reds/debug.reds).
4. once finished, run `just uninstall`: it will remove them from your game folder

### build
```
cargo build --release
```
This will produce a DLL file in `target/release/red4ext_rs_example.dll`.
It needs to be placed in `Cyberpunk 2077/red4ext/plugins` for RED4ext.
