# Optiq

A desktop app that optimizes text prompts for AI coding agents. Press a hotkey from anywhere on your system, and Optiq rewrites your clipboard text into a clear, structured prompt — then auto-pastes it right back where you were typing.

![Optiq](app-icon.png)

## Why I Built This

I use AI coding agents every day, and I kept running into the same problem: I knew what I wanted, but couldn't quite articulate it in a way that got the best result. The idea was clear in my head, but turning it into a well-structured prompt was the bottleneck. I'd write something vague, get a mediocre output, and spend more time fixing the result than if I'd just written a better prompt upfront.

Optiq adds an AI layer between your raw thought and the coding agent. You dump whatever's in your head — messy, incomplete, half-formed — and Optiq shapes it into a precise, well-structured prompt. The coding agent gets better input, and you get better code.

## How It Works

1. Copy a rough, unstructured prompt to your clipboard
2. Press the hotkey from any application (**Ctrl+Alt+P** on Windows/Linux, **Cmd+Alt+P** on macOS)
3. Optiq sends the text to OpenAI and rewrites it into an optimized prompt
4. The optimized version is pasted into your active field automatically

That's it. No context switching, no extra steps.

## Features

- **System-wide hotkey** — works from any app, not just when Optiq is focused
- **Automatic paste** — optimized text is pasted directly into your active field
- **Clipboard restoration** — if the API call fails, your original clipboard content is restored
- **Secure API key storage** — encrypted with Windows DPAPI on Windows, macOS Keychain on Mac
- **Optimization history** — the last optimized prompt is persisted across restarts
- **Dark, minimal UI** — compact window with color-coded status indicators

## Tech Stack

| Layer | Technology |
|---|---|
| Desktop framework | [Tauri v2](https://tauri.app/) |
| Backend | Rust |
| Frontend | Vanilla JavaScript + Vite |
| AI | OpenAI Chat Completions API |
| Keyboard simulation | [enigo](https://github.com/enigo-rs/enigo) |

## Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/)
- An [OpenAI API key](https://platform.openai.com/api-keys)

## Getting Started

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

On first launch, enter your OpenAI API key in the settings panel.

## Building

```bash
npm run tauri build
```

The installer will be generated in `src-tauri/target/release/bundle/`.

## Usage

1. Launch Optiq — it runs in the background as a small window
2. Enter your OpenAI API key if you haven't already
3. In any application, copy some text and press the hotkey (**Ctrl+Alt+P** / **Cmd+Alt+P**)
4. The optimized prompt replaces your clipboard contents and is auto-pasted

## License

[MIT](LICENSE)
