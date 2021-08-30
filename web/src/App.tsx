import React, { useEffect, useState } from "react";
import "./App.scss";
import { createClient, FluenceClient } from "@fluencelabs/fluence";
import { ping , read_last_price , process_data } from "./_aqua/aggregator_service";

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
  const [streamId, setStreamId] = useState<string>("kjzl6cwe1jw149pkg27j3xszjkxksp6jjwpqr7k84jlinxk4k7gyefnjaikezme");
  const [node, setNode] = useState<string>("12D3KooWCSLKALdhXgQzDBDqBx3WuA8sYRY7rATeo27bXrB1HB83");
  const [priceServiceId, setPriceServiceId] = useState<string>("4af1cb27-9d24-4741-9104-000f898e3188"); 
  const [price, setPrice] = useState<number>(0.0);
  const [result, setResult] = useState<Result | null>(null);
  let failed = false;
  let nodeAddr = "/ip4/127.0.0.1/tcp/9999/ws/p2p/" + node;
  let havePrice = false;

  const isConnected = client !== null;

  const doConnect = () =>{
    nodeAddr = "/ip4/127.0.0.1/tcp/9999/ws/p2p/" + node;
    console.log('Connecting to Fluence...',nodeAddr);
    if(isConnected){
      setClient(null);
    }
    
    createClient(nodeAddr)
        .then(setClient)
        .catch((err) => console.log("Client initialization failed", err));
  }

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
      havePrice = true;
      setResult(res);
      failed = false;
    } catch (err) {
      havePrice = false;
      failed = true;
      console.log(err);
    }
  };


  const doSendPrice = async () => {
    if (client === null) {
      return;
    }
    try {
      let now = new Date().getTime();
      const res = await process_data(
        client!,
        node,
        priceServiceId,
        streamId,
        price ,
        now       
      );
      console.log("Retrieved result: ", res);
    
      setResult(res);
      havePrice = true;
      failed = false;
    } catch (err) {
      havePrice = false;
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
         <h1>DX Aggregator DEMO</h1>
         <img src="./dx_icon.png" className="logo" alt="logo" /> 
      </header>

      <div className="content">

        <TextInput text={"Fluence Network Node"} value={node} setValue={setNode} />

        <TextInput text={"Service Id:"} value={priceServiceId} setValue={setPriceServiceId} />
         
        <h1 className={isConnected ? "success"  : "error"} >Status: {isConnected ? "Connected" : "Disconnected"}</h1>

        <div>
        <div className="row">
            <button className="btn btn-wide " onClick={() => doPing()}>
              Check 
            </button>
            <button className="btn btn-wide btn-connect" onClick={()=>doConnect()}>Connect</button>


          </div>          

        </div>

        <div>
          <h2>Update Last Price to Ceramic</h2>

          <TextInput text={"Ceramic Stream Id:"} value={streamId} setValue={setStreamId} />

          <NumberInput text="price" value={price} setValue={setPrice} />

          <div className="row">
          <button className="btn btn-wide" onClick={() => doSendPrice()}>
              Update  price
            </button>
            <button className="btn btn-wide" onClick={() => doGetPrice()}>
              Read Last price
            </button>

          </div>          

        </div>
        
        { havePrice  ?
          
          <h2>Bar price</h2> : ''
        }
        {
          <p className="success">The price is: {result?.ticker} | O: {result?.open} | H: {result?.high} | L: {result?.low} | C: {result?.close} </p> 
        }
      </div>
    </div>
  );
}

export default App;
