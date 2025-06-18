import { A } from "@solidjs/router";
import "./not_found.css";

function NotFound() {
  return (
    <div class="not-found-container">
      <h1>404</h1>
      <h2>Page Not Found</h2>
      <A href="/" class="home-button">
        GO BACK
      </A>
    </div>
  );
}

export default NotFound;
