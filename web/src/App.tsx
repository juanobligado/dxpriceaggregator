import React, { useEffect, useState } from "react";
import "./App.scss";

import { createClient, FluenceClient } from "@fluencelabs/fluence";
//import { get_last_price } from "./_aqua/run_get_stream_price";
import {ping , read_last_price , process_data } from "./_aqua/aggregator_service";

type Unpromise<T extends Promise<any>> = T extends Promise<infer U> ? U : never;

type Result = Unpromise<ReturnType<typeof read_last_price>>;

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
  const [streamId, setStreamId] = useState<string>("kjzl6cwe1jw147d3hz5mmf6997l8hjo6cbvwjn1t7210hjot8i371s9lzggidlz");
  const [node, setNode] = useState<string>("12D3KooWCSLKALdhXgQzDBDqBx3WuA8sYRY7rATeo27bXrB1HB83");
  const [priceServiceId, setPriceServiceId] = useState<string>("d4635b5a-6329-4e31-a0ee-b106441a358a"); 
  const [ticker, setTicker] = useState<string>("ETH"); 
  const [price, setPrice] = useState<number>(0.0);
  const [result, setResult] = useState<Result | null>(null);
  let failed = false;
  let nodeAddr = "/ip4/127.0.0.1/tcp/9999/ws/p2p/12D3KooWCSLKALdhXgQzDBDqBx3WuA8sYRY7rATeo27bXrB1HB83";
  useEffect(() => {
    createClient(nodeAddr)
      .then(setClient)
      .catch((err) => console.log("Client initialization failed", err));
  }, [client]);

  const isConnected = client !== null;

  const doGetPrice = async () => {
    if (client === null) {
      return;
    }
    try {
      const res = await read_last_price(
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


  const doSendPrice = async () => {
    if (client === null) {
      return;
    }
    try {
      let now = Date.prototype.getTime();
      console.log('Calling Process Data : ',node,priceServiceId,streamId,ticker,price ,Date.prototype.getTime());
      const res = await process_data(
        client!,
        node,
        priceServiceId,
        streamId,
        ticker,
        price ,
        now       
      );
      console.log("Retrieved result: ", res);
    
      setResult(res);
      failed = false;
    } catch (err) {
      failed = true;
      console.log(err);
    }
  };

  const doPing = async ()=>{
    if (client === null) {
      return;
    }
    try {
      const res = await ping(
        client!,
        node,
        priceServiceId        
      );
      var str = "Ping Result: " + res;
      alert(str);
      console.log();
      failed = false;
    } catch (err) {
      failed = true;
      console.log(err);
    }
  }
  return (
    <div className="App">
      <header>
         <div>Add Logo Here</div>
         <img src="./dx_icon.png" className="logo" alt="logo" /> 
      </header>

      <div className="content">
        <h1>Status: {isConnected ? "Connected" : "Disconnected"}</h1>
        <p>Simple app to test Aggregator price</p>
        <div>
          <h2>Update Last Price to Ceramic</h2>
          <TextInput text={"node"} value={node} setValue={setNode} />
          <TextInput text={"priceServiceId"} value={priceServiceId} setValue={setPriceServiceId} />
          <TextInput text={"Ceramic Stream Id"} value={streamId} setValue={setStreamId} />
          <TextInput text={"Ticker"} value={ticker} setValue={setTicker} />
          <NumberInput text="price" value={price} setValue={setPrice} />

          <div className="row">
          <button className="btn btn-hello" onClick={() => doSendPrice()}>
              Update  price
            </button>
            <button className="btn btn-hello" onClick={() => doGetPrice()}>
              Read Last price
            </button>

            <button className="btn btn-hello" onClick={() => doPing()}>
              Ping
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
