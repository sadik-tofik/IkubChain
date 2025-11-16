import { ApiPromise } from "@polkadot/api";
import { getPolkadotApi } from "../polkadot";

export interface ContributionCycle {
  id: number;
  clubId: number;
  startBlock: number;
  endBlock: number;
  totalContributions: string;
  returns: string;
  status: string;
  minimumContribution: string;
}

export class TreasuryService {
  private api: ApiPromise | null = null;

  async getApi(): Promise<ApiPromise> {
    if (!this.api) {
      this.api = await getPolkadotApi();
    }
    return this.api;
  }

  async getBalance(clubId: number): Promise<string> {
    const api = await this.getApi();
    const balance = await api.query.ikubTreasury.treasuryBalance(clubId);
    return balance.toString();
  }

  async deposit(clubId: number, amount: string): Promise<string> {
    const api = await this.getApi();
    const tx = api.tx.ikubTreasury.deposit(clubId, amount);
    return tx.hash.toString();
  }

  async openContributionCycle(
    clubId: number,
    contributionPeriod?: number,
    minimumContribution?: string
  ): Promise<string> {
    const api = await this.getApi();
    const tx = api.tx.ikubTreasury.openContributionCycle(
      clubId,
      contributionPeriod || null,
      minimumContribution || null
    );
    return tx.hash.toString();
  }

  async contribute(clubId: number, amount: string): Promise<string> {
    const api = await this.getApi();
    const tx = api.tx.ikubTreasury.contribute(clubId, amount);
    return tx.hash.toString();
  }

  async closeCycle(clubId: number): Promise<string> {
    const api = await this.getApi();
    const tx = api.tx.ikubTreasury.closeCycle(clubId);
    return tx.hash.toString();
  }

  async distributeReturns(
    clubId: number,
    cycleId: number,
    returns: string
  ): Promise<string> {
    const api = await this.getApi();
    const tx = api.tx.ikubTreasury.distributeReturns(clubId, cycleId, returns);
    return tx.hash.toString();
  }

  async claimReturns(clubId: number, cycleId: number): Promise<string> {
    const api = await this.getApi();
    const tx = api.tx.ikubTreasury.claimReturns(clubId, cycleId);
    return tx.hash.toString();
  }

  async getActiveCycle(clubId: number): Promise<number | null> {
    const api = await this.getApi();
    const cycleId = await api.query.ikubTreasury.activeCycle(clubId);
    if (cycleId.isSome) {
      return cycleId.unwrap().toNumber();
    }
    return null;
  }

  async getCycle(
    clubId: number,
    cycleId: number
  ): Promise<ContributionCycle | null> {
    const api = await this.getApi();
    const cycle = await api.query.ikubTreasury.contributionCycles(
      clubId,
      cycleId
    );
    if (cycle.isSome) {
      const cycleData = cycle.unwrap();
      return {
        id: cycleId,
        clubId,
        startBlock: cycleData.startBlock.toNumber(),
        endBlock: cycleData.endBlock.toNumber(),
        totalContributions: cycleData.totalContributions.toString(),
        returns: cycleData.returns.toString(),
        status: this.decodeStatus(cycleData.status),
        minimumContribution: cycleData.minimumContribution.toString(),
      };
    }
    return null;
  }

  private decodeStatus(status: any): string {
    if (status.isOpen) return "Open";
    if (status.isClosed) return "Closed";
    return "Distributed";
  }
}

export const treasuryService = new TreasuryService();
