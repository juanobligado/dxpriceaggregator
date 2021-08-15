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
ceramic commits kjzl6cwe1jw14bioibhblaztbeuucww2ckqnfvv2bjhxgf7cib6xvd6zwqedrkj

//
commitId : k3y52l7qbv1frypwua9n6fi6oh5nbvtw0pfhioykwi2nhws48y7lcv1trcrq9w1z4

ceramic create tile --content '{"ticker":"ETH","duration":10000,"start_time":1,"open":3000,"high":3204.43,"low":2904.43,"close":3145.32}' --schema k3y52l7qbv1frypwua9n6fi6oh5nbvtw0pfhioykwi2nhws48y7lcv1trcrq9w1z4


StreamId: kjzl6cwe1jw14b4p64saz1980b0gl3l1c98ag1pslez0shjrhv1mr44257hv9m0