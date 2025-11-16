"use client";

import Link from "next/link";
import WalletConnection from "./WalletConnection";

export default function Navigation() {
  return (
    <nav className="bg-white shadow-sm border-b">
      <div className="container mx-auto px-4">
        <div className="flex justify-between items-center h-16">
          <div className="flex items-center gap-6">
            <Link href="/" className="text-xl font-bold text-blue-600">
              IkubChain
            </Link>
            <div className="flex gap-4">
              <Link
                href="/clubs"
                className="text-gray-600 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium"
              >
                Clubs
              </Link>
              <Link
                href="/governance"
                className="text-gray-600 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium"
              >
                Governance
              </Link>
            </div>
          </div>
          <WalletConnection />
        </div>
      </div>
    </nav>
  );
}
