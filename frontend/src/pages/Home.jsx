import { createSignal, onCleanup, onMount } from "solid-js";

import "./home.css";
import RoomCard from "../components/room_card";
import search from "../assets/icons-search.svg";
import leave from "../assets/icons-leave.svg";
import setting from "../assets/icons-setting.svg";
import create from "../assets/icons-create.svg";
import { useNavigate } from "@solidjs/router";

function Home() {
  const navigate = useNavigate();

  const [rooms, setRooms] = createSignal([]);
  const [searchRooms, setSearchRooms] = createSignal(null);
  const [toggle, setToggle] = createSignal(false);

  let settingRef;
  let toggoleButton;

  onMount(async () => {
    document.addEventListener("click", handleClose);
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

  onCleanup(() => {
    document.removeEventListener("click", handleClose);
  });

  const handleInput = (e) => {
    let value = e.currentTarget.value;

    let new_rooms = rooms().filter((room) => room.room_name.includes(value));

    setSearchRooms(new_rooms);
  };

  const handleClose = (e) => {
    if (
      settingRef &&
      !settingRef.contains(e.target) &&
      !toggoleButton.contains(e.target)
    ) {
      setToggle(false);
    }
  };

  const handleCreate = async (_) => {
    navigate("/new_room");
  };

  const handleLogout = async (_) => {
    try {
      let res = await fetch("/api/logout");

      if (!res.ok) {
        throw new Error("failed to logout");
      }
    } catch (err) {
      console.error(err);
    } finally {
      window.location.replace("/login");
    }
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

      {toggle() && (
        <div class="setting-container" ref={settingRef}>
          <ul>
            <li class="create-container" onClick={handleCreate}>
              <img src={create} alt="create" width="24" height="24" />
              <p>Create</p>
            </li>
            <li class="logout-container" onClick={handleLogout}>
              <img src={leave} alt="logout" width="24" height="24" />
              <p>Logout</p>
            </li>
          </ul>
        </div>
      )}

      {!toggle() && (
        <div
          class="setting-icon"
          ref={toggoleButton}
          onClick={() => setToggle(true)}
        >
          <img src={setting} alt="setting" width="32" height="32" />
        </div>
      )}
    </div>
  );
}

export default Home;
