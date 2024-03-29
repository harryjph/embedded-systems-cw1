import { apiGet } from "../../API";
import { useEffect, useState } from "react";
import { Dropdown } from "flowbite-react";
import { AiOutlineUser } from "react-icons/ai";
import { BiLogOut } from "react-icons/bi";

function MainNavigation() {
  const [email, setEmail] = useState("");
  const [avatarDropdown, setAvatarDropdown] = useState(false);

  useEffect(() => {
    apiGet("/user")
      .then((r) => r.json())
      .then((user) => setEmail(user.email));
  });

  function logout() {
    apiGet("/user/logout", true).then((_) => {
      window.location = "/app";
    });
  }

  const page = window.location.pathname.split("/").pop();

  const tabClasses =
    "block py-2 pl-3 pr-4 text-gray-700 text-xl rounded hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:p-0 md:dark:hover:text-white dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700";
  const myBinsClasses = page === "my-bins" ? tabClasses + " font-bold underline" : tabClasses;
  const unownedBinsClasses = page === "unowned-bins" ? tabClasses + " font-bold underline" : tabClasses;

  return (
    <div>
      <nav className="bg-white border-gray-200 px-2 sm:px-4 py-2.5 rounded dark:bg-gray-900">
        <div className="container flex flex-wrap items-center justify-between mx-auto">
          <a href="/app/my-bins" className="flex items-center">
            <img src="https://flowbite.com/docs/images/logo.svg" className="h-6 mr-3 sm:h-10" alt="Logo" />
            <span className="self-center text-xl font-semibold whitespace-nowrap dark:text-white">IoT101</span>
          </a>

          <div className="flex md:order-2">
            <Dropdown onClick={() => setAvatarDropdown(!avatarDropdown)} label={<AiOutlineUser />}>
              {avatarDropdown && (
                <div className="">
                  <div className="px-4 py-3 text-sm text-gray-900 dark:text-white">
                    <div className="font-medium truncate">{email}</div>
                    <Dropdown.Item
                      className="items-center justify-center block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
                      onClick={logout}
                    >
                      Sign out <BiLogOut className="space-x-1" />{" "}
                    </Dropdown.Item>
                  </div>
                </div>
              )}
            </Dropdown>
          </div>

          <div className="items-center justify-between hidden w-full md:flex md:w-auto md:order-1" id="navbar-cta">
            <ul className="flex flex-col p-4 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 md:mt-0 md:text-sm md:font-medium md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
              <li>
                <a href="/app/my-bins" className={myBinsClasses}>
                  My Bins
                </a>
              </li>
              <li>
                <a href="/app/unowned-bins" className={unownedBinsClasses}>
                  Unowned Bins
                </a>
              </li>
            </ul>
          </div>
        </div>
      </nav>
    </div>
  );
}

export default MainNavigation;
