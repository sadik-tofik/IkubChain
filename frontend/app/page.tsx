"use client";

import Link from "next/link";
import Navigation from "../components/Navigation";

export default function HomePage() {
  return (
    <>
      <Navigation />
      <div className="container mx-auto px-4 py-16">
        <div className="text-center mb-16">
          <h1 className="text-5xl font-bold mb-4">IkubChain</h1>
          <p className="text-xl text-gray-600 mb-8">
            Decentralized Investment Club Platform on Polkadot
          </p>
          <p className="text-gray-500 max-w-2xl mx-auto">
            Transform traditional community-based investment models into
            transparent, secure, and globally accessible Web3 investment DAOs
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-16">
          <Link
            href="/clubs"
            className="p-6 bg-white rounded-lg shadow hover:shadow-lg transition"
          >
            <h2 className="text-2xl font-semibold mb-2">Investment Clubs</h2>
            <p className="text-gray-600">
              Create and join investment clubs. Manage members and track
              reputation.
            </p>
          </Link>

          <Link
            href="/governance"
            className="p-6 bg-white rounded-lg shadow hover:shadow-lg transition"
          >
            <h2 className="text-2xl font-semibold mb-2">Governance</h2>
            <p className="text-gray-600">
              Create proposals and vote using quadratic voting, conviction
              voting, and more.
            </p>
          </Link>

          <div className="p-6 bg-white rounded-lg shadow">
            <h2 className="text-2xl font-semibold mb-2">Treasury</h2>
            <p className="text-gray-600">
              Manage club funds with contribution cycles and proportional
              returns distribution.
            </p>
          </div>

          <div className="p-6 bg-white rounded-lg shadow">
            <h2 className="text-2xl font-semibold mb-2">Cross-Chain</h2>
            <p className="text-gray-600">
              Execute investments across parachains using XCM v3.
            </p>
          </div>

          <div className="p-6 bg-white rounded-lg shadow">
            <h2 className="text-2xl font-semibold mb-2">Disputes</h2>
            <p className="text-gray-600">
              Resolve conflicts through decentralized dispute resolution.
            </p>
          </div>

          <div className="p-6 bg-white rounded-lg shadow">
            <h2 className="text-2xl font-semibold mb-2">Analytics</h2>
            <p className="text-gray-600">
              Track performance metrics and on-chain events.
            </p>
          </div>
        </div>

        <div className="bg-blue-50 rounded-lg p-8 text-center">
          <h2 className="text-2xl font-semibold mb-4">Ready to Get Started?</h2>
          <p className="text-gray-600 mb-6">
            Connect your wallet and create your first investment club
          </p>
          <Link
            href="/clubs"
            className="inline-block px-8 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition"
          >
            Explore Clubs
          </Link>
        </div>
      </div>
    </>
  );
}
