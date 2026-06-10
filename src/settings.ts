import { invoke } from '@tauri-apps/api/core';

const ollamaUrl = document.getElementById('ollama-url') as HTMLInputElement;
const modelSelection = document.getElementById('model-selection') as HTMLInputElement;
const saveBtn = document.getElementById('save-btn') as HTMLButtonElement;

async function loadSettings() {
    try {
        const url = await invoke<string | null>('get_preference', { key: 'ollama_url' });
        if (url) ollamaUrl.value = url;
        
        const model = await invoke<string | null>('get_preference', { key: 'model' });
        if (model) modelSelection.value = model;
    } catch (e) {
        console.error("Failed to load settings", e);
    }
}

async function saveSettings() {
    try {
        await invoke('set_preference', { key: 'ollama_url', value: ollamaUrl.value });
        await invoke('set_preference', { key: 'model', value: modelSelection.value });
        alert('Settings saved!');
    } catch (e) {
        console.error("Failed to save settings", e);
        alert('Failed to save settings.');
    }
}

saveBtn.addEventListener('click', saveSettings);
loadSettings();
