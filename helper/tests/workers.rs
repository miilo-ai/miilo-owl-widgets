use miilo_owl::{
    adapters::workers,
    config::Store,
    domain::workers::{WorkerConfig, validate},
};
use serde_json::json;
#[test]
fn missing_backend_does_not_enroll_or_read_credentials() {
    let temp = tempfile::tempdir().unwrap();
    let store = Store {
        directory: temp.path().to_owned(),
    };
    assert_eq!(
        workers::dispatch(&store, &["list".into()]).unwrap()["status"],
        "unconfigured"
    );
    assert!(!temp.path().join("workers.json").exists());
}
#[test]
fn worker_setup_contains_only_explicit_endpoint_and_keyring_references() {
    let temp = tempfile::tempdir().unwrap();
    let store = Store {
        directory: temp.path().to_owned(),
    };
    let value = json!({"version":1,"endpoint":"https://factory.example.invalid","cliExecutable":"/usr/bin/miilo","collection":"/org/freedesktop/secrets/collection/Test","application":"miilo-workers","workstation":"fixture","gateway":"factory"});
    workers::configure(&store, value.to_string().as_bytes()).unwrap();
    let mut config: WorkerConfig = serde_json::from_value(value).unwrap();
    assert!(validate(&config).is_ok());
    config.endpoint = "http://remote.example.invalid".into();
    assert!(validate(&config).is_err());
    config.endpoint = "https://user:password@factory.example.invalid".into();
    assert!(validate(&config).is_err());
    config.endpoint = "http://127.0.0.1:33550".into();
    assert!(validate(&config).is_ok());
    config.application = "--secret-flag".into();
    assert!(validate(&config).is_err());
    assert!(
        workers::dispatch(
            &store,
            &[
                "available".into(),
                "../other".into(),
                "0".into(),
                "request-001".into()
            ]
        )
        .is_err()
    );
}
