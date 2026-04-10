# Prompt Optimizer (Tauri App) — Build Plan

## Goal

Create a lightweight desktop app that:

1. Takes user-written input (raw prompt)
2. Sends it to an LLM to optimize it for coding agents
3. Returns an improved prompt
4. Lets the user quickly paste/send it

\---

## Core MVP Flow

1. User types prompt anywhere
2. Copies prompt (`Cmd/Ctrl + C`)
3. Presses global hotkey (`Cmd/Ctrl + Shift + O`)
4. App:

   * Reads clipboard
   * Sends to LLM
   * Receives optimized prompt
   * Replaces clipboard
5. User pastes optimized version (`Cmd/Ctrl + V`) and sends

\---

## Tech Stack

* Desktop Framework: Tauri
* Frontend: Minimal (or none for MVP)
* Backend: Rust (Tauri core)
* LLM API: OpenAI (or compatible)
* Clipboard: Tauri clipboard API
* Hotkeys: Tauri global shortcut plugin

\---

## Project Setup

### 1\. Create Tauri App

```bash
npm create tauri-app@latest
```

Choose:

* Framework: Vanilla / minimal (no need for React initially)
* Package manager: npm

```bash
cd prompt-optimizer
npm install
npm run tauri dev
```

\---

### 2\. Install Required Plugins

```bash
npm install @tauri-apps/plugin-clipboard-manager
npm install @tauri-apps/plugin-global-shortcut
```

Update `tauri.conf.json`:

```json
{
  "plugins": {
    "globalShortcut": {},
    "clipboardManager": {}
  }
}
```

\---

## Core Features Implementation

### 3\. Register Global Hotkey

**Goal:** Trigger optimization from anywhere

Rust (Tauri backend):

```rust
use tauri\_plugin\_global\_shortcut::GlobalShortcutExt;

app.handle().plugin(
    tauri\_plugin\_global\_shortcut::Builder::new()
        .with\_shortcut("CmdOrCtrl+Shift+O", move || {
            // trigger optimization
        })
        .build()
)?;
```

\---

### 4\. Read Clipboard

```rust
use tauri\_plugin\_clipboard\_manager::ClipboardExt;

let text = app.clipboard().read\_text().unwrap\_or\_default();
```

\---

### 5\. Send to LLM (OpenAI)

Use `reqwest` in Rust:

```rust
let response = reqwest::Client::new()
    .post("https://api.openai.com/v1/chat/completions")
    .bearer\_auth(API\_KEY)
    .json(\&payload)
    .send()
    .await?;
```

\---

### 6\. Prompt Optimization System Prompt

```text
You are an expert prompt engineer for AI coding agents.

Rewrite the user's input into a clear, structured, and optimized prompt.

Rules:
- Be concise but specific
- Add relevant constraints
- Clarify intent
- Specify expected output format when useful
- Do NOT add unnecessary verbosity
- Preserve the user's goal exactly

Return ONLY the improved prompt.
```

\---

### 7\. Example Request Payload

```json
{
  "model": "gpt-4.1",
  "messages": \[
    {
      "role": "system",
      "content": "SYSTEM PROMPT HERE"
    },
    {
      "role": "user",
      "content": "USER CLIPBOARD TEXT"
    }
  ]
}
```

\---

### 8\. Replace Clipboard with Optimized Prompt

```rust
app.clipboard().write\_text(optimized\_text)?;
```

\---

## Optional UX (Post-MVP)

### Minimal UI Window (optional)

* Show:

  * Original prompt
  * Optimized prompt
  * Buttons:

    * Copy
    * Regenerate
    * Accept

\---

## Error Handling

* If clipboard is empty → do nothing
* If API fails → keep original clipboard
* Add basic logging

\---

## Config

Store API key securely:

* `.env` file (dev)
* OS keychain (later)

\---

## File Structure

```
src/
  main.rs
  optimizer.rs
  openai.rs

src-tauri/
  tauri.conf.json

.env
```

\---

## Milestones

### Phase 1 (1–2 hours)

* Tauri app runs
* Hotkey works

### Phase 2 (2–3 hours)

* Clipboard read/write works

### Phase 3 (2–4 hours)

* OpenAI request wired
* Prompt returns optimized text

### Phase 4 (1 hour)

* Full flow works end-to-end

\---

## Future Improvements

* Auto-send after replace (simulate Enter key)
* Modes (refactor / debug / feature)
* Context awareness (files, repo)
* Inline overlay UI
* Local model support

\---

## Done Criteria

* Press hotkey → clipboard prompt becomes optimized version in <2s
* Works across any app
* No crashes

