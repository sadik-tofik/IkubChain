"use client";

import { useState, useEffect } from "react";
import { clubService, Club } from "../../lib/services/clubService";
import WalletConnection from "../../components/WalletConnection";
import Link from "next/link";

export default function ClubsPage() {
  const [clubs, setClubs] = useState<Club[]>([]);
  const [loading, setLoading] = useState(true);
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [formData, setFormData] = useState({ name: "", description: "" });
  const [currentAccount, setCurrentAccount] = useState<string | null>(null);

  useEffect(() => {
    loadClubs();
  }, []);

  const loadClubs = async () => {
    try {
      setLoading(true);
      const data = await clubService.getClubs();
      setClubs(data);
    } catch (error) {
      console.error("Error loading clubs:", error);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateClub = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!currentAccount) {
      alert("Please connect your wallet first");
      return;
    }

    try {
      const txHash = await clubService.createClub(
        formData.name,
        formData.description
      );
      alert(`Club creation submitted! Transaction: ${txHash}`);
      setShowCreateForm(false);
      setFormData({ name: "", description: "" });
      // Reload clubs after a delay
      setTimeout(loadClubs, 2000);
    } catch (error) {
      console.error("Error creating club:", error);
      alert("Failed to create club");
    }
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="flex justify-between items-center mb-8">
        <h1 className="text-3xl font-bold">Investment Clubs</h1>
        <WalletConnection onAccountChange={setCurrentAccount} />
      </div>

      {showCreateForm ? (
        <div className="mb-8 p-6 bg-white rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-4">Create New Club</h2>
          <form onSubmit={handleCreateClub}>
            <div className="mb-4">
              <label className="block text-sm font-medium mb-2">
                Club Name
              </label>
              <input
                type="text"
                value={formData.name}
                onChange={(e) =>
                  setFormData({ ...formData, name: e.target.value })
                }
                className="w-full px-4 py-2 border rounded"
                required
              />
            </div>
            <div className="mb-4">
              <label className="block text-sm font-medium mb-2">
                Description
              </label>
              <textarea
                value={formData.description}
                onChange={(e) =>
                  setFormData({ ...formData, description: e.target.value })
                }
                className="w-full px-4 py-2 border rounded"
                rows={4}
                required
              />
            </div>
            <div className="flex gap-4">
              <button
                type="submit"
                className="px-6 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
              >
                Create Club
              </button>
              <button
                type="button"
                onClick={() => setShowCreateForm(false)}
                className="px-6 py-2 bg-gray-300 text-gray-700 rounded hover:bg-gray-400"
              >
                Cancel
              </button>
            </div>
          </form>
        </div>
      ) : (
        <button
          onClick={() => setShowCreateForm(true)}
          className="mb-8 px-6 py-2 bg-green-500 text-white rounded hover:bg-green-600"
        >
          + Create New Club
        </button>
      )}

      {loading ? (
        <div className="text-center py-8">Loading clubs...</div>
      ) : clubs.length === 0 ? (
        <div className="text-center py-8 text-gray-500">
          No clubs found. Create one to get started!
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {clubs.map((club) => (
            <Link
              key={club.id}
              href={`/clubs/${club.id}`}
              className="p-6 bg-white rounded-lg shadow hover:shadow-lg transition"
            >
              <h3 className="text-xl font-semibold mb-2">{club.name}</h3>
              <p className="text-gray-600 mb-4 line-clamp-3">
                {club.description}
              </p>
              <div className="text-sm text-gray-500">
                <p>
                  Creator: {club.creator.slice(0, 8)}...{club.creator.slice(-8)}
                </p>
                <p>
                  Created:{" "}
                  {new Date(club.createdAt * 1000).toLocaleDateString()}
                </p>
              </div>
            </Link>
          ))}
        </div>
      )}
    </div>
  );
}
