import { Route, Router } from "@solidjs/router";

import "./App.css";
import Layout from "./components/layout";
import Home from "./pages/Home";
import Login from "./pages/Login";
import NotFound from "./pages/NotFound";
import SignUp from "./pages/signup";

function App() {
  return (
    <Router root={Layout}>
      <Route path="/" component={Home} />
      <Route path="login" component={Login} />
      <Route path="signup" component={SignUp} />
      <Route path="*404" component={NotFound} />
    </Router>
  );
}

export default App;
