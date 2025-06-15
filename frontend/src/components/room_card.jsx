import "./room_card.css";
import join from "../assets/icons-join.svg";

function RoomCard(props) {
  return (
    <li key={props.room.room_id}>
      <div class="room-card-container">
        {/* <p>{props.room.room_id}</p> */}
        <h2>{props.room.room_name}</h2>
        <button>
          <img src={join} alt="join" width="20" height="20" />
          &nbsp
          <span>Join</span>
        </button>
      </div>
    </li>
  );
}

export default RoomCard;
