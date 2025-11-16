import { ApiPromise } from "@polkadot/api";
import { getPolkadotApi } from "../polkadot";

export enum ProposalType {
  Investment = "Investment",
  Operational = "Operational",
  Emergency = "Emergency",
  Constitutional = "Constitutional",
}

export enum VotingMechanism {
  SimpleMajority = "SimpleMajority",
  Quadratic = "Quadratic",
  Conviction = "Conviction",
  Delegated = "Delegated",
}

export enum VoteChoice {
  Aye = "Aye",
  Nay = "Nay",
  Abstain = "Abstain",
}

export interface Proposal {
  id: number;
  clubId: number;
  proposer: string;
  proposalType: ProposalType;
  votingMechanism: VotingMechanism;
  title: string;
  description: string;
  deposit: string;
  createdAt: number;
  votingEnd: number;
  status: string;
  ayeVotes: string;
  nayVotes: string;
  abstainVotes: string;
  approvalThreshold: number;
}

export class ProposalService {
  private api: ApiPromise | null = null;

  async getApi(): Promise<ApiPromise> {
    if (!this.api) {
      this.api = await getPolkadotApi();
    }
    return this.api;
  }

  async createProposal(
    clubId: number,
    proposalType: ProposalType,
    votingMechanism: VotingMechanism,
    title: string,
    description: string,
    votingDuration: number,
    approvalThreshold: number
  ): Promise<string> {
    const api = await this.getApi();
    const tx = api.tx.ikubGovernance.createProposal(
      clubId,
      { [proposalType]: null },
      { [votingMechanism]: null },
      new TextEncoder().encode(title),
      new TextEncoder().encode(description),
      votingDuration,
      approvalThreshold
    );
    return tx.hash.toString();
  }

  async vote(
    clubId: number,
    proposalId: number,
    choice: VoteChoice
  ): Promise<string> {
    const api = await this.getApi();
    const tx = api.tx.ikubGovernance.vote(clubId, proposalId, {
      [choice]: null,
    });
    return tx.hash.toString();
  }

  async finalizeProposal(clubId: number, proposalId: number): Promise<string> {
    const api = await this.getApi();
    const tx = api.tx.ikubGovernance.finalizeProposal(clubId, proposalId);
    return tx.hash.toString();
  }

  async getProposals(clubId: number): Promise<Proposal[]> {
    const api = await this.getApi();
    const proposalCount = await api.query.ikubGovernance.proposalCount(clubId);
    const count = proposalCount.toNumber();

    const proposals: Proposal[] = [];
    for (let i = 0; i < count; i++) {
      const proposal = await api.query.ikubGovernance.proposals(clubId, i);
      if (proposal.isSome) {
        const propData = proposal.unwrap();
        proposals.push({
          id: i,
          clubId,
          proposer: propData.proposer.toString(),
          proposalType: this.decodeProposalType(propData.proposalType),
          votingMechanism: this.decodeVotingMechanism(propData.votingMechanism),
          title: new TextDecoder().decode(propData.title),
          description: new TextDecoder().decode(propData.description),
          deposit: propData.deposit.toString(),
          createdAt: propData.createdAt.toNumber(),
          votingEnd: propData.votingEnd.toNumber(),
          status: this.decodeStatus(propData.status),
          ayeVotes: propData.ayeVotes.toString(),
          nayVotes: propData.nayVotes.toString(),
          abstainVotes: propData.abstainVotes.toString(),
          approvalThreshold: propData.approvalThreshold,
        });
      }
    }
    return proposals;
  }

  private decodeProposalType(type: any): ProposalType {
    if (type.isInvestment) return ProposalType.Investment;
    if (type.isOperational) return ProposalType.Operational;
    if (type.isEmergency) return ProposalType.Emergency;
    return ProposalType.Constitutional;
  }

  private decodeVotingMechanism(mechanism: any): VotingMechanism {
    if (mechanism.isSimpleMajority) return VotingMechanism.SimpleMajority;
    if (mechanism.isQuadratic) return VotingMechanism.Quadratic;
    if (mechanism.isConviction) return VotingMechanism.Conviction;
    return VotingMechanism.Delegated;
  }

  private decodeStatus(status: any): string {
    if (status.isActive) return "Active";
    if (status.isPassed) return "Passed";
    if (status.isRejected) return "Rejected";
    if (status.isExpired) return "Expired";
    return "Cancelled";
  }
}

export const proposalService = new ProposalService();
