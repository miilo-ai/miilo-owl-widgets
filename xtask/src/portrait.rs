use crate::{Result, root};
use std::fs;

// Generate the portrait program from the same authored robot and palette source.
// Native onboarding must not carry a second, simplified character implementation.
pub fn assemble() -> Result<()> {
    let root = root();
    let common = fs::read_to_string(root.join("widgets/common.inc"))?;
    let palette = common
        .split_once("  let robotPalette = [")
        .and_then(|(_, tail)| tail.split_once("\n  ]"))
        .ok_or("Shared robot palette not found")?
        .0;
    let source = format!(
        "widget \"miilo-portrait\" {{\n size: 1x1\n{}\n state portraitAgent = {{_key: \"welcome\", name: \"Miilo\", status: \"ready\", health: \"healthy\"}}\n state portraitSize = 64\n state robotColors = []\n let actors = [portraitAgent]\n let works = []\n let robotPalette = [{}\n ]\n{}\n Bot {{ agent: portraitAgent; s: portraitSize }}\n}}\n",
        fs::read_to_string(root.join("widgets/appearance.inc"))?,
        palette,
        fs::read_to_string(root.join("widgets/robot.inc"))?
    );
    fs::write(
        root.join("desktop/RobotProgram.js"),
        format!(
            ".pragma library\n// Generated from shared robot.inc, appearance.inc and palette. Do not edit.\nvar source = {};\n",
            serde_json::to_string(&source)?
        ),
    )?;
    Ok(())
}
