import React, { useEffect, useState } from "react";
import { CircularProgressbar } from "react-circular-progressbar";
import "react-circular-progressbar/dist/styles.css";

function CircularProgressBar({ upper_value }) {
  const [percentage, setPercentage] = useState(0);

  useEffect(() => {
    let timer = setTimeout(() => {
      if (percentage < upper_value) {
        setPercentage(percentage + 1);
      }
    }, 5);
    return () => {
      clearTimeout(timer);
    };
  }, [percentage, upper_value]);

  return <CircularProgressbar value={percentage} text={`${percentage}%`} />;
}
export default CircularProgressBar;
