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
          alert("Registration failed: " + await response.text());
        }
      })
      .catch(e => alert("Error accessing API: " + e.toString()));
  }

  return <div className="flex items-center h-screen w-screen justify-center bg-slate-300	">
    <div className="flex flex-col">

      <div className="grid gap-1 grid-cols-2">
        <label className="flex items-center font-bold pr-2" htmlFor="email">Email:</label>
        <input
          type="email"
          required
          id="email"
          ref={emailRef}
          />
        <label className="flex items-center font-bold pr-2" htmlFor="password">Password:</label>
        <input
          type="password"
          required
          id="password"
          ref={passwordRef}
          />
      </div>
        <div className="flex justify-center items-center">
          <button className="p-3 hover:bg-sky-700 rounded-sm hover:font-bold" onClick={login}>Login</button>
          <button className="p-3 hover:bg-sky-700 rounded-sm hover:font-bold" onClick={register}>Register</button>
        </div>
    </div>

  </div>;
}

export default LoginPage;
