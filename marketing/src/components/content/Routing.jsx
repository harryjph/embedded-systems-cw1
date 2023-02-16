import { map_routed } from "../../assets/index.js";
import styles, { layout } from "../../style.js";
import { useState } from "react";

const Routing = () => {
  return (
    <section className={layout.section}>
      <div className={layout.sectionInfo}>
        <h2 className={styles.heading2}>
          We are <span className="text-gradient">routing</span> for you.
        </h2>
        <p className={`${styles.paragraph} max-w-[470px] mt-5`}>
          Take the hassle out of waste management with our IoT startup! Create
          your personalized network of smart bins, and let us help you keep it
          neat and tidy. Our cutting-edge routing algorithm dynamically
          prioritizes which bins need emptying first, ensuring efficient and
          streamlined waste disposal. Choose us and enjoy hassle-free waste
          management - we help you keep it clean!
        </p>
      </div>

      <div
        className={layout.sectionImg}
      >
        <div className="company-block">
          <img src={map_routed} />
        </div>
      </div>
    </section>
  );
};

export default Routing;
