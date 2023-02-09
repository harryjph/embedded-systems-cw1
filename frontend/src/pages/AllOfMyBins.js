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
  const [loadedBins, setLoadedBins] = useState([]);

  useEffect(() => {
    const timer = setInterval(() => {
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
          setLoadedBins(bins);
        });
    }, 1000);
    return () => {
      clearInterval(timer);
    }
  }, []);

  return (
    <div>
      <Layout />
      <div className="flex p-5">
        <div className="container flex flex-wrap items-center justify-center">
          <button>Hello!</button>
        </div>
        <div className="flex-auto insert-0">
          <BinsList PostRequest={ReleaseFunction} Text={"Release"} AllData={loadedBins} showPropertiesButton={true} />
        </div>
      </div>
    </div>
  );
}

export default AllOfMyBinsPage;
