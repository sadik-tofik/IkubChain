"use client";

import Link from "next/link";

export default function ClubsPage() {
  return (
    <div className="container mx-auto px-4 py-8">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-bold">Investment Clubs</h1>
        <Link
          href="/clubs/create"
          className="px-6 py-3 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors"
        >
          Create Club
        </Link>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {/* Club cards would be rendered here */}
        <div className="bg-white dark:bg-gray-800 p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-2">No clubs yet</h2>
          <p className="text-gray-600 dark:text-gray-400">
            Create your first investment club to get started
          </p>
        </div>
      </div>
    </div>
  );
}
