import { ApiPromise, WsProvider } from "@polkadot/api";

let api: ApiPromise | null = null;

export async function getPolkadotApi(): Promise<ApiPromise> {
  if (api) {
    return api;
  }

  const wsProvider = new WsProvider("ws://127.0.0.1:9944");
  api = await ApiPromise.create({ provider: wsProvider });

  return api;
}

export function disconnectApi(): void {
  if (api) {
    api.disconnect();
    api = null;
  }
}
