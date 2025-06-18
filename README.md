  <h1>🗨️ WS Chat Room</h1>
  <p><strong>WS Chat Room</strong> is a real-time multi-room chat application built using <strong>Rust (Axum)</strong> on the backend and <strong>SolidJS</strong> on the frontend. It leverages <strong>WebSocket</strong> for bi-directional communication and uses <strong>PostgreSQL</strong> as the database layer.</p>

  <h2>📚 Overview</h2>
  <p>This project demonstrates an end-to-end implementation of a session-based, real-time messaging platform. It supports user authentication, room-based messaging, automatic cleanup of idle rooms, and persistent message storage.</p>

  <h2>🧱 Database Schema</h2>
  <ul>
    <li><strong>accounts</strong> – Stores account credentials
      <ul><li>id, account, password, created_at</li></ul>
    </li>
    <li><strong>users</strong> – Stores user profile information linked to an account
      <ul><li>id, account_id, username</li></ul>
    </li>
    <li><strong>rooms</strong> – Represents chat rooms
      <ul><li>id (UUID), room_name, created_at, closed_at</li></ul>
    </li>
    <li><strong>messages</strong> – Stores chat messages
      <ul><li>id, room_id, user_id, content, sent_at</li></ul>
    </li>
  </ul>

  <h2>🧩 Backend Architecture (Axum)</h2>
  <ul>
    <li><strong>Session Management</strong>
      <br>Manages user sessions using <code>SessionManager</code> and a secure <code>session_id</code> stored in HTTP cookies.
    </li>
    <li><strong>Room Management</strong>
      <br>Maintains active rooms and connected users via <code>RoomManager</code>.
      <br>Rooms are cleaned up automatically after an idle timeout.
    </li>
    <li><strong>WebSocket Communication</strong>
      <br>Upon joining or creating a room, a WebSocket connection is established.
      <br>Messages are exchanged using a custom <code>StreamCommand</code> JSON protocol.
      <br>Internally uses <code>mpsc</code> channels for room commands and <code>broadcast</code> for message dissemination.
    </li>
  </ul>

  <h2>🧩 Frontend Architecture (SolidJS)</h2>
  <ul>
    <li><strong>Routing</strong>
      <ul>
        <li><code>/</code> – Home page (authentication required)</li>
        <li><code>/login</code> – Login page</li>
        <li><code>/signup</code> – Registration page</li>
        <li><code>/room/</code> – Join a specific chat room (WebSocket connection)</li>
        <li><code>/new-room</code> – Create and join a new chat room</li>
        <li><code>*</code> – Not Found page</li>
      </ul>
    </li>
    <li><strong>Interaction Flow</strong>
      <ol>
        <li>User signs up or logs in, receiving a <code>session_id</code> via cookie.</li>
        <li>Home page fetches available rooms from <code>/api/rooms</code>.</li>
        <li>User joins or creates a room, establishing a WebSocket connection.</li>
        <li>Messages are exchanged in real time using JSON commands.</li>
        <li>User may leave the room or log out at any time.</li>
      </ol>
    </li>
  </ul>

  <h2>🔄 Application Flow</h2>
  <ol>
    <li>Users authenticate via <code>/signup</code> or <code>/login</code>, receiving a <code>session_id</code> cookie.</li>
    <li>Home page lists rooms via <code>/api/rooms</code>.</li>
    <li>WebSocket is connected via <code>/api/join_room</code> or <code>/api/create_room</code>.</li>
    <li>Each client receives room channels and subscribes to broadcasts.</li>
    <li>Communication is conducted using <code>StreamCommand</code> JSON messages.</li>
    <li>Idle rooms are automatically removed by <code>RoomManager</code>.</li>
  </ol>

  <h2>✅ Features</h2>
  <ul>
    <li>Secure user authentication with session tracking</li>
    <li>Real-time chat using WebSocket</li>
    <li>Room creation and joining</li>
    <li>Message broadcasting</li>
    <li>Automatic cleanup of idle chat rooms</li>
    <li>Persistent message storage in PostgreSQL</li>
  </ul>

  <h2>🛠 Prerequisites</h2>
  <ul>
    <li>PostgreSQL (with pre-created schema)</li>
    <li>Rust (latest stable version recommended)</li>
    <li>Node.js + npm (for building SolidJS frontend)</li>
  </ul>

  <h2>🚀 Getting Started</h2>
  <ol>
    <li>Clone the repository</li>
    <li>Configure <code>.env</code> and set up PostgreSQL schema</li>
    <li>Run the backend server (Axum)</li>
    <li>Build and serve the frontend (SolidJS)</li>
    <li>Open the app in browser and start chatting</li>
  </ol>

  
