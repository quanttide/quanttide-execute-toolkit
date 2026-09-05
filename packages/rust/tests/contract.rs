//! 契约测试：与根 tests/ 的 Schema + Fixture 对齐。

use quanttide_execute::{Executor, List, Outcome, Task};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "{}/../../tests/fixtures/{name}.json",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap_or_else(|_| panic!("fixture {name} 不存在"))
}

#[test]
fn executor_fixture_round_trip() {
    let e: Executor = serde_json::from_str(&fixture("executor")).unwrap();
    assert_eq!(e.label, "张三");
    let json = serde_json::to_string(&e).unwrap();
    let e2: Executor = serde_json::from_str(&json).unwrap();
    assert_eq!(e, e2);
}

#[test]
fn list_fixture_round_trip() {
    let l: List = serde_json::from_str(&fixture("list")).unwrap();
    assert_eq!(l.name, "量潮数据");
    let json = serde_json::to_string(&l).unwrap();
    let l2: List = serde_json::from_str(&json).unwrap();
    assert_eq!(l, l2);
}

#[test]
fn task_fixture_round_trip() {
    let t: Task = serde_json::from_str(&fixture("task")).unwrap();
    assert_eq!(t.status, "in_progress");
    assert_eq!(t.priority, "high");
    assert!(t.is_active);
    assert_eq!(t.completed_at, None);
    let json = serde_json::to_string(&t).unwrap();
    let t2: Task = serde_json::from_str(&json).unwrap();
    assert_eq!(t, t2);
}

#[test]
fn outcome_fixture_round_trip() {
    let o: Outcome = serde_json::from_str(&fixture("outcome")).unwrap();
    assert_eq!(o.title, "口径确认单验收通过");
    assert_eq!(o.list_id, "5f0c9a1e-3d4b-4e8f-9a2c-1b7d6e5f4a3c");
    let json = serde_json::to_string(&o).unwrap();
    let o2: Outcome = serde_json::from_str(&json).unwrap();
    assert_eq!(o, o2);
}
