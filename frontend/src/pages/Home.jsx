import { createSignal, onMount } from "solid-js";

import "./home.css";
import RoomCard from "../components/room_card";
import search from "../assets/icons-search.svg";

function Home() {
  const [rooms, setRooms] = createSignal([]);
  const [searchRooms, setSearchRooms] = createSignal(null);

  onMount(async () => {
    try {
      let roomsRes = await fetch("api/rooms");

      if (!roomsRes.ok) {
        if (roomsRes.status == "401") {
          window.location.replace("/login");
        } else {
          let error = await roomsRes.json();
          throw new Error(error.message);
        }
      }

      let result = await roomsRes.json();
      setRooms(result.data);
    } catch (err) {
      console.error(err);
    }
  });

  const handleInput = (e) => {
    let value = e.currentTarget.value;

    let new_rooms = rooms().filter((room) => room.room_name.includes(value));

    setSearchRooms(new_rooms);
  };

  return (
    <div>
      <div class="search-bar-container">
        <input id="search-bar" placeholder="Search" onInput={handleInput} />
        <p>
          <img src={search} alt="seach" width="24" height="24" />
        </p>
      </div>
      <div class="rooms-container">
        <For each={searchRooms() ? searchRooms() : rooms()}>
          {(room, _) => <RoomCard room={room} />}
        </For>
      </div>
    </div>
  );
}

export default Home;
