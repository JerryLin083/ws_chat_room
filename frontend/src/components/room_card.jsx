import "./room_card.css";
import join from "../assets/icons-join.svg";
import { useNavigate } from "@solidjs/router";

function RoomCard(props) {
  const navigate = useNavigate();

  return (
    <li key={props.room.room_id}>
      <div class="room-card-container">
        <h2>{props.room.room_name}</h2>
        <button onClick={() => navigate(`/room?room_id=${props.room.room_id}`)}>
          <img src={join} alt="join" width="20" height="20" />
          &nbsp
          <span>Join</span>
        </button>
      </div>
    </li>
  );
}

export default RoomCard;
