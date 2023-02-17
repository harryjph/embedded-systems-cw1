import styles, { layout } from "../../style.js";

const Security = () => (
  <section id="features" className={layout.section}>
    <div className={layout.sectionInfo}>
      <h2 className={styles.heading2}>
        <span className="text-gradient">Cutting-Edge</span> Security
      </h2>
      <p className={`${styles.paragraph} max-w-[470px] mt-5`}>
        Rest easy knowing that your data is always protected with our
        state-of-the-art security measures. Our network features TLS encryption
        to safeguard your information, while your password is secured by 10,000
        PBKDF2 iterations and salting. We take your privacy seriously and are
        committed to keeping your data safe and secure.
      </p>
    </div>

    <div className={`${layout.sectionImg} flex-col`}>
      {
        <iframe
          width="560"
          height="315"
          src="https://www.youtube.com/embed/sA55PezhjhE"
          title="YouTube video player"
          style={{border: 0}}
          allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
          allowFullScreen
        ></iframe>
      }
    </div>
  </section>
);

export default Security;
