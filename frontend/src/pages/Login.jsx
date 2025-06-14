import { createSignal } from "solid-js";
import { A } from "@solidjs/router";
import "./login.css";

function Login() {
  const [account, setAccount] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [logging, setLogging] = createSignal(false);
  const [error, setError] = createSignal("");

  const handleSubmit = async (e) => {
    e.preventDefault();

    setLogging(true);
    try {
      let authRes = await fetch("api/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ account: account(), password: password() }),
      });

      if (!authRes.ok) {
        let error = await authRes.json();
        setError(error.message);
        throw new Error(error.message);
      }
      window.location.assign("/");
    } catch (err) {
      console.error(err);
    } finally {
      setLogging(false);
    }
  };

  return (
    <div class="login-container">
      <h3>Log in</h3>
      <form class="login-form" onSubmit={handleSubmit}>
        <div>
          <label for="account">Account: </label>
          <input
            id="account"
            placeholder="account"
            value={account()}
            onInput={(e) => {
              setAccount(e.currentTarget.value);
            }}
          />
        </div>

        <div>
          <label for="password">Password: </label>
          <input
            id="password"
            type="password"
            placeholder="password"
            value={password()}
            onInput={(e) => {
              setPassword(e.currentTarget.value);
            }}
          />
        </div>
        {error() ? <p>{error()}</p> : null}
        {logging() ? (
          <button type="button" class="disabled-button" disabled>
            waiting...
          </button>
        ) : (
          <button>Sign up</button>
        )}
      </form>
      <A href="/signup">Sign up now</A>
    </div>
  );
}

export default Login;
