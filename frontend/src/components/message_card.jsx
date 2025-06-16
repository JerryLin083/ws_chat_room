import "./message_card.css";

function MessageCard(props) {
  if (props.message.sender == "System") {
    return (
      <li key={props.index} class="system-message-container">
        {props.message.message}
      </li>
    );
  }

  let component;

  if (props.message.is_self) {
    component = (
      <li key={props.index} class="self-message-container">
        {props.message.message}
      </li>
    );
  }
  if (!props.message.is_self) {
    component = (
      <li key={props.index} class="others-message-container">
        {props.message.sender}: {props.message.message}
      </li>
    );
  }

  return component;
}

export default MessageCard;
