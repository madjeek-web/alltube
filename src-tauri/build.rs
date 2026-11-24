// build.rs : petit programme lancé AVANT la compilation.
// Il prépare tout ce dont Tauri a besoin (icônes, droits, configuration).
fn main() {
    tauri_build::build()
}
