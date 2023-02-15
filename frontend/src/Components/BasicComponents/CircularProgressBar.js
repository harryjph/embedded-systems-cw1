import {buildStyles, CircularProgressbar} from "react-circular-progressbar";
import "react-circular-progressbar/dist/styles.css";
import AnimatedProgressProvider from "./AnimatedProgressProvider";
import { easeQuadInOut } from "d3-ease";

function CircularProgressBar({ value }) {
  return <AnimatedProgressProvider
    valueStart={0}
    valueEnd={value}
    duration={1.4}
    easingFunction={easeQuadInOut}
    repeat
  >
    {value => {
      const roundedValue = Math.round(value);
      return (
        <CircularProgressbar
          value={value}
          text={`${roundedValue}%`}
          styles={buildStyles({ pathTransition: "none" })}
        />
      );
    }}
  </AnimatedProgressProvider>;
}
export default CircularProgressBar;
