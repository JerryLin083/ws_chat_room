import { createSignal } from "solid-js";

import "./signup.css";

function SignUp() {
  const [account, setAccount] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [isSubmit, setIsSubmit] = createSignal(false);
  const [error, setError] = createSignal("");

  const handleSubmit = async (e) => {
    e.preventDefault();
    setIsSubmit(true);

    try {
      let res = await fetch("api/signup", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          account: account(),
          password: password(),
        }),
      });

      if (!res.ok) {
        let error = await res.json();
        setError(error.message);

        throw new Error(error.message);
      }

      window.location.assign("/");
    } catch (err) {
      console.error(err);
    } finally {
      setIsSubmit(false);
    }
  };

  return (
    <div class="signup-container">
      <h3>Sign up</h3>
      <form class="signup-form" onSubmit={handleSubmit}>
        <div>
          <label for="account">Account: </label>
          <input
            id="account"
            placeholder="account"
            value={account()}
            onInput={(e) => setAccount(e.currentTarget.value)}
          />
        </div>

        <div>
          <label for="password-1">Password: </label>
          <input
            id="password-1"
            type="password"
            placeholder="password"
            value={password()}
            onInput={(e) => setPassword(e.currentTarget.value)}
          />
        </div>

        {error() ? <p>{error()}</p> : null}
        {isSubmit() ? (
          <button type="button" class="disabled-button" disabled>
            waiting...
          </button>
        ) : (
          <button>Sign up</button>
        )}
      </form>
    </div>
  );
}

export default SignUp;
