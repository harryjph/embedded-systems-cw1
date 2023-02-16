import { before, after } from "../assets";
import styles, { layout } from "../style";
import { useState } from "react";

const CardDeal = () => {
  const [open, setOpen] = useState(false);
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
        onMouseOver={() => setOpen(true)}
        onMouseOut={() => setOpen(false)}
      >
        <div className="company-block">
          <img src={open ? after : before} />
        </div>
      </div>
    </section>
  );
};

export default CardDeal;
