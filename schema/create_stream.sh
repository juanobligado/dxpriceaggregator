#!/usr/bin/env bash
cat << EOF 
' {"\$schema": "http://json-schema.org/draft-07/schema#","title": "AggregatedPrice","type": "object", 
"properties": {
     "ticker": { "type": "string" },
	 "open": { "type": "string" },
	 "high": { "type": "string" },
	 "low": { "type": "string" },
	 "last": { "type": "string" },
	 "start_time" :{"type" : "number"},
	 "duration" :{"type" : "number"}
   },
   "required": [ 
	"ticker",
     "duration",
     "start_time",
	 "open","high","low","close
   ]
 }'
EOF
ceramic create tile --content ' {"$schema": "http://json-schema.org/draft-07/schema#","title": "AggregatedPrice","type": "object", "properties": {"ticker": { "type": "string" },"open": { "type": "number" },"high": { "type": "number" },"low": { "type": "number" },"last": { "type": "number" },"start_time" :{"type" : "number"},"duration" :{"type" : "number"}},"required": [ "ticker","duration","start_time","open","high","low","close"]}'
ceramic commits kjzl6cwe1jw146n0caf5dvq69kl047846okmq4sp8j9by26cprzr7romko5w5ky

//
commitId : k3y52l7qbv1frxr7mfe3qapm4k2dctvdpriqplu43goycy7h6fb29j8v4iztwnoxs

ceramic create tile --content '{"ticker":"ETH","duration":10000,"start_time":1,"open":0,"high":0.0,"low":9999.99,"close":0.0,"last_updated":0}' --schema k3y52l7qbv1frxr7mfe3qapm4k2dctvdpriqplu43goycy7h6fb29j8v4iztwnoxs


StreamId: kjzl6cwe1jw14b4p64saz1980b0gl3l1c98ag1pslez0shjrhv1mr44257hv9m0

ceramic show kjzl6cwe1jw14b4p64saz1980b0gl3l1c98ag1pslez0shjrhv1mr44257hv9m0