# T - A Little Friend, Always Nearby

<div align="center">
  <img alt="Tauri" src="https://img.shields.io/badge/Tauri-v2-24C8DB?style=flat-square&logo=tauri&logoColor=white" />
  <img alt="Rust" src="https://img.shields.io/badge/Rust-Backend-000000?style=flat-square&logo=rust&logoColor=white" />
  <img alt="TypeScript" src="https://img.shields.io/badge/TypeScript-Vanilla-3178C6?style=flat-square&logo=typescript&logoColor=white" />
  <img alt="Ollama" src="https://img.shields.io/badge/Ollama-Cloud-white?style=flat-square&logo=ollama&logoColor=black" />
</div>

<br/>

**T** is a tiny, lightweight AI companion for macOS that lives quietly inside your computer. It bypasses the traditional heavy UI of modern AI apps—no dock icons, no tabs, no startup screens. Just shake your mouse, and T appears right next to your cursor, ready to help.

---

## ✨ Features

- **Cursor Summoning:** Shake your mouse, and T instantly teleports to your cursor's location.
- **Invisible Footprint:** Runs completely in the background. No dock icon, no menubar clutter.
- **Spring Physics:** T elegantly follows your cursor around the screen using a custom-built spring physics engine.
- **Cloud AI Integration:** Connects directly to Ollama Cloud API (defaulting to `gemma3:4b`), providing incredibly fast, intelligent responses.
- **Contextual Memory:** Uses a local SQLite database to quietly remember your goals, projects, and context so you don't have to repeat yourself.
- **Beautiful Markdown:** Renders code blocks, lists, and formatting natively in real-time.

---

## 📖 The Philosophy

Technology shouldn't always demand your attention. Sometimes it should quietly wait until you need it. 

T was built not as another application or another window competing for space on your screen, but as a small presence that appears beside your cursor when you ask. The cursor is where your attention lives—where you point, click, create, and learn. So that's where T stays.

Over time, T learns the things you choose to share: your projects, your ideas, and the questions that keep you curious. It doesn't remember everything—just enough to understand you a little better, so every conversation begins with context instead of repetition. 

Whenever you're ready to brainstorm, debug code, or just think out loud... **shake the mouse. T will be there.**

---

## 🛠️ Tech Stack

T was meticulously engineered to be as lightweight and native as possible, entirely avoiding heavy web-wrappers like Electron.

- **Frontend:** Vanilla TypeScript, HTML, & CSS. No heavy frameworks, zero bloat.
- **Backend:** Tauri v2 & Rust. Unbelievably fast and memory efficient, utilizing native macOS window hooks.
- **Database:** `rusqlite` for persistent, offline memory storage.
- **AI:** Streaming JSON pipeline connected to cloud models.

---

## 🚀 Getting Started

To invite T into your machine, you can easily build it from source.

### Prerequisites
* macOS (Requires native macOS CoreGraphics hooks)
* [Node.js](https://nodejs.org/) & npm
* [Rust](https://rustup.rs/)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/t-companion.git
   cd t-companion
   ```

2. **Install frontend dependencies**
   ```bash
   npm install
   ```

3. **Run in Development Mode**
   ```bash
   npm run tauri dev
   ```

4. **Build for Production** (Generates `.app` and `.dmg`)
   ```bash
   npm run tauri build
   ```

---

## ⚙️ Configuration

T stores its preferences in standard macOS directories (`~/Library/Application Support/com.t.app/`).

You can customize:
* **API Key:** Provide your own Bearer token for cloud endpoints.
* **Cloud Model:** Default is `gemma3:4b`, but can be swapped to any OpenAI/Ollama compatible model.
* **Endpoint URL:** Connect to the official Ollama Cloud or host your own proxy.
