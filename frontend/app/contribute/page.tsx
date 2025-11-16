"use client";

import { useState, useEffect } from "react";
import { useSearchParams } from "next/navigation";
import { treasuryService } from "../../lib/services/treasuryService";
import WalletConnection from "../../components/WalletConnection";

export default function ContributePage() {
  const searchParams = useSearchParams();
  const clubId = parseInt(searchParams.get("clubId") || "0");

  const [amount, setAmount] = useState("");
  const [balance, setBalance] = useState<string>("0");
  const [activeCycle, setActiveCycle] = useState<number | null>(null);
  const [currentAccount, setCurrentAccount] = useState<string | null>(null);

  useEffect(() => {
    if (clubId) {
      loadTreasuryData();
    }
  }, [clubId]);

  const loadTreasuryData = async () => {
    try {
      const [balanceData, cycleData] = await Promise.all([
        treasuryService.getBalance(clubId),
        treasuryService.getActiveCycle(clubId),
      ]);
      setBalance(balanceData);
      setActiveCycle(cycleData);
    } catch (error) {
      console.error("Error loading treasury data:", error);
    }
  };

  const handleContribute = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!currentAccount) {
      alert("Please connect your wallet first");
      return;
    }

    try {
      const txHash = await treasuryService.contribute(clubId, amount);
      alert(`Contribution submitted! Transaction: ${txHash}`);
      setAmount("");
      loadTreasuryData();
    } catch (error) {
      console.error("Error contributing:", error);
      alert("Failed to contribute");
    }
  };

  if (!clubId) {
    return (
      <div className="container mx-auto px-4 py-8">Please select a club</div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="flex justify-between items-center mb-8">
        <h1 className="text-3xl font-bold">Contribute to Treasury</h1>
        <WalletConnection onAccountChange={setCurrentAccount} />
      </div>

      <div className="bg-white p-6 rounded-lg shadow max-w-md">
        <div className="mb-4">
          <p className="text-sm text-gray-600">Current Balance</p>
          <p className="text-2xl font-bold">{balance}</p>
        </div>

        {activeCycle === null ? (
          <div className="mb-4 p-4 bg-yellow-50 rounded">
            <p className="text-sm text-yellow-800">
              No active contribution cycle. Please open a cycle first.
            </p>
          </div>
        ) : (
          <form onSubmit={handleContribute}>
            <div className="mb-4">
              <label className="block text-sm font-medium mb-2">Amount</label>
              <input
                type="number"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                className="w-full px-4 py-2 border rounded"
                placeholder="Enter amount"
                required
              />
            </div>
            <button
              type="submit"
              className="w-full px-6 py-2 bg-green-500 text-white rounded hover:bg-green-600"
            >
              Contribute
            </button>
          </form>
        )}
      </div>
    </div>
  );
}
