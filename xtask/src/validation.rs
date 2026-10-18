//! Execute the official compiler/evaluator and retain the original UI assertions.
use crate::{Result, fixtures, output, root};
use serde_json::{Value, json};
use std::{env, fs, process::Command};
fn nodes<'a>(tree: &'a Value, out: &mut Vec<&'a Value>) {
    for node in tree.as_array().into_iter().flatten() {
        out.push(node);
        nodes(&node["children"], out);
    }
}
fn all(tree: &Value) -> Vec<&Value> {
    let mut out = vec![];
    nodes(&tree["tile"], &mut out);
    nodes(&tree["screen"], &mut out);
    out
}
fn texts<'a>(tree: &'a Value, part: &str) -> Vec<&'a str> {
    let mut out = vec![];
    nodes(&tree[part], &mut out);
    out.into_iter()
        .filter(|n| n["kind"] == "Text" || n["kind"] == "Button")
        .filter_map(|n| n["props"]["text"].as_str())
        .collect()
}
fn toolbar_path(screen: &[&Value], start: [f64; 2]) -> bool {
    screen.iter().any(|node| {
        node["kind"] == "Canvas"
            && node["props"]["width"] == 24
            && node["props"]["height"] == 24
            && node["children"].as_array().is_some_and(|draws| {
                draws.iter().any(|draw| {
                    draw["op"] == "path"
                        && draw["args"][0][0][0].as_f64() == Some(start[0])
                        && draw["args"][0][0][1].as_f64() == Some(start[1])
                })
            })
    })
}
fn row_widths(tree: &Value) {
    for row in all(tree) {
        let children = row["children"].as_array().map(Vec::as_slice).unwrap_or(&[]);
        if row["kind"] == "Row"
            && children
                .iter()
                .any(|c| c["props"]["weight"].as_f64().unwrap_or(0.) > 0.)
        {
            for child in children {
                if child["kind"] == "Text" && child["props"]["weight"].as_f64().unwrap_or(0.) == 0.
                {
                    assert!(
                        child["props"]["width"].as_f64().unwrap_or(0.) > 0.,
                        "Trailing Row text requires explicit width"
                    );
                }
            }
        }
    }
}
pub fn check() -> Result<()> {
    let root = root();
    let owl = env::var("OWL_CHECKER_CLASSPATH")?;
    let compiler = env::var("KOTLIN_COMPILER_CLASSPATH")?;
    let files = [
        "70-miilo-fleet.owl",
        "71-miilo-attention.owl",
        "72-miilo-board.owl",
    ];
    print!(
        "{}",
        output(
            Command::new("java")
                .args(["-cp", &owl, "dev.omalauncher.widgets.cli.MainKt", "check"])
                .args(files.map(|f| root.join(f)))
        )?
    );
    let folder = tempfile::tempdir()?;
    output(
        Command::new("java")
            .args([
                "-cp",
                &compiler,
                "org.jetbrains.kotlin.cli.jvm.K2JVMCompiler",
                "-no-stdlib",
                "-no-reflect",
                "-classpath",
                &owl,
                "-d",
            ])
            .arg(folder.path())
            .arg(root.join("tools/Inspect.kt")),
    )?;
    let cases = fixtures::write(folder.path())?;
    let classpath = format!("{}:{}", folder.path().display(), owl);
    let evaluate = |file: &str,
                    case: &str,
                    width: i32,
                    height: i32,
                    theme: &str,
                    scenario: &str|
     -> Result<Value> {
        Ok(serde_json::from_str(&output(
            Command::new("java")
                .args(["-cp", &classpath, "InspectKt"])
                .arg(root.join(file))
                .arg(folder.path().join(format!("{case}.json")))
                .args([
                    width.to_string(),
                    height.to_string(),
                    theme.into(),
                    scenario.into(),
                ]),
        )?)?)
    };
    let mut total = 0;
    for theme in ["dark", "light"] {
        for (width, height) in [
            (120, 64),
            (168, 112),
            (180, 180),
            (240, 180),
            (360, 180),
            (180, 300),
        ] {
            for (case, data) in &cases {
                for file in files {
                    let mut result = evaluate(file, case, width, height, theme, "default")?;
                    assert!(!result["tile"].as_array().unwrap().is_empty());
                    assert!(!result["screen"].as_array().unwrap().is_empty());
                    row_widths(&result);
                    let labels = texts(&result, "tile");
                    let feed = file.starts_with("70");
                    let mut tile = vec![];
                    nodes(&result["tile"], &mut tile);
                    if feed && case == "populated" {
                        for work in data["snapshot"]["workItems"].as_array().unwrap() {
                            assert!(!labels.contains(&work["title"].as_str().unwrap()));
                        }
                        assert!(
                            !tile
                                .iter()
                                .any(|n| n["kind"] == "Canvas" && n["props"]["height"] == 25)
                        );
                    }
                    if feed {
                        for node in &tile {
                            if node["kind"] == "Canvas" {
                                assert!(
                                    node["props"]["width"].as_f64().unwrap_or(0.)
                                        <= (width - 16) as f64
                                );
                                assert!(
                                    node["props"]["height"].as_f64().unwrap_or(0.)
                                        <= (height - 16) as f64
                                );
                            }
                            if node["kind"] == "Text" && node["props"]["maxLines"] == 2 {
                                assert!(
                                    node["props"]["height"] == 24 || node["props"]["height"] == 28
                                );
                            }
                        }
                    }
                    if case == "disconnected" {
                        assert!(labels.contains(&if feed {
                            "Tap to add a system"
                        } else {
                            "Tap to add systems"
                        }));
                        if feed {
                            assert!(tile.iter().any(|n| n["kind"] == "Canvas"));
                        }
                    }
                    if case == "empty" {
                        assert!(
                            labels.contains(&"Nothing needs you")
                                || labels.contains(&"No work in flight")
                        );
                    }
                    if case == "populated"
                        && ((feed && width == 180) || (!feed && width == 168))
                        && let Some(destination) = env::var_os("PREVIEW_DIR")
                    {
                        fs::create_dir_all(&destination)?;
                        result["width"] = json!(width);
                        result["height"] = json!(height);
                        fs::write(
                            std::path::Path::new(&destination)
                                .join(format!("{}-{theme}.json", file.trim_end_matches(".owl"))),
                            serde_json::to_vec(&result)?,
                        )?;
                    }
                    total += 1;
                }
            }
        }
    }
    for file in files {
        for scenario in [
            "detail",
            "confirm",
            "filter",
            "actor",
            "interaction",
            "group",
            "search",
            "no-match",
            "agent-tab",
            "needs",
            "left-hand",
            "preview",
            "colors",
            "search-edit",
            "search-done",
            "search-clear",
        ] {
            let result = evaluate(file, "multiple", 180, 180, "dark", scenario)?;
            row_widths(&result);
            let labels = texts(&result, "screen");
            let feed = file.starts_with("70");
            let mut screen = vec![];
            nodes(&result["screen"], &mut screen);
            if scenario == "group" && feed {
                assert!(!labels.iter().any(|t| t.starts_with("Agents ")));
            }
            if scenario == "agent-tab" && feed {
                assert!(labels.contains(&"All work"));
                let actor = &cases["multiple"]["snapshot"]["actors"][0];
                for work in cases["multiple"]["snapshot"]["workItems"]
                    .as_array()
                    .unwrap()
                {
                    if work["providerId"] == actor["providerId"]
                        && !work["_actorKeys"]
                            .as_array()
                            .is_some_and(|v| v.contains(&actor["_key"]))
                    {
                        assert!(!labels.contains(&work["title"].as_str().unwrap()));
                    }
                }
            }
            if scenario == "preview" && feed {
                assert!(texts(&result, "tile").contains(&"Preview sleep"));
                assert!(!labels.iter().any(|t| t.contains("Preview ")));
            }
            if scenario == "search-edit" {
                assert!(
                    screen
                        .iter()
                        .any(|n| n["kind"] == "Input" && n["props"]["focus"] == true)
                );
                assert!(!screen.iter().any(|n| n["kind"] == "Canvas"
                    && n["props"]["width"] == 24
                    && n["props"]["height"] == 24));
                assert!(!labels.contains(&"↻"));
                assert!(labels.contains(&"✓"));
            }
            if matches!(scenario, "search-done" | "search-clear") {
                assert!(!screen.iter().any(|n| n["kind"] == "Input"));
                assert!(screen.iter().any(|n| n["kind"] == "Canvas"
                    && n["props"]["width"] == 24
                    && n["props"]["height"] == 24));
                assert!(toolbar_path(&screen, [15.1, 7.0]), "Refresh vector missing");
            }
            if scenario == "confirm" {
                assert!(labels.contains(&"Confirm") && labels.contains(&"Cancel"));
            }
            if scenario == "detail" {
                assert!(labels.iter().any(|t| t.contains("No repeated events")));
                let rows = screen
                    .iter()
                    .filter(|n| {
                        n["kind"] == "Flow"
                            && ["Explain", "Approve", "Reject", "Retry"]
                                .iter()
                                .all(|label| {
                                    n["children"].as_array().is_some_and(|cs| {
                                        cs.iter().any(|c| c["props"]["text"] == *label)
                                    })
                                })
                    })
                    .count();
                assert_eq!(rows, 1, "Explain and actions must share one row");
                assert!(
                    labels.contains(&"src/worker.rs")
                        && labels.contains(&"Diff not supplied")
                        && toolbar_path(&screen, [20.0, 12.0])
                );
                assert!(labels.iter().any(|t| t.contains("@@ -1 +1 @@")));
                assert!(!screen.iter().any(|n| (n["kind"] == "Button"
                    && n["props"]["text"] == "Evidence")
                    || n["kind"] == "Input"));
            }
            if scenario == "no-match" {
                assert!(!labels.iter().any(|t| t.contains("No repeated events")));
            }
            if scenario == "search" && (feed || file.starts_with("72")) {
                assert!(
                    labels.contains(
                        &cases["multiple"]["snapshot"]["workItems"][0]["title"]
                            .as_str()
                            .unwrap()
                    )
                );
            }
            if scenario == "filter" {
                assert!(
                    !labels
                        .iter()
                        .any(|t| t.starts_with("office /") && t.contains(" · "))
                );
            }
            total += 1;
        }
    }
    println!("PASS: {total} official OWL tile/screen evaluations");
    Ok(())
}
