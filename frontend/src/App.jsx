import { Route, Router } from "@solidjs/router";

import "./App.css";
import Layout from "./components/layout";
import Home from "./pages/home";
import Login from "./pages/login";
import NotFound from "./pages/not_found";
import SignUp from "./pages/signup";
import Room from "./pages/room";
import NewRoom from "./pages/new_room";

function App() {
  return (
    <Router root={Layout}>
      <Route path="/" component={Home} />
      <Route path="login" component={Login} />
      <Route path="signup" component={SignUp} />
      <Route path="room" component={Room} />
      <Route path="new_room" component={NewRoom} />
      <Route path="*404" component={NotFound} />
    </Router>
  );
}

export default App;
