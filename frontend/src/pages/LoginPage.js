import {useRef} from "react";
import {apiPostForm} from '../API';

function LoginPage() {
  const emailRef = useRef();
  const passwordRef = useRef();

  function loginRequest(endpoint) {
    const loginForm = {
      'email': emailRef.current.value,
      'password': passwordRef.current.value,
    };

    return apiPostForm("/user/" + endpoint, loginForm, true);
  }

  function login() {
    loginRequest("login", true)
      .then(async response => {
        if (response.ok) {
          window.location = "/my-bins";
        } else {
          alert("Login failed: " + await response.text());
        }
      })
      .catch(e => alert("Error accessing API: " + e.toString()));
  }

  function register() {
    loginRequest("register")
      .then(async response => {
        if (response.ok) {
          window.location = "/my-bins";
        } else {
          alert("Login failed: " + await response.text());
        }
      })
      .catch(e => alert("Error accessing API: " + e.toString()));
  }

  return <div>
    <label htmlFor="email">Email</label>
    <input
      type="email"
      required
      id="email"
      ref={emailRef}
    />
    <label htmlFor="password">Password</label>
    <input
      type="password"
      required
      id="password"
      ref={passwordRef}
    />
    <button onClick={login}>Login</button>
    <button onClick={register}>Register</button>
  </div>;
}

export default LoginPage;
