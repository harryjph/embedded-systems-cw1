import { bin } from "../assets";
import styles, { layout } from "../style";

const Billing = () => (
  <section id="product" className={layout.sectionReverse}>
    <div className={layout.sectionImgReverse}>
      <img src={bin} alt="bin" className="w-[100%] h-[90%] relative z-[5]" />

      {/* gradient start */}
      <div className="absolute z-[3] -left-1/2 top-0 w-[50%] h-[50%] rounded-full white__gradient" />
      <div className="absolute z-[0] w-[50%] h-[50%] -left-1/2 bottom-0 rounded-full pink__gradient" />
      {/* gradient end */}
    </div>

    <div className={layout.sectionInfo}>
      <h2 className={styles.heading2}>
        Easily control your <br className="sm:block hidden" />
        <span className="text-gradient">Bins</span>{" "}
      </h2>
      <p className={`${styles.paragraph} max-w-[470px] mt-5`}>
        Our advanced routing algorithm employs real-time data analytics to
        determine which bins require immediate attention, optimizing waste
        collection and disposal. It is also designed to handle a large number of
        bins, without any loss of efficiency or performance. Choose us and
        experience the benefits of a high-performance waste management system
        that can adapt to your evolving needs.
      </p>
    </div>
  </section>
);

export default Billing;
