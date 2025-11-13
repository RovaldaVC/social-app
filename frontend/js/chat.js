// chat.js

const WS_URL = "ws://127.0.0.1:8082";
let username = localStorage.getItem("username");
let token = localStorage.getItem("token");

if (!username) {
    alert("Please login first.");
    window.location.href = "login.html";
}

const socket = new WebSocket(WS_URL);

const messagesDiv = document.getElementById("chat-messages");
const messageInput = document.getElementById("message-input");
const sendBtn = document.getElementById("send-btn");

function appendMessage(msgData, isOwn=false) {
    const msgEl = document.createElement("div");
    msgEl.classList.add("message");
    msgEl.classList.add(isOwn ? "own" : "other");
    
    const content = document.createElement("div");
    content.textContent = msgData.content;
    msgEl.appendChild(content);

    const timestamp = document.createElement("div");
    timestamp.classList.add("timestamp");
    timestamp.textContent = `${msgData.username} | ${msgData.timestamp}`;
    msgEl.appendChild(timestamp);

    messagesDiv.appendChild(msgEl);
    messagesDiv.scrollTop = messagesDiv.scrollHeight;
}

// WebSocket events
socket.addEventListener("open", () => {
    // Send username + JWT token as first message
    socket.send(JSON.stringify({ username, token }));
});

socket.addEventListener("message", (event) => {
    try {
        const msgData = JSON.parse(event.data);
        const isOwn = msgData.username === username;
        appendMessage(msgData, isOwn);
    } catch (err) {
        console.error("Failed to parse message:", err);
    }
});

function sendMessage() {
    const content = messageInput.value.trim();
    if (!content) return;

    const msg = {
        username: username,
        content: content,
        timestamp: new Date().toLocaleTimeString(),
        message_type: "UserMessage"
    };

    socket.send(JSON.stringify(msg));
    messageInput.value = "";
}

sendBtn.addEventListener("click", sendMessage);
messageInput.addEventListener("keydown", (e) => {
    if (e.key === "Enter") sendMessage();
});
