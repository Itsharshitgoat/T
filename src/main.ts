import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { marked } from 'marked';
import './style.css';

// UI Elements
const chatBubble = document.getElementById('chat-bubble') as HTMLDivElement;
const messagesContainer = document.getElementById('messages') as HTMLDivElement;
const promptInput = document.getElementById('prompt-input') as HTMLInputElement;
const sendBtn = document.getElementById('send-btn') as HTMLButtonElement;

// State
let isVisible = false;
let idleTimer: number | null = null;
let currentResponseDiv: HTMLDivElement | null = null;
let currentResponseText: string = "";

// Physics State
let targetX = 0;
let targetY = 0;
let currentX = 0;
let currentY = 0;
let velocityX = 0;
let velocityY = 0;
const TENSION = 0.15;
const FRICTION = 0.8;

// Reset idle timer
function resetIdleTimer() {
    if (idleTimer) clearTimeout(idleTimer);
    idleTimer = window.setTimeout(() => {
        hideApp();
    }, 15000);
}

// Ensure it starts hidden
setTimeout(() => {
    if (!isVisible) invoke('hide_window');
}, 500);

function showApp(cursorX: number, cursorY: number) {
    if (!isVisible) {
        isVisible = true;
        
        // Initial placement directly at cursor to avoid flying from 0,0
        currentX = cursorX;
        currentY = cursorY;
        invoke('set_window_position', { x: cursorX, y: cursorY });
        
        invoke('show_window').then(() => {
            setTimeout(() => {
                promptInput.focus();
            }, 50);
        });
        
        chatBubble.classList.add('visible');
        
        // Start animation loop if not running
        requestAnimationFrame(updatePhysics);
    }
    resetIdleTimer();
}

function hideApp() {
    if (isVisible) {
        isVisible = false;
        chatBubble.classList.remove('visible');
        setTimeout(() => {
            invoke('hide_window');
        }, 200); // Wait for CSS fade/scale out
    }
}

// Event Listeners
document.addEventListener('mousemove', resetIdleTimer);
document.addEventListener('keydown', resetIdleTimer);

// Tauri Events
listen<{x: number, y: number}>('shake-detected', (event) => {
    showApp(event.payload.x, event.payload.y);
});

listen<{x: number, y: number}>('cursor-moved', (event) => {
    if (!isVisible) return;
    
    // Zone logic - calculate target window position
    // For simplicity in v1, we position bottom-right of cursor
    // In a real multi-monitor setup we'd get monitor bounds.
    let offsetX = 10;
    let offsetY = 10;
    
    // If we had screen bounds, we would calculate zone 1-4 here
    // For now, assume a fixed offset and spring towards it
    targetX = event.payload.x + offsetX;
    targetY = event.payload.y + offsetY;
    
    // Set border radius based on zone (simplified)
    chatBubble.style.borderRadius = '0 28px 28px 28px';
});

let lastAppliedX = -1;
let lastAppliedY = -1;

// Spring Physics Loop
function updatePhysics() {
    if (!isVisible) return;

    const dx = targetX - currentX;
    const dy = targetY - currentY;
    
    velocityX += dx * TENSION;
    velocityY += dy * TENSION;
    
    velocityX *= FRICTION;
    velocityY *= FRICTION;
    
    currentX += velocityX;
    currentY += velocityY;
    
    const newX = Math.round(currentX);
    const newY = Math.round(currentY);
    
    if (newX !== lastAppliedX || newY !== lastAppliedY) {
        invoke('set_window_position', { x: newX, y: newY });
        lastAppliedX = newX;
        lastAppliedY = newY;
    }
    
    requestAnimationFrame(updatePhysics);
}

// Chat UI Logic
function addMessage(text: string, sender: 'user' | 'assistant'): HTMLDivElement {
    const div = document.createElement('div');
    div.className = `message ${sender}`;
    div.innerText = text;
    messagesContainer.appendChild(div);
    messagesContainer.scrollTop = messagesContainer.scrollHeight;
    return div;
}

async function handleSend() {
    const text = promptInput.value.trim();
    if (!text) return;

    promptInput.value = '';
    addMessage(text, 'user');
    
    currentResponseText = "";
    currentResponseDiv = addMessage('', 'assistant');
    
    try {
        await invoke('send_prompt', { prompt: text });
    } catch (e) {
        console.error(e);
        if (currentResponseDiv) {
            currentResponseDiv.innerText = "Error: " + String(e);
            currentResponseDiv = null;
        }
    }
}

sendBtn.addEventListener('click', handleSend);
promptInput.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') handleSend();
});

// Stream handlers
listen<string>('ai-chunk', async (event) => {
    if (currentResponseDiv) {
        currentResponseText += event.payload;
        currentResponseDiv.innerHTML = await marked.parse(currentResponseText);
        // Scroll to bottom
        messagesContainer.scrollTop = messagesContainer.scrollHeight;
    }
});

listen('ai-done', () => {
    currentResponseDiv = null;
    // Save to memory (simplified)
    // invoke('add_conversation', ...);
});
