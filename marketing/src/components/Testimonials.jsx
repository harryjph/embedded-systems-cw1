import styles from "../style";
import FeedbackCard from "./FeedbackCard";
import {people01, people02, people03} from "../assets/index.js";

const feedback = [
  {
    id: "feedback-1",
    content:
      "This product would help make tracking biowaste much more cost friendly on the NHS.",
    name: "Herman Ali-Gallagher",
    title: "NHS Hospital Administrator",
    img: people01,
  },
  {
    id: "feedback-2",
    content:
      "Adding smart-bins into our infrastructure would be a valuable tool in identifying areas of budget waste and furthering our efforts to reduce fuel costs.",
    name: "Steve Necchi-Phillips",
    title: "City Councilman",
    img: people02,
  },
  {
    id: "feedback-3",
    content:
      "I believe in an innovative IoT product that also has a focus on security is a way for the future.",
    name: "Kenn Stott",
    title: "CEO Entrepreneur",
    img: people03,
  },
];

const Testimonials = () => (
  <section
    id="clients"
    className={`${styles.paddingY} ${styles.flexCenter} flex-col relative `}
  >
    <div className="absolute z-[0] w-[60%] h-[60%] -right-[50%] rounded-full blue__gradient bottom-40" />

    <div className="w-full flex justify-between items-center md:flex-row flex-col sm:mb-16 mb-6 relative z-[1]">
      <h2 className={styles.heading2}>
        What our<span className="text-gradient"> Early Buyers</span> are{" "}
        <br className="sm:block hidden" /> saying about us:
      </h2>
    </div>

    <div className="flex flex-wrap sm:justify-start justify-center w-full feedback-container relative z-[1]">
      {feedback.map((card) => (
        <FeedbackCard key={card.id} {...card} />
      ))}
    </div>
  </section>
);

export default Testimonials;
