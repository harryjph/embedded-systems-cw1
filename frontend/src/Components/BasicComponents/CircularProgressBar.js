import { CircularProgressbar } from "react-circular-progressbar";
import "react-circular-progressbar/dist/styles.css";
import { useEffect, useState } from "react";

const ProgressProvider = ({ valueStart, valueEnd, children }) => {
  const [value, setValue] = useState(valueStart);
  useEffect(() => {
    setValue(valueEnd);
  }, [valueEnd]);

  return children(value);
};

function CircularProgressBar({ value }) {
  return (
    <ProgressProvider valueStart={0} valueEnd={value}>
      {(value) => <CircularProgressbar value={value} text={`${value}%`} />}
    </ProgressProvider>
  );
}
export default CircularProgressBar;
