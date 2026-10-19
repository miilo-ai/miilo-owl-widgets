mod common;
use common::*;
use miilo_owl::{
    Result,
    setup::{self, Console},
};
use std::collections::VecDeque;

struct Script {
    answers: VecDeque<String>,
    secret_prompts: usize,
    messages: Vec<String>,
}
impl Script {
    fn new(answers: &[&str]) -> Self {
        Self {
            answers: answers.iter().map(|s| s.to_string()).collect(),
            secret_prompts: 0,
            messages: vec![],
        }
    }
}
impl Console for Script {
    fn say(&mut self, message: &str) -> Result<()> {
        self.messages.push(message.into());
        Ok(())
    }
    fn read(&mut self, _: &str, secret: bool) -> Result<Option<String>> {
        if secret {
            self.secret_prompts += 1;
        }
        Ok(self.answers.pop_front())
    }
}
#[test]
fn onboarding_selects_two_endpoints_and_removes_one() {
    let tmp = tempfile::tempdir().unwrap();
    let s = store(tmp.path(), vec![]);
    let http = Fake::default();
    let mut ui = Script::new(&[
        "a",
        "home",
        "http://127.0.0.1:3700",
        "secret",
        "all",
        "n",
        "a",
        "office",
        "http://127.0.0.1:3701",
        "secret",
        "1",
        "y",
        "q",
    ]);
    setup::setup(&s, &http, &mut ui).unwrap();
    assert_eq!(ui.secret_prompts, 2);
    assert!(!ui.messages.join("").contains("secret"));
    let config = s.load().unwrap();
    assert_eq!(config.endpoints.len(), 2);
    assert!(!config.endpoints[0].allow_actions);
    assert!(config.endpoints[1].allow_actions);
    setup::setup(&s, &http, &mut Script::new(&["r", "home", "q"])).unwrap();
    assert_eq!(s.load().unwrap().endpoints[0].id, "office");
}
#[test]
fn invalid_selection_preserves_config_and_can_be_corrected() {
    let tmp = tempfile::tempdir().unwrap();
    let s = store(tmp.path(), vec![endpoint()]);
    let mut ui = Script::new(&["a", "new", "http://127.0.0.1:3701", "secret", "0", "q"]);
    setup::setup(&s, &Fake::default(), &mut ui).unwrap();
    assert_eq!(s.load().unwrap().endpoints.len(), 1);
    assert!(ui.messages.iter().any(|s| s.contains("Not saved")));
    setup::setup(&s, &Fake::default(), &mut Script::new(&[])).unwrap();
}

#[test]
fn updating_paired_systems_preserves_the_pin_and_never_requests_a_credential() {
    let tmp = tempfile::tempdir().unwrap();
    let mut endpoint = endpoint();
    endpoint.token.clear();
    endpoint.allow_actions = false;
    endpoint.url = "https://example.invalid".into();
    endpoint.paired_host = Some(miilo_pairing::domain::PairedHost {
        host_id: "a".repeat(32),
        device_id: "b".repeat(32),
        name: "Workstation".into(),
        url: endpoint.url.clone(),
        certificate_pem: "fixture".into(),
        fingerprint: "c".repeat(64),
        token: "d".repeat(64),
        expires_at: 1,
    });
    let id = endpoint.id.clone();
    let s = store(tmp.path(), vec![endpoint.clone()]);
    let http = Fake::default();
    let mut ui = Script::new(&["a", &id, "all", "q"]);
    setup::setup(&s, &http, &mut ui).unwrap();
    let saved = s.load().unwrap();
    assert_eq!(ui.secret_prompts, 0);
    assert_eq!(
        saved.endpoints[0].paired_host.as_ref().unwrap().token,
        endpoint.paired_host.unwrap().token
    );
    assert!(!saved.endpoints[0].allow_actions);
    assert!(
        http.calls
            .lock()
            .unwrap()
            .iter()
            .all(|(_, path, body)| path == "/v1/snapshot" && body.is_none())
    );
}
