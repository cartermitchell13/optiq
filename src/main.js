import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const statusEl = document.getElementById("status-text");
const statusBox = document.getElementById("status");
const apiKeyInput = document.getElementById("api-key");
const saveBtn = document.getElementById("save-btn");
const saveStatus = document.getElementById("save-status");
const historyDiv = document.getElementById("history");
const optimizedPre = document.getElementById("optimized");
const copyBtn = document.getElementById("copy-btn");
const keyConfigured = document.getElementById("key-configured");
const keyForm = document.getElementById("key-form");
const deleteBtn = document.getElementById("delete-btn");

const settingsDiv = document.getElementById("settings");

function restoreLastOptimization() {
  const saved = localStorage.getItem("last_optimized");
  if (saved) {
    optimizedPre.textContent = saved;
    historyDiv.classList.remove("hidden");
  }
}

function showConfigured() {
  keyConfigured.classList.remove("hidden");
  keyForm.classList.add("hidden");
  settingsDiv.classList.add("has-key");
  statusBox.classList.add("ready");
}

function showForm() {
  keyConfigured.classList.add("hidden");
  keyForm.classList.remove("hidden");
  settingsDiv.classList.remove("has-key");
  statusBox.classList.remove("ready");
  apiKeyInput.value = "";
  apiKeyInput.focus();
}

async function loadApiKey() {
  try {
    const key = await invoke("get_api_key");
    if (key) {
      showConfigured();
    } else {
      showForm();
    }
  } catch {
    showForm();
  }
}

saveBtn.addEventListener("click", async () => {
  const key = apiKeyInput.value.trim();
  if (!key) return;
  try {
    await invoke("set_api_key", { key });
    showConfigured();
  } catch (e) {
    saveStatus.textContent = "Error: " + e;
  }
});

deleteBtn.addEventListener("click", async () => {
  try {
    await invoke("delete_api_key");
    showForm();
  } catch {}
});

copyBtn.addEventListener("click", async () => {
  const text = optimizedPre.textContent;
  if (!text) return;
  try {
    await navigator.clipboard.writeText(text);
    copyBtn.classList.add("copied");
    copyBtn.querySelector("span").textContent = "Copied!";
    setTimeout(() => {
      copyBtn.classList.remove("copied");
      copyBtn.querySelector("span").textContent = "Copy";
    }, 2000);
  } catch {}
});

listen("optimize-status", (event) => {
  const { status, original, optimized, error } = event.payload;

  if (status === "started") {
    statusBox.className = "optimizing";
    statusEl.textContent = "Optimizing...";
  } else if (status === "done") {
    statusBox.className = "done";
    statusEl.textContent = "Done! Clipboard updated.";
    historyDiv.classList.remove("hidden");
    optimizedPre.textContent = optimized || "";
    localStorage.setItem("last_optimized", optimized || "");
    copyBtn.classList.remove("copied");
    copyBtn.querySelector("span").textContent = "Copy";
    setTimeout(() => {
      statusBox.className = "";
      statusEl.textContent = "Ready";
    }, 3000);
  } else if (status === "error") {
    statusBox.className = "";
    statusEl.textContent = "Error: " + (error || "Unknown");
  }
});

loadApiKey();
restoreLastOptimization();
