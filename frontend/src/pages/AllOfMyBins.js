import BinsList from "../Components/Bins/BinsList.js";
import { useEffect, useState } from "react";
import Layout from "../Components/Layout/Layout";
import { apiGet, apiPost } from "../API";

import { useNavigate } from "react-router-dom";

function AllOfMyBinsPage() {
  const history = useNavigate();

  function ReleaseBin(id) {
    apiPost("/bins/" + id + "/release").then(() => {
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
    };
  }, []);

  return (
    <div>
      <Layout />
      <BinsList PostRequest={ReleaseBin} Text={"Release"} AllData={loadedBins} showPropertiesButton={true} />
    </div>
  );
}

export default AllOfMyBinsPage;
