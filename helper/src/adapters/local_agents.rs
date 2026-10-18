//! Metadata-only host detection. Never opens config, credential, or transcript files.
use serde_json::{Value, json};
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

pub fn detect(home: &Path, search_path: &[PathBuf]) -> Value {
    let agents = [
        ("codex","Codex",vec![home.join(".codex")],true),
        ("cursor","Cursor",vec![home.join(".cursor")],true),
        ("devin","Devin CLI",vec![home.join(".devin"),home.join(".config/devin"),home.join(".local/share/devin/cli")],true),
        ("claude","Claude Code",vec![home.join(".claude")],true),
        ("opencode","OpenCode",vec![home.join(".config/opencode"),home.join(".local/share/opencode")],true),
    ].into_iter().map(|(id,name,mut paths,supported)|{
        if id=="codex" && let Some(path)=std::env::var_os("CODEX_HOME") {paths.push(path.into());}
        if id=="cursor" && let Some(path)=std::env::var_os("MIILO_CURSOR_PROJECTS") {paths.push(path.into());}
        if id=="claude" && let Some(path)=std::env::var_os("CLAUDE_CONFIG_DIR") {paths.push(path.into());}
        if matches!(id,"opencode"|"devin") && let Some(data)=std::env::var_os("XDG_DATA_HOME") {paths.push(PathBuf::from(data).join(if id=="opencode"{"opencode"}else{"devin/cli"}));}
        let folders:Vec<_>=paths.into_iter().filter(|path|path.is_dir()).collect();
        let executable=search_path.iter().any(|directory| fs::metadata(directory.join(if id=="claude"{"claude"}else{id})).is_ok_and(|metadata|metadata.is_file() && metadata.permissions().mode()&0o111!=0));
        json!({"id":id,"name":name,"found":executable||!folders.is_empty(),"commandFound":executable,"configFound":!folders.is_empty(),"locations":folders,"observationSupported":supported})
    }).collect::<Vec<_>>();
    json!({"agents":agents,"metadataOnly":true})
}
