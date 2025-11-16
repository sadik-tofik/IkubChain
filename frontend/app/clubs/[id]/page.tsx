"use client";

import { useState, useEffect } from "react";
import { useParams } from "next/navigation";
import { clubService, Club } from "../../../lib/services/clubService";
import { treasuryService } from "../../../lib/services/treasuryService";
import {
  proposalService,
  Proposal,
} from "../../../lib/services/proposalService";
import WalletConnection from "../../../components/WalletConnection";
import Link from "next/link";

export default function ClubDetailPage() {
  const params = useParams();
  const clubId = parseInt(params.id as string);

  const [club, setClub] = useState<Club | null>(null);
  const [balance, setBalance] = useState<string>("0");
  const [proposals, setProposals] = useState<Proposal[]>([]);
  const [loading, setLoading] = useState(true);
  const [currentAccount, setCurrentAccount] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<
    "overview" | "proposals" | "treasury"
  >("overview");

  useEffect(() => {
    if (clubId) {
      loadClubData();
    }
  }, [clubId]);

  const loadClubData = async () => {
    try {
      setLoading(true);
      const [clubData, balanceData, proposalsData] = await Promise.all([
        clubService.getClub(clubId),
        treasuryService.getBalance(clubId),
        proposalService.getProposals(clubId),
      ]);

      setClub(clubData);
      setBalance(balanceData);
      setProposals(proposalsData);
    } catch (error) {
      console.error("Error loading club data:", error);
    } finally {
      setLoading(false);
    }
  };

  const handleJoinClub = async () => {
    if (!currentAccount) {
      alert("Please connect your wallet first");
      return;
    }

    try {
      const txHash = await clubService.joinClub(clubId);
      alert(`Join request submitted! Transaction: ${txHash}`);
    } catch (error) {
      console.error("Error joining club:", error);
      alert("Failed to join club");
    }
  };

  if (loading) {
    return <div className="container mx-auto px-4 py-8">Loading...</div>;
  }

  if (!club) {
    return <div className="container mx-auto px-4 py-8">Club not found</div>;
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="flex justify-between items-start mb-8">
        <div>
          <Link
            href="/clubs"
            className="text-blue-500 hover:underline mb-4 inline-block"
          >
            ← Back to Clubs
          </Link>
          <h1 className="text-3xl font-bold mb-2">{club.name}</h1>
          <p className="text-gray-600">{club.description}</p>
        </div>
        <WalletConnection onAccountChange={setCurrentAccount} />
      </div>

      <div className="mb-6">
        <button
          onClick={handleJoinClub}
          className="px-6 py-2 bg-green-500 text-white rounded hover:bg-green-600"
        >
          Join Club
        </button>
      </div>

      <div className="border-b mb-6">
        <div className="flex gap-4">
          <button
            onClick={() => setActiveTab("overview")}
            className={`px-4 py-2 ${
              activeTab === "overview" ? "border-b-2 border-blue-500" : ""
            }`}
          >
            Overview
          </button>
          <button
            onClick={() => setActiveTab("proposals")}
            className={`px-4 py-2 ${
              activeTab === "proposals" ? "border-b-2 border-blue-500" : ""
            }`}
          >
            Proposals
          </button>
          <button
            onClick={() => setActiveTab("treasury")}
            className={`px-4 py-2 ${
              activeTab === "treasury" ? "border-b-2 border-blue-500" : ""
            }`}
          >
            Treasury
          </button>
        </div>
      </div>

      {activeTab === "overview" && (
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-4">Club Information</h2>
          <div className="space-y-2">
            <p>
              <strong>Creator:</strong> {club.creator}
            </p>
            <p>
              <strong>Created:</strong>{" "}
              {new Date(club.createdAt * 1000).toLocaleString()}
            </p>
            <p>
              <strong>Status:</strong> {club.isActive ? "Active" : "Inactive"}
            </p>
            <p>
              <strong>Treasury Balance:</strong> {balance}
            </p>
          </div>
        </div>
      )}

      {activeTab === "proposals" && (
        <div>
          <Link
            href={`/proposals/new?clubId=${clubId}`}
            className="mb-4 inline-block px-6 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
          >
            + Create Proposal
          </Link>
          <div className="space-y-4">
            {proposals.map((proposal) => (
              <div key={proposal.id} className="bg-white p-6 rounded-lg shadow">
                <h3 className="text-xl font-semibold mb-2">{proposal.title}</h3>
                <p className="text-gray-600 mb-4">{proposal.description}</p>
                <div className="flex justify-between text-sm text-gray-500">
                  <span>Status: {proposal.status}</span>
                  <span>
                    Votes: Aye {proposal.ayeVotes} / Nay {proposal.nayVotes}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {activeTab === "treasury" && (
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-4">Treasury</h2>
          <p className="text-2xl font-bold mb-4">Balance: {balance}</p>
          <Link
            href={`/contribute?clubId=${clubId}`}
            className="px-6 py-2 bg-green-500 text-white rounded hover:bg-green-600"
          >
            Contribute
          </Link>
        </div>
      )}
    </div>
  );
}
