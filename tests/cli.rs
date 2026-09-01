use std::process::Command;

use tempfile::TempDir;

fn scratch(dir: &TempDir, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_tic"))
        .args(args)
        .arg("--path")
        .arg(dir.path())
        .output()
        .expect("failed to run scratch")
}

#[test]
fn full_workflow() {
    let dir = tempfile::tempdir().unwrap();

    let add1 = scratch(&dir, &["add", "write report"]);
    assert!(add1.status.success());

    let add2 = scratch(&dir, &["add", "send email"]);
    assert!(add2.status.success());

    // `list` should show both tasks.
    let list = scratch(&dir, &["list"]);
    let out = String::from_utf8_lossy(&list.stdout);
    assert!(out.contains("write report"));
    assert!(out.contains("send email"));

    // Mark #1 done, remove #2.
    let done = scratch(&dir, &["done", "1"]);
    assert!(done.status.success());
    let remove = scratch(&dir, &["remove", "2"]);
    assert!(remove.status.success());

    // Final `list` should show task #1 done, and task #2 gone.
    let final_list = scratch(&dir, &["list"]);
    let out = String::from_utf8_lossy(&final_list.stdout);
    assert!(out.contains("[x] #1 write report"));
    assert!(!out.contains("send email"));
}
