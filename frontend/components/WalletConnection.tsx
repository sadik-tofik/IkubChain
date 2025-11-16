"use client";

import { useState, useEffect } from "react";
import { web3Accounts, web3Enable } from "@polkadot/extension-dapp";
import { ApiPromise } from "@polkadot/api";
import { getPolkadotApi } from "../lib/polkadot";

interface WalletConnectionProps {
  onAccountChange?: (account: string | null) => void;
}

export default function WalletConnection({
  onAccountChange,
}: WalletConnectionProps) {
  const [accounts, setAccounts] = useState<any[]>([]);
  const [selectedAccount, setSelectedAccount] = useState<string | null>(null);
  const [isConnecting, setIsConnecting] = useState(false);

  useEffect(() => {
    if (selectedAccount && onAccountChange) {
      onAccountChange(selectedAccount);
    }
  }, [selectedAccount, onAccountChange]);

  const connectWallet = async () => {
    setIsConnecting(true);
    try {
      const extensions = await web3Enable("IkubChain");
      if (extensions.length === 0) {
        alert(
          "No Polkadot extension found. Please install Polkadot.js extension."
        );
        return;
      }

      const allAccounts = await web3Accounts();
      setAccounts(allAccounts);

      if (allAccounts.length > 0) {
        setSelectedAccount(allAccounts[0].address);
      }
    } catch (error) {
      console.error("Error connecting wallet:", error);
      alert("Failed to connect wallet");
    } finally {
      setIsConnecting(false);
    }
  };

  return (
    <div className="flex items-center gap-4">
      {selectedAccount ? (
        <div className="flex items-center gap-2">
          <span className="text-sm text-gray-600">
            {selectedAccount.slice(0, 8)}...{selectedAccount.slice(-8)}
          </span>
          <button
            onClick={() => setSelectedAccount(null)}
            className="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
          >
            Disconnect
          </button>
        </div>
      ) : (
        <button
          onClick={connectWallet}
          disabled={isConnecting}
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:opacity-50"
        >
          {isConnecting ? "Connecting..." : "Connect Wallet"}
        </button>
      )}
    </div>
  );
}
