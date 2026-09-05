package quanttide_execute_test

import (
	"encoding/json"
	"os"
	"testing"

	quanttide_execute "github.com/quanttide/quanttide-execute-toolkit/packages/go/pkg"
)

func fixture(t *testing.T, name string) []byte {
	t.Helper()
	raw, err := os.ReadFile("../../../tests/fixtures/" + name + ".json")
	if err != nil {
		t.Fatalf("fixture %s 不存在: %v", name, err)
	}
	return raw
}

func roundTrip(t *testing.T, raw []byte, v any) {
	t.Helper()
	if err := json.Unmarshal(raw, v); err != nil {
		t.Fatalf("反序列化失败: %v", err)
	}
	data, err := json.Marshal(v)
	if err != nil {
		t.Fatalf("序列化失败: %v", err)
	}
	if err := json.Unmarshal(data, v); err != nil {
		t.Fatalf("round-trip 再解析失败: %v", err)
	}
}

func TestExecutorFixture(t *testing.T) {
	var e quanttide_execute.Executor
	roundTrip(t, fixture(t, "executor"), &e)
	if e.Label != "张三" {
		t.Errorf("label = %q", e.Label)
	}
}

func TestListFixture(t *testing.T) {
	var l quanttide_execute.List
	roundTrip(t, fixture(t, "list"), &l)
	if l.Name != "量潮数据" {
		t.Errorf("name = %q", l.Name)
	}
}

func TestTaskFixture(t *testing.T) {
	var task quanttide_execute.Task
	roundTrip(t, fixture(t, "task"), &task)
	if task.Status != quanttide_execute.StatusInProgress {
		t.Errorf("status = %q", task.Status)
	}
	if task.Priority != quanttide_execute.PriorityHigh {
		t.Errorf("priority = %q", task.Priority)
	}
	if !task.IsActive {
		t.Error("is_active 应为 true")
	}
	if task.CompletedAt != nil {
		t.Errorf("completed_at 应为 null，得到 %v", *task.CompletedAt)
	}
}

func TestOutcomeFixture(t *testing.T) {
	var o quanttide_execute.Outcome
	roundTrip(t, fixture(t, "outcome"), &o)
	if o.Title != "口径确认单验收通过" {
		t.Errorf("title = %q", o.Title)
	}
	if o.ListID != "5f0c9a1e-3d4b-4e8f-9a2c-1b7d6e5f4a3c" {
		t.Errorf("list_id = %q", o.ListID)
	}
}
