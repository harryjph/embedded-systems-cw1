import styles from "./style";
import Navbar from "./components/Navbar";
import Hero from "./components/Hero";
import Security from "./components/content/Security";
import ControlBins from "./components/content/ControlBins";
import Routing from "./components/content/Routing";
import Testimonials from "./components/content/Testimonials";

const App = () => (
  <div className="bg-primary w-full overflow-hidden">
    <div className={`${styles.paddingX} ${styles.flexCenter}`}>
      <div className={`${styles.boxWidth}`}>
        <Navbar />
      </div>
    </div>

    <div className={`bg-primary ${styles.flexStart}`}>
      <div className={`${styles.boxWidth}`}>
        <Hero />
      </div>
    </div>

    <div className={`bg-primary ${styles.paddingX} ${styles.flexCenter}`}>
      <div className={`${styles.boxWidth}`}>
        <Security />
        <ControlBins />
        <Routing />
        <Testimonials />
      </div>
    </div>
  </div>
);

export default App;
