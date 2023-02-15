import React, {useEffect, useState} from "react";
import { Animate } from "react-move";

function AnimatedProgressProvider(props) {
  const [isAnimated, setIsAnimated] = useState(false);

  useEffect(() => {
    if (props.repeat) {
      const timer = window.setInterval(() => {
        setIsAnimated(!isAnimated);
      }, props.duration * 1000);
      return () => clearInterval(timer);
    } else {
      setIsAnimated(!isAnimated);
    }
  }, [props.repeat, props.duration, isAnimated]);

  return (
    <Animate
      start={() => ({
        value: props.valueStart
      })}
      update={() => ({
        value: [
          isAnimated ? props.valueEnd : props.valueStart
        ],
        timing: {
          duration: props.duration * 1000,
          ease: props.easingFunction
        }
      })}
    >
      {({ value }) => props.children(value)}
    </Animate>
  );
}

export default AnimatedProgressProvider;
