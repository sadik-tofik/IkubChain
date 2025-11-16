"use client";

import { useState, useEffect } from "react";
import { useSearchParams, useRouter } from "next/navigation";
import {
  proposalService,
  ProposalType,
  VotingMechanism,
} from "../../../lib/services/proposalService";
import WalletConnection from "../../../components/WalletConnection";

export default function NewProposalPage() {
  const searchParams = useSearchParams();
  const router = useRouter();
  const clubId = parseInt(searchParams.get("clubId") || "0");

  const [formData, setFormData] = useState({
    title: "",
    description: "",
    proposalType: ProposalType.Operational,
    votingMechanism: VotingMechanism.SimpleMajority,
    votingDuration: 10000,
    approvalThreshold: 50,
  });
  const [currentAccount, setCurrentAccount] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!currentAccount) {
      alert("Please connect your wallet first");
      return;
    }

    setSubmitting(true);
    try {
      const txHash = await proposalService.createProposal(
        clubId,
        formData.proposalType,
        formData.votingMechanism,
        formData.title,
        formData.description,
        formData.votingDuration,
        formData.approvalThreshold
      );
      alert(`Proposal created! Transaction: ${txHash}`);
      router.push(`/clubs/${clubId}?tab=proposals`);
    } catch (error) {
      console.error("Error creating proposal:", error);
      alert("Failed to create proposal");
    } finally {
      setSubmitting(false);
    }
  };

  if (!clubId) {
    return (
      <div className="container mx-auto px-4 py-8">
        Please select a club first
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8 max-w-2xl">
      <div className="flex justify-between items-center mb-8">
        <h1 className="text-3xl font-bold">Create New Proposal</h1>
        <WalletConnection onAccountChange={setCurrentAccount} />
      </div>

      <form onSubmit={handleSubmit} className="bg-white p-6 rounded-lg shadow">
        <div className="mb-4">
          <label className="block text-sm font-medium mb-2">Title</label>
          <input
            type="text"
            value={formData.title}
            onChange={(e) =>
              setFormData({ ...formData, title: e.target.value })
            }
            className="w-full px-4 py-2 border rounded"
            required
          />
        </div>

        <div className="mb-4">
          <label className="block text-sm font-medium mb-2">Description</label>
          <textarea
            value={formData.description}
            onChange={(e) =>
              setFormData({ ...formData, description: e.target.value })
            }
            className="w-full px-4 py-2 border rounded"
            rows={6}
            required
          />
        </div>

        <div className="mb-4">
          <label className="block text-sm font-medium mb-2">
            Proposal Type
          </label>
          <select
            value={formData.proposalType}
            onChange={(e) =>
              setFormData({
                ...formData,
                proposalType: e.target.value as ProposalType,
              })
            }
            className="w-full px-4 py-2 border rounded"
          >
            <option value={ProposalType.Investment}>Investment</option>
            <option value={ProposalType.Operational}>Operational</option>
            <option value={ProposalType.Emergency}>Emergency</option>
            <option value={ProposalType.Constitutional}>Constitutional</option>
          </select>
        </div>

        <div className="mb-4">
          <label className="block text-sm font-medium mb-2">
            Voting Mechanism
          </label>
          <select
            value={formData.votingMechanism}
            onChange={(e) =>
              setFormData({
                ...formData,
                votingMechanism: e.target.value as VotingMechanism,
              })
            }
            className="w-full px-4 py-2 border rounded"
          >
            <option value={VotingMechanism.SimpleMajority}>
              Simple Majority
            </option>
            <option value={VotingMechanism.Quadratic}>Quadratic</option>
            <option value={VotingMechanism.Conviction}>Conviction</option>
            <option value={VotingMechanism.Delegated}>Delegated</option>
          </select>
          {formData.votingMechanism === VotingMechanism.Quadratic && (
            <p className="text-sm text-gray-500 mt-2">
              Quadratic voting: Cost = votes². You can cast up to sqrt(balance)
              votes.
            </p>
          )}
        </div>

        <div className="mb-4">
          <label className="block text-sm font-medium mb-2">
            Voting Duration (blocks)
          </label>
          <input
            type="number"
            value={formData.votingDuration}
            onChange={(e) =>
              setFormData({
                ...formData,
                votingDuration: parseInt(e.target.value) || 0,
              })
            }
            className="w-full px-4 py-2 border rounded"
            min="1"
            required
          />
        </div>

        <div className="mb-4">
          <label className="block text-sm font-medium mb-2">
            Approval Threshold (%)
          </label>
          <input
            type="number"
            value={formData.approvalThreshold}
            onChange={(e) =>
              setFormData({
                ...formData,
                approvalThreshold: parseInt(e.target.value) || 0,
              })
            }
            className="w-full px-4 py-2 border rounded"
            min="1"
            max="100"
            required
          />
        </div>

        <div className="flex gap-4">
          <button
            type="submit"
            disabled={submitting}
            className="px-6 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:opacity-50"
          >
            {submitting ? "Creating..." : "Create Proposal"}
          </button>
          <button
            type="button"
            onClick={() => router.back()}
            className="px-6 py-2 bg-gray-300 text-gray-700 rounded hover:bg-gray-400"
          >
            Cancel
          </button>
        </div>
      </form>
    </div>
  );
}
