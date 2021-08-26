# DXPriceaggregator
Decentralized Price Aggregation Service

#Tree Structure

├── schema                  # Ceramic schema definition \
├── services                # Fluence Services \
│     ├── ceramic_adapter     # ceramic adapter service \
│     ├── aggregator_service  # price aggregator service \
│     ├── artifacts           # Web Assembly outputs \
│     ├── configs             # Configuration files to deploy services \
│     ├── build.sh            # Build script to create artifacts \
│     ├── ... \
│     └── deploy_service.sh   # Script to deploy artifacts into a node using fldist \
│ \
├── service_interface       # scripts to create service interface (Air and typescripts)\
│     ├── aqua_scripts        # Orchestrating scripts to de\
│     ├── air-scripts         # generated air and typescript\ 
│     └── build.interface.sh\
├── ...\
├── web\
│     └── src\
│         └── _aqua           # Here we copy aqua generated  typescript file so we can \
│                           #  invoke fluence code from Web project\
└── ...\
#Introduction

The idea is to create a Composable price aggregation service which will allow devs to easily access / create and store historical data sets in a descentralized manner 
#
Will be programming an aggregator service on fluence so can do some price processing and then 
will push processed prices into a separate price stream

1- A daemon receives new price and invokes air script
2- The air script will orchestrate the process and will 
  - Call processing method
  - Push price to ceramic stream


#Design Assumptions
- We are implementing everything as a fat service, meaning that all processing will be done in the same webassembly module, keep in mind that for a prod version would be far better to granularize functions in smaller modules which could be run in separate nodes 

-Assuming that ceramic CLI is installed in Running node, another approach would be to implement a native client in Rust but would need to implement DS-JWS to sign and process messages  

-Stream granularity
There is room for enhacement in deciding how many tickers we should put per stream 

-Stream Authentication
In order to get write permissions to ceramic stream each node should be registered into the streams that can write or we could have a single application DID for the environment and 


#Preparing the demo

1- Start ceramic CLI with
```bash
ceramic daemon
```
2- Create schema for holding price bars and take note for schemaId

```bash
ceramic create tile --content ' {"$schema": "http://json-schema.org/draft-07/schema#","title": "AggregatedPrice","type": "object", "properties": {"ticker": { "type": "string" },"open": { "type": "number" },"high": { "type": "number" },"low": { "type": "number" },"last": { "type": "number" },"start_time" :{"type" : "number"},"duration" :{"type" : "number"}},"required": [ "ticker","duration","start_time","open","high","low","close"]}'
```

3- Get Schema CommitId
```bash
ceramic commits <<SchemaId>>
```

4- Initialize stream with some mock data
```bash
ceramic create tile --content '{"ticker":"Foo","duration":0,"start_time":0,"open":0.0,"high":0.0,"low":0.0,"close":0.0}' --schema <<schemaId>
```

#License
This project is licensed under the MIT license.
