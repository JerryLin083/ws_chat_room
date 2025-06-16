import { createWS } from "@solid-primitives/websocket";
import { useSearchParams } from "@solidjs/router";
import { createSignal, onCleanup, onMount } from "solid-js";

import "./room.css";
import MessageCard from "../components/message_card";
import send from "../assets/icons-send.svg";

function Room() {
  const [searchParams, _setSearchParams] = useSearchParams();
  const [messages, setMessages] = createSignal([]);

  const [message, setMessage] = createSignal("");

  let ws;

  onMount(async () => {
    let room_id = searchParams.room_id;

    if (!room_id) {
      window.location.replace("/not_found");
    }

    //websocket connection
    ws = createWS(`/api/join_room?room_id=${room_id}`);
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
      alert("Room dose not exist");
      window.location.replace("/");
    };
  });

  onCleanup(() => {
    if (ws) {
      ws.close();
    }
  });

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

  return (
    <div>
      <ul class="messages-container">
        <For each={messages()}>
          {(message, i) => <MessageCard message={message} index={i} />}
        </For>
      </ul>
      <div class="message-bar-container">
        <input
          value={message()}
          onInput={(e) => setMessage(e.currentTarget.value)}
          onKeyDown={handleKeydown}
        />
        <p>
          <img src={send} alt="send" width="24" height="24" />
        </p>
      </div>
    </div>
  );
}

export default Room;
