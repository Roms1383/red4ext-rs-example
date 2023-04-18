set dotenv-load

DEFAULT_GAME_DIR := join("C:\\", "Program Files (x86)", "Steam", "steamapps", "common", "Cyberpunk 2077")

game_dir := env_var_or_default("GAME_DIR", DEFAULT_GAME_DIR)

mod_name           := "example"
bin_name           := "example.dll"

red4ext_repo_dir   := join(".", "target", "release")
red4ext_game_dir   := join(game_dir, "red4ext", "plugins")

redscript_repo_dir := join(".", "reds")
redscript_game_dir := join(game_dir, "r6", "scripts")

# list all commands
_default:
  @just --list --unsorted
  @echo "⚠️  on Windows, paths defined in .env must be double-escaped:"
  @echo 'e.g. GAME_DIR="C:\\\\path\\\\to\\\\your\\\\game\\\\folder"'

# copy .reds files to game folder (can be hot-reloaded in-game)
hot-reload:
 cp -R '{{redscript_repo_dir}}' '{{ join(redscript_game_dir, mod_name) }}'

# copy all files to game folder (before launching the game)
install:
 cargo build --release
 mkdir -p '{{ join(red4ext_game_dir, mod_name) }}'
 cp '{{ join(red4ext_repo_dir, bin_name) }}' '{{ join(red4ext_game_dir, mod_name, bin_name) }}'
 @just hot-reload

# remove all files from game folder
uninstall:
 rm -rf '{{ join(red4ext_game_dir, mod_name) }}'
 rm -rf '{{ join(redscript_game_dir, mod_name) }}'

# display red4ext logs
logs:
 cat '{{ join(game_dir, "red4ext", "logs", "red4ext.log") }}'