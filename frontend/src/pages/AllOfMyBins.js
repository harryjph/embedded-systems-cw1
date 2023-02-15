import BinsList from "../Components/Bins/BinsList.js";
import { useEffect, useState } from "react";
import Layout from "../Components/Layout/Layout";
import { apiGet, apiPost } from "../API";

function AllOfMyBinsPage() {
  function ReleaseBin(id) {
    apiPost("/bins/" + id + "/release").then(() => {
      window.location.reload();
    });
  }
  const [loadedBins, setLoadedBins] = useState([]);

  function updateBins() {
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
  }

  useEffect(() => {
    updateBins();
    const timer = setInterval(updateBins, 1000);
    return () => {
      clearTimeout(timer);
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
