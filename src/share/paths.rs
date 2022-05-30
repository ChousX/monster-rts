pub const MAP_ISO: &str = if cfg!(target_os = "windows") {r"textures\iso_color.png"} else {r"textures/iso_color.png"};
pub const FONT: &str = if cfg!(target_os = "windows") {r"fonts\OpenDyslexicMono-Regular.otf"} else {r"fonts/OpenDyslexicMono-Regular.otf"};
pub const CURSOR: &str = if cfg!(target_os = "windows") {r"textures\cursors\Crosshair.png"} else {r"textures/cursors/Crosshair.png"};