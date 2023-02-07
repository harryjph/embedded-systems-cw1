import BinsList from "../Components/Bins/BinsList.js";
import { useEffect, useState } from "react";
import Layout from "../Components/Layout/Layout";
import { apiGet, apiPostForm } from "../API";

import Map from "../Components/Map/Map";
import { useNavigate } from "react-router-dom";

function AllOfMyBinsPage() {
  const history = useNavigate();

  function ReleaseFunction(variables) {
    apiPostForm("/bins/" + variables.ID + "/release").then(() => {
      history("/unowned-bins");
    });
  }
  const [isLoading, setIsloading] = useState(true);
  const [loadedBins, setLoadedBins] = useState([]);

  useEffect(() => {
    setIsloading(true);
    apiGet("/bins")
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
      <div className="flex p-5">
        <div className="flex-auto">
          <Map AllData={loadedBins} />
        </div>

        <div className="flex-auto">
          <BinsList PostRequest={ReleaseFunction} Text={"Release"} AllData={loadedBins} />
        </div>
      </div>
    </div>
  );
}

export default AllOfMyBinsPage;
