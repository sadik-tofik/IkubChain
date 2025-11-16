import { ApiPromise } from "@polkadot/api";
import { getPolkadotApi } from "../polkadot";

export interface CrossChainOperation {
  id: number;
  clubId: number;
  destinationParachainId: number;
  operationType: string;
  status: string;
  amount: string | null;
  createdAt: number;
  completedAt: number | null;
}

export class XcmService {
  private api: ApiPromise | null = null;

  async getApi(): Promise<ApiPromise> {
    if (!this.api) {
      this.api = await getPolkadotApi();
    }
    return this.api;
  }

  async sendFundsToParachain(
    clubId: number,
    destParaId: number,
    amount: string,
    beneficiary: string
  ): Promise<string> {
    const api = await this.getApi();
    const tx = api.tx.ikubCrosschain.sendFundsToParachain(
      clubId,
      destParaId,
      amount,
      new TextEncoder().encode(beneficiary)
    );
    return tx.hash.toString();
  }

  async executeRemoteInvestment(
    clubId: number,
    destParaId: number,
    callData: string
  ): Promise<string> {
    const api = await this.getApi();
    const tx = api.tx.ikubCrosschain.executeRemoteInvestment(
      clubId,
      destParaId,
      new TextEncoder().encode(callData)
    );
    return tx.hash.toString();
  }

  async getOperations(clubId: number): Promise<CrossChainOperation[]> {
    const api = await this.getApi();
    const operationCount = await api.query.ikubCrosschain.operationCount(
      clubId
    );
    const count = operationCount.toNumber();

    const operations: CrossChainOperation[] = [];
    for (let i = 0; i < count; i++) {
      const operation = await api.query.ikubCrosschain.operations(clubId, i);
      if (operation.isSome) {
        const opData = operation.unwrap();
        operations.push({
          id: i,
          clubId,
          destinationParachainId: opData.destinationParachainId,
          operationType: this.decodeOperationType(opData.operationType),
          status: this.decodeStatus(opData.status),
          amount: opData.amount ? opData.amount.toString() : null,
          createdAt: opData.createdAt.toNumber(),
          completedAt: opData.completedAt
            ? opData.completedAt.toNumber()
            : null,
        });
      }
    }
    return operations;
  }

  private decodeOperationType(type: any): string {
    if (type.isTransferFunds) return "TransferFunds";
    return "ExecuteInvestment";
  }

  private decodeStatus(status: any): string {
    if (status.isPending) return "Pending";
    if (status.isInProgress) return "InProgress";
    if (status.isCompleted) return "Completed";
    return "Failed";
  }
}

export const xcmService = new XcmService();
