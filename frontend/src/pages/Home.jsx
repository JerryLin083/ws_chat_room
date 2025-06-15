import { createSignal, onMount } from "solid-js";

import "./home.css";
import RoomCard from "../components/room_card";
import search from "../assets/icons-search.svg";

function Home() {
  const [rooms, setRooms] = createSignal(dummy_data);
  const [searchRooms, setSearchRooms] = createSignal(null);

  // onMount(async () => {
  //   try {
  //     let roomsRes = await fetch("api/rooms");

  //     if (!roomsRes.ok) {
  //       if (roomsRes.status == "401") {
  //         window.location.replace("/login");
  //       } else {
  //         let error = await roomsRes.json();
  //         throw new Error(error.message);
  //       }
  //     }

  //     let result = await roomsRes.json();
  //     setRooms(result.data);
  //   } catch (err) {
  //     console.error(err);
  //   }
  // });

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

const dummy_data = [
  {
    room_id: "6dfaf670-2bda-4c0d-b851-50a98cf3d359",
    room_name: "hello_world",
  },
  {
    room_id: "57cecf06-a7af-40ed-9e65-2c475e5ec4e4",
    room_name: "chatchat",
  },
  {
    room_id: "fc4985bc-3baf-4c1f-abcf-1ade5b82b4df",
    room_name: "sport",
  },
  {
    room_id: "f1b939c2-fcb0-4543-9976-5550cbe9bf84",
    room_name: "bird",
  },
  {
    room_id: "3f80e6d8-5464-4648-be53-860c372164ee",
    room_name: "fish",
  },
  {
    room_id: "0bdabc36-5fbc-4f85-b98a-0674b24807d1",
    room_name: "chiken",
  },
];
