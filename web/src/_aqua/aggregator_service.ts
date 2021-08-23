/**
 *
 * This file is auto-generated. Do not edit manually: changes may be erased.
 * Generated by Aqua compiler: https://github.com/fluencelabs/aqua/. 
 * If you find any bugs, please write an issue on GitHub: https://github.com/fluencelabs/aqua/issues
 * Aqua version: 0.1.9-163
 *
 */
import { FluenceClient, PeerIdB58 } from '@fluencelabs/fluence';
import { RequestFlowBuilder } from '@fluencelabs/fluence/dist/api.unstable';
import { RequestFlow } from '@fluencelabs/fluence/dist/internal/RequestFlow';



<<<<<<< HEAD
export async function read_last_price(client: FluenceClient, node: string, aggregator_service_id: string, streamId: string, config?: {ttl?: number}): Promise<{close:number;error_msg:string;high:number;low:number;open:number;success:boolean;ticker:string}> {
    let request: RequestFlow;
    const promise = new Promise<{close:number;error_msg:string;high:number;low:number;open:number;success:boolean;ticker:string}>((resolve, reject) => {
=======
export async function read_last_price(client: FluenceClient, node: string, aggregator_service_id: string, streamId: string, config?: {ttl?: number}): Promise<{close:number;error_msg:string;high:number;last_updated:number;low:number;open:number;period:number;start_time:number;success:boolean;ticker:string}> {
    let request: RequestFlow;
    const promise = new Promise<{close:number;error_msg:string;high:number;last_updated:number;low:number;open:number;period:number;start_time:number;success:boolean;ticker:string}>((resolve, reject) => {
>>>>>>> 97cbeefef2ce1089523760caa8965f8a77cf44ad
        const r = new RequestFlowBuilder()
            .disableInjections()
            .withRawScript(
                `
(xor
 (seq
  (seq
   (seq
    (seq
     (seq
      (seq
       (seq
        (call %init_peer_id% ("getDataSrv" "-relay-") [] -relay-)
        (call %init_peer_id% ("getDataSrv" "node") [] node)
       )
       (call %init_peer_id% ("getDataSrv" "aggregator_service_id") [] aggregator_service_id)
      )
      (call %init_peer_id% ("getDataSrv" "streamId") [] streamId)
     )
     (call -relay- ("op" "noop") [])
    )
    (xor
     (seq
      (call -relay- ("op" "noop") [])
      (call node (aggregator_service_id "read_last_price") [streamId] last_price)
     )
     (seq
      (call -relay- ("op" "noop") [])
      (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 1])
     )
    )
   )
   (call -relay- ("op" "noop") [])
  )
  (xor
   (call %init_peer_id% ("callbackSrv" "response") [last_price])
   (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 2])
  )
 )
 (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 3])
)

            `,
            )
            .configHandler((h) => {
                h.on('getDataSrv', '-relay-', () => {
                    return client.relayPeerId!;
                });
                h.on('getDataSrv', 'node', () => {return node;});
h.on('getDataSrv', 'aggregator_service_id', () => {return aggregator_service_id;});
h.on('getDataSrv', 'streamId', () => {return streamId;});
                h.onEvent('callbackSrv', 'response', (args) => {
  const [res] = args;
  resolve(res);
});

                h.onEvent('errorHandlingSrv', 'error', (args) => {
                    // assuming error is the single argument
                    const [err] = args;
                    reject(err);
                });
            })
            .handleScriptError(reject)
            .handleTimeout(() => {
                reject('Request timed out for read_last_price');
            })
        if(config?.ttl) {
            r.withTTL(config.ttl)
        }
        request = r.build();
    });
    await client.initiateFlow(request!);
    return promise;
}
      


export async function ping(client: FluenceClient, node: string, aggregator_service_id: string, config?: {ttl?: number}): Promise<string> {
    let request: RequestFlow;
    const promise = new Promise<string>((resolve, reject) => {
        const r = new RequestFlowBuilder()
            .disableInjections()
            .withRawScript(
                `
(xor
 (seq
  (seq
   (seq
    (seq
     (seq
      (seq
       (call %init_peer_id% ("getDataSrv" "-relay-") [] -relay-)
       (call %init_peer_id% ("getDataSrv" "node") [] node)
      )
      (call %init_peer_id% ("getDataSrv" "aggregator_service_id") [] aggregator_service_id)
     )
     (call -relay- ("op" "noop") [])
    )
    (xor
     (seq
      (call -relay- ("op" "noop") [])
      (call node (aggregator_service_id "ping") [] ping_result)
     )
     (seq
      (call -relay- ("op" "noop") [])
      (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 1])
     )
    )
   )
   (call -relay- ("op" "noop") [])
  )
  (xor
   (call %init_peer_id% ("callbackSrv" "response") [ping_result])
   (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 2])
  )
 )
 (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 3])
)

            `,
            )
            .configHandler((h) => {
                h.on('getDataSrv', '-relay-', () => {
                    return client.relayPeerId!;
                });
                h.on('getDataSrv', 'node', () => {return node;});
h.on('getDataSrv', 'aggregator_service_id', () => {return aggregator_service_id;});
                h.onEvent('callbackSrv', 'response', (args) => {
  const [res] = args;
  resolve(res);
});

                h.onEvent('errorHandlingSrv', 'error', (args) => {
                    // assuming error is the single argument
                    const [err] = args;
                    reject(err);
                });
            })
            .handleScriptError(reject)
            .handleTimeout(() => {
                reject('Request timed out for ping');
            })
        if(config?.ttl) {
            r.withTTL(config.ttl)
        }
        request = r.build();
    });
    await client.initiateFlow(request!);
    return promise;
}
<<<<<<< HEAD
=======
      


export async function process_data(client: FluenceClient, node: string, aggregator_service_id: string, streamId: string, ticker: string, newPrice: number, now: number, config?: {ttl?: number}): Promise<{close:number;error_msg:string;high:number;last_updated:number;low:number;open:number;period:number;start_time:number;success:boolean;ticker:string}> {
    let request: RequestFlow;
    const promise = new Promise<{close:number;error_msg:string;high:number;last_updated:number;low:number;open:number;period:number;start_time:number;success:boolean;ticker:string}>((resolve, reject) => {
        const r = new RequestFlowBuilder()
            .disableInjections()
            .withRawScript(
                `
(xor
 (seq
  (seq
   (seq
    (seq
     (seq
      (seq
       (seq
        (seq
         (seq
          (seq
           (call %init_peer_id% ("getDataSrv" "-relay-") [] -relay-)
           (call %init_peer_id% ("getDataSrv" "node") [] node)
          )
          (call %init_peer_id% ("getDataSrv" "aggregator_service_id") [] aggregator_service_id)
         )
         (call %init_peer_id% ("getDataSrv" "streamId") [] streamId)
        )
        (call %init_peer_id% ("getDataSrv" "ticker") [] ticker)
       )
       (call %init_peer_id% ("getDataSrv" "newPrice") [] newPrice)
      )
      (call %init_peer_id% ("getDataSrv" "now") [] now)
     )
     (call -relay- ("op" "noop") [])
    )
    (xor
     (seq
      (call -relay- ("op" "noop") [])
      (call node (aggregator_service_id "process_data") [streamId ticker newPrice now] ping_result)
     )
     (seq
      (call -relay- ("op" "noop") [])
      (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 1])
     )
    )
   )
   (call -relay- ("op" "noop") [])
  )
  (xor
   (call %init_peer_id% ("callbackSrv" "response") [ping_result])
   (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 2])
  )
 )
 (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 3])
)

            `,
            )
            .configHandler((h) => {
                h.on('getDataSrv', '-relay-', () => {
                    return client.relayPeerId!;
                });
                h.on('getDataSrv', 'node', () => {return node;});
h.on('getDataSrv', 'aggregator_service_id', () => {return aggregator_service_id;});
h.on('getDataSrv', 'streamId', () => {return streamId;});
h.on('getDataSrv', 'ticker', () => {return ticker;});
h.on('getDataSrv', 'newPrice', () => {return newPrice;});
h.on('getDataSrv', 'now', () => {return now;});
                h.onEvent('callbackSrv', 'response', (args) => {
  const [res] = args;
  resolve(res);
});

                h.onEvent('errorHandlingSrv', 'error', (args) => {
                    // assuming error is the single argument
                    const [err] = args;
                    reject(err);
                });
            })
            .handleScriptError(reject)
            .handleTimeout(() => {
                reject('Request timed out for process_data');
            })
        if(config?.ttl) {
            r.withTTL(config.ttl)
        }
        request = r.build();
    });
    await client.initiateFlow(request!);
    return promise;
}
>>>>>>> 97cbeefef2ce1089523760caa8965f8a77cf44ad
      