"use client";

import { useEffect, useState } from "react";
import { getPolkadotApi } from "@/lib/polkadot";

export default function DashboardPage() {
  const [apiConnected, setApiConnected] = useState(false);
  const [chainInfo, setChainInfo] = useState<string>("");

  useEffect(() => {
    async function connect() {
      try {
        const api = await getPolkadotApi();
        const [chain, nodeName, nodeVersion] = await Promise.all([
          api.rpc.system.chain(),
          api.rpc.system.name(),
          api.rpc.system.version(),
        ]);
        setChainInfo(`${chain} (${nodeName} v${nodeVersion})`);
        setApiConnected(true);
      } catch (error) {
        console.error("Failed to connect to Polkadot API:", error);
      }
    }
    connect();
  }, []);

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-3xl font-bold mb-6">Dashboard</h1>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div className="bg-white dark:bg-gray-800 p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-2">Connection Status</h2>
          <p className={apiConnected ? "text-green-600" : "text-red-600"}>
            {apiConnected ? "Connected" : "Disconnected"}
          </p>
          {chainInfo && (
            <p className="text-sm text-gray-600 dark:text-gray-400 mt-2">
              {chainInfo}
            </p>
          )}
        </div>

        <div className="bg-white dark:bg-gray-800 p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-2">My Clubs</h2>
          <p className="text-3xl font-bold">0</p>
          <p className="text-sm text-gray-600 dark:text-gray-400">
            Investment clubs you're a member of
          </p>
        </div>

        <div className="bg-white dark:bg-gray-800 p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-2">Active Proposals</h2>
          <p className="text-3xl font-bold">0</p>
          <p className="text-sm text-gray-600 dark:text-gray-400">
            Proposals requiring your vote
          </p>
        </div>
      </div>
    </div>
  );
}
