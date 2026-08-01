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

## 數學編輯器

「📝 數學編輯器」是直接從 vi-tools 的 `nemeth_converter.html` 整段複製過來的（按鈕、彈出視窗、
CSS、JS 全部照搬，只改了「套用到轉換器」按鈕呼叫的函式名稱以接上這裡的轉換流程）。nc 裡這段
本身是整合自外部的「通用數學文件編輯器」（`math-editor3q.htm`），來源見
[108 數學課綱與 LaTeX 語法](https://class.kh.edu.tw/19061/bulletin/msg_view/620)。授權狀態不明確，
沿用 nc 已經在用的現況。

## 功能

- LaTeX → Nemeth 點字，Unicode／ASCII／ASCII+SimBraille 字型三種顯示格式可切換。
- 🔊 朗讀整個算式、🧭 導覽模式（MathCAT `navigate` 高階指令：ZoomIn/ZoomOut/MovePrevious/
  MoveNext/ReadCurrent，逐步探索算式的各個部分，每一步同步更新點字跟報讀文字），
  語音角色可選（`speechSynthesis.getVoices()`）。
- 📝 數學編輯器（照搬自 nc，見下方說明）。
- ☀ 高對比白底黑字主題切換（demo／投影用）。
- 版面：左欄輸入與操作控制、右欄視覺呈現／點字／報讀文字三格堆疊（2:3 比例），
  設計目的是拿來對外示範「NVDA 報讀數學」的實際狀態——真的用 NVDA 操作時，
  螢幕上同時看不到算式、點字、報讀文字三者，這個頁面刻意把三者並列方便展示。

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
- **多行空間排列（聯立方程式/矩陣/行列式）MathCAT 目前輸出不出正確點字**，包括未經修改的
  MathCAT 官方 demo 本身也測不出來——這點觀察是在跟 nc 對比時發現的：nc 對這幾種（Nemeth
  Rule 19 enlarged bracket 分組符號）都能正確輸出多行點字（每行各自帶正確的放大括號記號，
  真的是換行，不是擠成一行），逐條驗證過 `\begin{cases}`、`\begin{matrix}`、`\begin{vmatrix}`
  都對。方向反過來了：不是「兩邊都做不到的共同限制」，是 nc 這塊做得到、MathCAT（含官方版本）
  目前做不到。沒有回報上游，只在這裡記錄。**注意跟另一個常見混淆點區分**：Nemeth Rule 25
  （長除法、直式加減這類需要欄位對齊的空間排版算術題）才是 nc 自己也明確沒實作的範圍，
  跟這裡講的 Rule 19 分組符號多行輸出是不同規則、不同範圍，不要混在一起講。

## 建置方式（如何重新編譯 wasm）

MathCAT 沒有發布現成的 WASM 檔案可以直接下載，必須自己用 Rust 工具鏈編譯。這代表**這個專案
長期需要有人備有 Rust + wasm-pack 環境才能更新引擎版本**，跟 vi-tools 其他工具用的 liblouis
WASM（直接下載別人編譯好的檔案）維護方式不同，是這個 repo 存在的主要限制。

需要：
- Rust（`rustup`）+ `wasm32-unknown-unknown` target
- Windows 上另外需要 MSVC Build Tools（`link.exe`/`cl.exe`），例如透過
  `winget install Microsoft.VisualStudio.2022.BuildTools`（勾 C++ 工作負載）
- `wasm-pack`（`cargo install wasm-pack`）

建一個暫存 Rust 專案（或直接用本 repo 的 [`rust-src/`](rust-src/) 資料夾），`Cargo.toml`：

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

`src/lib.rs`（完整版本，含 `nemeth_from_mathml`／`spoken_text`／`navigate`／`nav_braille`
四個 wasm-bindgen 匯出函式，也存了一份在本 repo 的 [`rust-src/lib.rs`](rust-src/lib.rs) 備查）：

```rust
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
```

編譯（Windows，要先跑過 `vcvars64.bat` 載入 MSVC 環境變數）：

```
wasm-pack build --target web --out-name mathcat_nemeth --out-dir pkg
```

把 `pkg/mathcat_nemeth.js`、`pkg/mathcat_nemeth_bg.wasm` 複製進本 repo 的 `mathcat-wasm/` 即可。

`catch_unwind` 能攔到一般的 Rust panic（例如某些不支援的 MathML 結構），但攔不到上面提到的
`<merror>` 崩潰——那個是更底層的 trap，前端頁面自己的 `<merror>` 偵測才是真正有效的防護。

## MathJax 無障礙設定的坑（寫給以後的自己）

`index.html` 的 `window.MathJax` 設定踩過幾個坑，都是實測才發現、文件沒講清楚：

1. **combined bundle（`tex-mml-chtml.js`）不能跟 `loader.load` 混用**——combined bundle
   已經整包載好了，額外指定 `loader.load` 會讓瀏覽器嘗試動態抓不存在的檔案路徑，
   悄悄失敗、畫面不會報錯，但指定的元件實際上沒生效。
2. **`enableAssistiveMml` 在目前 CDN 版本已經不是合法選項**，主控台會噴
   `Invalid option "enableAssistiveMml" (no default value)`；真正有效的是
   `options.menuOptions.settings.assistiveMml`。
3. **combined bundle 預設會自己開一整套 Explorer/選單/語音**，會把單一 `<math>` 物件拆成
   一堆個別可探索的小物件、蓋掉 NVDA 自己的 MathCAT 報讀。要完全比照 MathCAT 官方 demo
   （[MathCATDemo](https://github.com/daisy/MathCATDemo)）的作法整組關掉
   （`enableMenu/enableEnrichment/enableSpeech/enableBraille/enableExplorer/enableComplexity`
   全部 false + `renderActions` 清空，而且要在 `startup.ready()` 裡再清一次，因為 combined
   component 是在 config 套用「之後」才載入這些擴充、會把 renderActions 蓋回去）。
4. **`role="application"` 能讓方向鍵確實傳給網頁**（不會被 NVDA 瀏覽模式攔截），但會讓
   NVDA 那個區域整個進入應用程式模式，蓋掉第 3 點辛苦換來的「整句自動唸出來」效果——
   兩個效果衝突、只能二選一，目前做法是動態切換：平常拿掉這個屬性，按「🧭 導覽模式」
   進入導覽時才加上去，離開導覽時拿掉。

## 授權

本 repo 程式碼沒有特別授權聲明（比照 [vi-tools](https://github.com/hurthuang/vi-tools) 現況）。
MathCAT 本身是 MIT 授權，見 [daisy/MathCAT](https://github.com/daisy/MathCAT)。
