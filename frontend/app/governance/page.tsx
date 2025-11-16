"use client";

import { useState, useEffect } from "react";
import { useSearchParams } from "next/navigation";
import {
  proposalService,
  Proposal,
  ProposalType,
  VotingMechanism,
  VoteChoice,
} from "../../lib/services/proposalService";
import WalletConnection from "../../components/WalletConnection";

export default function GovernancePage() {
  const searchParams = useSearchParams();
  const clubId = parseInt(searchParams.get("clubId") || "0");

  const [proposals, setProposals] = useState<Proposal[]>([]);
  const [loading, setLoading] = useState(true);
  const [currentAccount, setCurrentAccount] = useState<string | null>(null);
  const [selectedProposal, setSelectedProposal] = useState<Proposal | null>(
    null
  );
  const [voteChoice, setVoteChoice] = useState<VoteChoice>(VoteChoice.Aye);

  useEffect(() => {
    if (clubId) {
      loadProposals();
    }
  }, [clubId]);

  const loadProposals = async () => {
    try {
      setLoading(true);
      const data = await proposalService.getProposals(clubId);
      setProposals(data);
    } catch (error) {
      console.error("Error loading proposals:", error);
    } finally {
      setLoading(false);
    }
  };

  const handleVote = async (proposalId: number) => {
    if (!currentAccount) {
      alert("Please connect your wallet first");
      return;
    }

    try {
      const txHash = await proposalService.vote(clubId, proposalId, voteChoice);
      alert(`Vote submitted! Transaction: ${txHash}`);
      loadProposals();
    } catch (error) {
      console.error("Error voting:", error);
      alert("Failed to vote");
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
        <h1 className="text-3xl font-bold">Governance</h1>
        <WalletConnection onAccountChange={setCurrentAccount} />
      </div>

      {loading ? (
        <div>Loading proposals...</div>
      ) : (
        <div className="space-y-4">
          {proposals.map((proposal) => (
            <div key={proposal.id} className="bg-white p-6 rounded-lg shadow">
              <h3 className="text-xl font-semibold mb-2">{proposal.title}</h3>
              <p className="text-gray-600 mb-4">{proposal.description}</p>
              <div className="flex justify-between items-center">
                <div className="text-sm text-gray-500">
                  <p>Status: {proposal.status}</p>
                  <p>
                    Votes: Aye {proposal.ayeVotes} / Nay {proposal.nayVotes} /
                    Abstain {proposal.abstainVotes}
                  </p>
                </div>
                {proposal.status === "Active" && (
                  <div className="flex gap-2">
                    <select
                      value={voteChoice}
                      onChange={(e) =>
                        setVoteChoice(e.target.value as VoteChoice)
                      }
                      className="px-4 py-2 border rounded"
                    >
                      <option value={VoteChoice.Aye}>Aye</option>
                      <option value={VoteChoice.Nay}>Nay</option>
                      <option value={VoteChoice.Abstain}>Abstain</option>
                    </select>
                    <button
                      onClick={() => handleVote(proposal.id)}
                      className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
                    >
                      Vote
                    </button>
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
