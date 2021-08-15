import React, { useEffect, useState } from "react";
//import logo from "./.svg";
import "./App.scss";

import { createClient, FluenceClient } from "@fluencelabs/fluence";
import { krasnodar } from "@fluencelabs/fluence-network-environment";
import { get_last_price } from "./_aqua/run_get_stream_price";

const relayNode = krasnodar[0];

type Unpromise<T extends Promise<any>> = T extends Promise<infer U> ? U : never;

type Result = Unpromise<ReturnType<typeof get_last_price>>;

const TextInput = (props: {
  text: string;
  value: string;
  setValue: React.Dispatch<React.SetStateAction<string>>;
}) => {
  return (
    <div className="row">
      <label className="label bold">{props.text}</label>
      <input
        className="input"
        type="text"
        onChange={(e) => props.setValue(e.target.value)}
        value={props.value}
        required={true}
      />
    </div>
  );
};

const NumberInput = (props: {
  text: string;
  value: number;
  setValue: React.Dispatch<React.SetStateAction<number>>;
}) => {
  return (
    <div className="row">
      <label className="label bold">{props.text}</label>
      <input
        className="input"
        type="number"
        onChange={(e) => props.setValue(e.target.valueAsNumber)}
        value={props.value}
        required={true}
      />
    </div>
  );
};


function App() {
  const [client, setClient] = useState<FluenceClient | null>(null);
  const [streamId, setStreamId] = useState<string>("kjzl6cwe1jw14b4p64saz1980b0gl3l1c98ag1pslez0shjrhv1mr44257hv9m0");
  const [node, setNode] = useState<string>("12D3KooWSD5PToNiLQwKDXsu8JSysCwUt8BVUJEqCHcDe7P5h45e");
  const [priceServiceId, setPriceServiceId] = useState<string>("61fcbff6-8982-418f-badf-afcd8d9885f7");  
  const [price, setPrice] = useState<number>(
    0.0
  );
  const [result, setResult] = useState<Result | null>(null);
  let failed = false;
  useEffect(() => {
    createClient(relayNode.multiaddr)
      .then(setClient)
      .catch((err) => console.log("Client initialization failed", err));
  }, [client]);

  const isConnected = client !== null;

  const doGetPrice = async () => {
    if (client === null) {
      return;
    }
    try {
      const res = await get_last_price(
        client!,
        node,
        priceServiceId,
        streamId        
      );
      console.log("Retrieved result: ", res);
      setResult(res);
      failed = false;
    } catch (err) {
      failed = true;
      console.log(err);
    }
  };

  return (
    <div className="App">
      <header>
        { <div>Add Logo Here</div>
        /* <img src={logo} className="logo" alt="logo" /> */}
      </header>

      <div className="content">
        <h1>Status: {isConnected ? "Connected" : "Disconnected"}</h1>
        <p>Simple app to test Aggregator price</p>
        <div>
          <h2>Update Last Price to Ceramic</h2>
          <TextInput text={"node"} value={node} setValue={setNode} />
          <TextInput text={"priceServiceId"} value={priceServiceId} setValue={setPriceServiceId} />
          <TextInput text={"Ceramic Stream Id"} value={streamId} setValue={setStreamId} />
          <NumberInput text="price" value={price} setValue={setPrice} />
          <div className="row">
            <button className="btn btn-hello" onClick={() => doGetPrice()}>
              Push price
            </button>
          </div>
        </div>
        <h2>Bar price</h2>
        { result?.ticker !== "" &&    (
          <p className="success">The price is: {result?.ticker} | O: {result?.open} | H: {result?.high} | L: {result?.low} | C: {result?.close} </p>
        )}
      </div>
    </div>
  );
}

export default App;
