import { useState } from "react";

import { close, menu } from "../assets";

const Navbar = () => {
  const [toggle, setToggle] = useState(false);

  return (
    <nav className="w-full flex py-6 justify-between items-center navbar">
      <img src="https://flowbite.com/docs/images/logo.svg" alt="Logo" className="w-[60px] h-auto mt-5" />

      <span className="font-poppins font-semibold text-[32px] text-white ml-5 mt-6">IoT101</span>

      <ul className="list-none sm:flex hidden justify-end items-center flex-1">
        <li key="login" className="font-poppins font-normal cursor-pointer text-[16px] text-white mr-10 mt-5">
          <a href="/app" className="py-4 px-6 font-poppins font-medium text-[18px] text-primary bg-blue-gradient rounded-[10px] outline-none">
            Login
          </a>
        </li>
      </ul>

      <div className="sm:hidden flex flex-1 justify-end items-center">
        <img
          src={toggle ? close : menu}
          alt="menu"
          className="w-[28px] h-[28px] object-contain"
          onClick={() => setToggle(!toggle)}
        />

        <div
          className={`${
            !toggle ? "hidden" : "flex"
          } p-6 bg-black-gradient absolute top-20 right-0 mx-4 my-2 min-w-[140px] rounded-xl sidebar`}
        >
          <ul className="list-none flex justify-end items-start flex-1 flex-col">
            <li key="login" className="font-poppins font-medium cursor-pointer text-[16px] text-dimWhite mb-0">
              <a href={`/app`}>Login</a>
            </li>
          </ul>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
