use wasm_bindgen::prelude::*;
use libmathcat::interface::*;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Once;

static INIT: Once = Once::new();

fn ensure_init() {
    INIT.call_once(|| { init_panic_handler(); });
}

fn panic_msg(payload: Box<dyn std::any::Any + Send>) -> String {
    payload.downcast_ref::<&str>().map(|s| s.to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "unknown panic payload".to_string())
}

#[wasm_bindgen]
pub fn nemeth_from_mathml(mathml: &str) -> String {
    ensure_init();
    let mathml = mathml.to_string();
    let result = catch_unwind(AssertUnwindSafe(|| -> Result<String, String> {
        set_rules_dir("Rules".to_string()).map_err(|e| format!("set_rules_dir error: {:?}", e))?;
        set_preference("CheckRuleFiles".to_string(), "None".to_string())
            .map_err(|e| format!("set_preference CheckRuleFiles error: {:?}", e))?;
        set_preference("BrailleCode".to_string(), "Nemeth".to_string())
            .map_err(|e| format!("set_preference BrailleCode error: {:?}", e))?;
        set_preference("Language".to_string(), "zh".to_string())
            .map_err(|e| format!("set_preference Language error: {:?}", e))?;
        set_mathml(mathml).map_err(|e| format!("set_mathml error: {:?}", e))?;
        get_braille("".to_string()).map_err(|e| format!("get_braille error: {:?}", e))
    }));

    match result {
        Ok(Ok(s)) => s,
        Ok(Err(e)) => format!("ERROR: {}", e),
        Err(payload) => format!("PANIC: {}", panic_msg(payload)),
    }
}

#[wasm_bindgen]
pub fn spoken_text() -> String {
    ensure_init();
    let result = catch_unwind(AssertUnwindSafe(|| -> Result<String, String> {
        get_spoken_text().map_err(|e| format!("get_spoken_text error: {:?}", e))
    }));
    match result {
        Ok(Ok(s)) => s,
        Ok(Err(e)) => format!("ERROR: {}", e),
        Err(payload) => format!("PANIC: {}", panic_msg(payload)),
    }
}

/// command 是 MathCAT 的高階導覽指令字串，例如
/// MoveNext / MovePrevious / ZoomIn / ZoomOut / ZoomOutAll / ReadCurrent / Exit
#[wasm_bindgen]
pub fn navigate(command: &str) -> String {
    ensure_init();
    let command = command.to_string();
    let result = catch_unwind(AssertUnwindSafe(|| -> Result<String, String> {
        do_navigate_command(command).map_err(|e| format!("do_navigate_command error: {:?}", e))
    }));
    match result {
        Ok(Ok(s)) => s,
        Ok(Err(e)) => format!("ERROR: {}", e),
        Err(payload) => format!("PANIC: {}", panic_msg(payload)),
    }
}

#[wasm_bindgen]
pub fn nav_braille() -> String {
    ensure_init();
    let result = catch_unwind(AssertUnwindSafe(|| -> Result<String, String> {
        get_navigation_braille().map_err(|e| format!("get_navigation_braille error: {:?}", e))
    }));
    match result {
        Ok(Ok(s)) => s,
        Ok(Err(e)) => format!("ERROR: {}", e),
        Err(payload) => format!("PANIC: {}", panic_msg(payload)),
    }
}
