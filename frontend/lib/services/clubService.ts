import { ApiPromise } from "@polkadot/api";
import { getPolkadotApi } from "../polkadot";

export interface Club {
  id: number;
  name: string;
  description: string;
  creator: string;
  createdAt: number;
  isActive: boolean;
}

export interface MemberProfile {
  account: string;
  clubId: number;
  joinedAt: number;
  reputation: number;
  contributionWeight: number;
  votingParticipation: number;
  proposalSuccessRate: number;
}

export class ClubService {
  private api: ApiPromise | null = null;

  async getApi(): Promise<ApiPromise> {
    if (!this.api) {
      this.api = await getPolkadotApi();
    }
    return this.api;
  }

  async createClub(name: string, description: string): Promise<string> {
    const api = await this.getApi();
    const tx = api.tx.ikubMembers.createClub(
      new TextEncoder().encode(name),
      new TextEncoder().encode(description)
    );
    return tx.hash.toString();
  }

  async getClubs(): Promise<Club[]> {
    const api = await this.getApi();
    const clubCount = await api.query.ikubMembers.clubCount();
    const count = clubCount.toNumber();

    const clubs: Club[] = [];
    for (let i = 0; i < count; i++) {
      const club = await api.query.ikubMembers.clubs(i);
      if (club.isSome) {
        const clubData = club.unwrap();
        clubs.push({
          id: i,
          name: new TextDecoder().decode(clubData.name),
          description: new TextDecoder().decode(clubData.description),
          creator: clubData.creator.toString(),
          createdAt: clubData.createdAt.toNumber(),
          isActive: clubData.isActive.valueOf(),
        });
      }
    }
    return clubs;
  }

  async getClub(clubId: number): Promise<Club | null> {
    const api = await this.getApi();
    const club = await api.query.ikubMembers.clubs(clubId);
    if (club.isSome) {
      const clubData = club.unwrap();
      return {
        id: clubId,
        name: new TextDecoder().decode(clubData.name),
        description: new TextDecoder().decode(clubData.description),
        creator: clubData.creator.toString(),
        createdAt: clubData.createdAt.toNumber(),
        isActive: clubData.isActive.valueOf(),
      };
    }
    return null;
  }

  async joinClub(clubId: number): Promise<string> {
    const api = await this.getApi();
    const tx = api.tx.ikubMembers.joinClub(clubId);
    return tx.hash.toString();
  }

  async getMembers(clubId: number): Promise<MemberProfile[]> {
    const api = await this.getApi();
    const memberCount = await api.query.ikubMembers.memberCount(clubId);
    const count = memberCount.toNumber();

    // Note: In production, you'd need to iterate through all members
    // This is a simplified version
    return [];
  }
}

export const clubService = new ClubService();
