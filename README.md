# dxpriceaggregator
Decentralized Price Aggregation Service

#Introduction

The idea is to create a Composable price aggregation service which will allow devs to easily access / create and store historical data sets in a descentralized manner 
#
Will be programming an aggregator service on fluence so can do some price processing and then 
will push processed prices into a separate price stream

1- A daemon receives new price and invokes air script
2- The air script will orchestrate the process and will 
  - Call processing method
  - Push price to ceramic stream

