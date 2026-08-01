# MathCAT Lab

實驗性的單頁工具：LaTeX → MathML（[MathJax](https://www.mathjax.org/) v4）→ Nemeth 點字（[MathCAT](https://github.com/daisy/MathCAT)，WASM）。

**不是正式工具**，只做單一算式的單向轉換，目的是拿 MathCAT 的輸出跟
[vi-tools](https://github.com/hurthuang/vi-tools) 裡 `nemeth_converter.html`（π 數學點字，nc）
的輸出互相對照、驗證彼此的 Nemeth 點字轉換正確性。兩邊是完全獨立的實作，沒有共用程式碼，
輸出不一致不代表任一邊一定錯，需要對照官方 *The Nemeth Braille Code for Mathematics and
Science Notation, 2022* 規則書判斷。

線上：<https://hurthuang.github.io/mathcat-lab/>（若已啟用 GitHub Pages）

## 背景

MathCAT 是 [DAISY Consortium](https://daisy.org/) 開發、Neil Soiffer 主導的 Rust 函式庫，
把 MathML 轉成語音跟點字（Nemeth、UEB Technical 等），是 NVDA 2026.1+ 內建、JAWS 2024+
支援的官方數學報讀引擎，也是 BrailleBlaster 用來產生 Nemeth 點字的引擎，內建超過 700 條測試。

vi-tools 的 `nemeth_converter.html`（nc）是完全獨立手刻的 LaTeX↔Nemeth 轉換器，已經用官方
規則書逐條稽核過 8 輪。用 MathCAT 當第二套獨立實作互相比對，能抓到規則書人工稽核漏掉的問題
（也可能反過來抓到 MathCAT 的問題——過程中就抓到一個，見下方「已知問題」）。

## 檔案結構

```
index.html              頁面本體（LaTeX 輸入 / MathJax 預覽 / Nemeth 輸出）
mathcat-wasm/
  mathcat_nemeth.js      wasm-bindgen 產生的 JS 膠水程式（--target web）
  mathcat_nemeth_bg.wasm 編譯好的 MathCAT 引擎（含內嵌 Nemeth 規則檔，~3.5MB）
```

純靜態頁面，開瀏覽器（需 http:// 而非 file://，wasm 的 ES module 匯入受瀏覽器安全限制）
直接用即可，沒有任何 build step 或伺服器端邏輯。

## 已知問題

- **MathCAT 餵到 `<merror>` 節點會直接讓 WASM 崩潰**（`RuntimeError: unreachable`，Rust 端
  `catch_unwind` 攔不到，判斷是真正的 trap 不是一般 panic）。這是 MathCAT 本身的 bug，還沒
  回報上游。`index.html` 裡送進引擎前會先檢查 MathML 有沒有 `<merror>`，有的話當「輸入還沒
  打完」處理、不呼叫 WASM——這是繞過，不是修好，MathCAT 本身這個 bug 還在。
- **巢狀下標（例如 `x_{i_1}`）MathCAT 比官方規則書多插入一個下標指示符**：跟 nc 對比時發現，
  nc 的輸出（兩個指示符）符合 Nemeth 2022 規則書 Rule 14.6 Example 14-40 的官方範例，
  MathCAT 算出三個。已排除是我們這邊 MathML 結構的問題（拿掉多餘 `<mrow>` 包裝重測結果一樣）。
  沒有回報上游，只在這裡記錄。
- **crates.io 上發布的 `mathcat` crate（0.6.10）build script 有 bug**（`include-zip` feature
  的 zip 解壓縮 feature flag 沒開全，會編譯失敗），必須改用 GitHub 原始碼（見下方建置方式）
  才能編譯成功——這也是 MathCAT 官方 demo（[MathCATDemo](https://github.com/daisy/MathCATDemo)）
  自己 `Cargo.toml` 採用的方式，不是我們自己發明的繞路。

## 建置方式（如何重新編譯 wasm）

MathCAT 沒有發布現成的 WASM 檔案可以直接下載，必須自己用 Rust 工具鏈編譯。這代表**這個專案
長期需要有人備有 Rust + wasm-pack 環境才能更新引擎版本**，跟 vi-tools 其他工具用的 liblouis
WASM（直接下載別人編譯好的檔案）維護方式不同，是這個 repo 存在的主要限制。

需要：
- Rust（`rustup`）+ `wasm32-unknown-unknown` target
- Windows 上另外需要 MSVC Build Tools（`link.exe`/`cl.exe`），例如透過
  `winget install Microsoft.VisualStudio.2022.BuildTools`（勾 C++ 工作負載）
- `wasm-pack`（`cargo install wasm-pack`）

建一個暫存 Rust 專案，`Cargo.toml`：

```toml
[package]
name = "mathcat-nemeth"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"

[dependencies.mathcat]
git = "https://github.com/daisy/MathCAT"
features = ["include-zip"]

[profile.release]
opt-level = "z"
```

`src/lib.rs`：

```rust
use wasm_bindgen::prelude::*;
use libmathcat::interface::*;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Once;

static INIT: Once = Once::new();

#[wasm_bindgen]
pub fn nemeth_from_mathml(mathml: &str) -> String {
    INIT.call_once(|| { init_panic_handler(); });

    let mathml = mathml.to_string();
    let result = catch_unwind(AssertUnwindSafe(|| -> Result<String, String> {
        set_rules_dir("Rules".to_string()).map_err(|e| format!("set_rules_dir error: {:?}", e))?;
        set_preference("CheckRuleFiles".to_string(), "None".to_string())
            .map_err(|e| format!("set_preference CheckRuleFiles error: {:?}", e))?;
        set_preference("BrailleCode".to_string(), "Nemeth".to_string())
            .map_err(|e| format!("set_preference BrailleCode error: {:?}", e))?;
        set_mathml(mathml).map_err(|e| format!("set_mathml error: {:?}", e))?;
        get_braille("".to_string()).map_err(|e| format!("get_braille error: {:?}", e))
    }));

    match result {
        Ok(Ok(s)) => s,
        Ok(Err(e)) => format!("ERROR: {}", e),
        Err(payload) => {
            let msg = payload.downcast_ref::<&str>().map(|s| s.to_string())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "unknown panic payload".to_string());
            format!("PANIC: {}", msg)
        }
    }
}
```

編譯（Windows，要先跑過 `vcvars64.bat` 載入 MSVC 環境變數）：

```
wasm-pack build --target web --out-name mathcat_nemeth --out-dir pkg
```

把 `pkg/mathcat_nemeth.js`、`pkg/mathcat_nemeth_bg.wasm` 複製進本 repo 的 `mathcat-wasm/` 即可。

`catch_unwind` 能攔到一般的 Rust panic（例如某些不支援的 MathML 結構），但攔不到上面提到的
`<merror>` 崩潰——那個是更底層的 trap，前端頁面自己的 `<merror>` 偵測才是真正有效的防護。

## 授權

本 repo 程式碼沒有特別授權聲明（比照 [vi-tools](https://github.com/hurthuang/vi-tools) 現況）。
MathCAT 本身是 MIT 授權，見 [daisy/MathCAT](https://github.com/daisy/MathCAT)。
