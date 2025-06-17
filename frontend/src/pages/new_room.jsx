import { createWS } from "@solid-primitives/websocket";
import { createSignal, onCleanup, onMount } from "solid-js";

import "./room.css";
import "./new_room.css";
import MessageCard from "../components/message_card";
import send from "../assets/icons-send.svg";
import leave from "../assets/icons-leave.svg";

function Room() {
  const [messages, setMessages] = createSignal([]);
  const [message, setMessage] = createSignal("");

  const [showForm, setShowForm] = createSignal(true);
  const [roomName, setRoomName] = createSignal("");

  let ws;

  onMount(async () => {
    try {
      let authRes = await fetch("/api/auth");

      if (!authRes.ok) {
        let result = await authRes.json();
        throw new Error(result.message);
      }
    } catch (err) {
      console.error(err);

      window.location.replace("/login");
    }
  });

  onCleanup(() => {
    if (ws) {
      ws.close();
    }
  });

  const handleSubmit = (e) => {
    e.preventDefault();

    if (!roomName()) return;

    //websocket connection
    ws = createWS(`/api/create_room?room_name=${roomName()}`);
    ws.onopen = () => {
      let command = {
        method: "Join",
        message: "",
        sender: "",
        is_self: false,
      };
      ws.send(JSON.stringify(command));
    };
    ws.onmessage = (e) => {
      let message = JSON.parse(e.data);
      setMessages((prev) => {
        return [...prev, message];
      });
    };
    ws.onclose = () => {
      console.log("connection closed");
    };
    ws.onerror = (err) => {
      console.error(err);
      alert("Failed to create room");
      window.location.replace("/");
    };

    setShowForm(false);
  };

  const handleKeydown = (e) => {
    if (e.key == "Enter" && ws) {
      let command = {
        method: "Send",
        message: e.currentTarget.value,
        sender: "",
        is_self: false,
      };

      ws.send(JSON.stringify(command));
      setMessage("");
    }
  };

  const handleSend = (_) => {
    if (message()) {
      let command = {
        method: "Send",
        message: message(),
        sender: "",
        is_self: false,
      };

      ws.send(JSON.stringify(command));
      setMessage("");
    }
  };

  return (
    <div>
      <ul class="messages-container">
        <For each={messages()}>
          {(message, i) => <MessageCard message={message} index={i} />}
        </For>
      </ul>
      <div class="message-bar-container">
        <div onClick={() => window.location.replace("/")}>
          <img src={leave} alt="leave" width="24" height="24" />
        </div>
        <input
          id="message"
          placeholder="message"
          value={message()}
          onInput={(e) => setMessage(e.currentTarget.value)}
          onKeyDown={handleKeydown}
        />
        <div onClick={handleSend}>
          <img src={send} alt="send" width="24" height="24" />
        </div>
      </div>

      {showForm() && (
        <div class="form-container">
          <form onSubmit={handleSubmit}>
            <label for="room_name">Room Name</label>
            <input
              id="room_name"
              value={roomName()}
              onInput={(e) => setRoomName(e.currentTarget.value)}
            />
            <p></p>
            <button>Create</button>
          </form>
        </div>
      )}
    </div>
  );
}

export default Room;
