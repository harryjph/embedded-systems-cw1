import BinsList from "../Components/Bins/BinsList.js";
import { useEffect, useState } from "react";
import Layout from "../Components/Layout/Layout";
import { apiGet, apiPost } from "../API";

import { useNavigate } from "react-router-dom";

function UnclaimedBinsPage() {
  const history = useNavigate();

  function ClaimBin(id) {
    apiPost("/bins/" + id + "/claim").then(() => {
      history("/unowned-bins");
    });
  }
  const [isLoading, setIsloading] = useState(true);
  const [loadedBins, setLoadedBins] = useState([]);

  useEffect(() => {
    setIsloading(true);
    apiGet("/bins/unowned")
      .then((response) => {
        return response.json();
      })
      .then((data) => {
        const bins = [];
        for (const key in data) {
          const bin = {
            ...data[key],
          };
          bins.push(bin);
        }
        setIsloading(false);
        setLoadedBins(bins);
      });
  }, []);

  if (isLoading) {
    return (
      <section>
        <p>Loading...</p>
      </section>
    );
  }

  return (
    <div>
      <Layout />
      <BinsList PostRequest={ClaimBin} Text={"Claim"} AllData={loadedBins} showPropertiesButton={false} />
    </div>
  );
}

export default UnclaimedBinsPage;
