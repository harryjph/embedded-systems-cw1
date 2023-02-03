import { Link } from "react-router-dom";
import classes from "./MainNavigation.module.css";

function MainNavigation() {
  return (
    <header className={classes.header}>
      <div className={classes.logo}>IoT101</div>
      <nav>
        <ul>
          <li>
            <Link to="/my-bins">
              My Bins
            </Link>
          </li>
          <li>
            <Link to="/unowned-bins">Unowned Bins</Link>
          </li>
        </ul>
      </nav>
    </header>
  );
}

export default MainNavigation;
