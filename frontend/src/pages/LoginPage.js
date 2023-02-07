import { useRef } from "react";
import { apiPostForm } from "../API";

import DropDown from "../Components/BasicComponents/DropDown";

function LoginPage() {
  const emailRef = useRef();
  const passwordRef = useRef();

  function loginRequest(endpoint) {
    const loginForm = {
      email: emailRef.current.value,
      password: passwordRef.current.value,
    };

    return apiPostForm("/user/" + endpoint, loginForm, true);
  }

  function login() {
    loginRequest("login", true)
      .then(async (response) => {
        if (response.ok) {
          window.location = "/app/my-bins";
        } else {
          alert("Login failed: " + (await response.text()));
        }
      })
      .catch((e) => alert("Error accessing API: " + e.toString()));
  }

  function register() {
    loginRequest("register")
      .then(async (response) => {
        if (response.ok) {
          window.location = "/app/my-bins";
        } else {
          alert("Registration failed: " + (await response.text()));
        }
      })
      .catch((e) => alert("Error accessing API: " + e.toString()));
  }

  return <div className="flex items-center h-screen w-screen justify-center bg-slate-300">
    <div className="w-full max-w-xs">
      <div className="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
        <div className="mb-4">
          <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="email">
            Username
          </label>
          <input className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" 
                  type="email"
                  required
                  id="email"
                  ref={emailRef}
          />
        </div>

          <div className="mb-6">
            <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="password">
              Password
            </label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              type="password"
              required
              id="password"
              ref={passwordRef}
            />
            <p className="text-red-500 text-xs italic mt-1">Please choose a password.</p>
          </div>

          <div className="flex items-center justify-between">
            <button
            className="p-3 inline-flex items-center px-4 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
            onClick={login}
            >
              Login
            </button>
            <button
              className="text-[#3b5998] bg-white focus:ring-4 focus:outline-none font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center mr-2 mb-2"
              onClick={register}
            >
              Register
            </button>
          </div>
        </div>
      </div>
    </div>;
}

export default LoginPage;
