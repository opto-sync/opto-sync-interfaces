export type Operation = 'upsert' | 'delete';
export type LocalProtocolStatus = 'pending' | 'confirmed';
export type ResultStatus = 'applied' | 'duplicate' | 'rejected';
export type ConnectivityState = 'unknown' | 'offline' | 'link' | 'internet';
export type ConnectivityMode = 'automatic' | 'offline';
export type ConnectivitySource =
  | 'initial'
  | 'manual'
  | 'browser-event'
  | 'probe'
  | 'forced-offline';

export interface IngestRecord {
  readonly table: string;
  readonly recordId: string;
  readonly operation?: Operation;
  readonly baseRevision?: string;
  readonly payload: Readonly<Record<string, unknown>>;
}

export interface IngestEnvelope {
  readonly formatVersion: 1;
  readonly source?: string;
  readonly records: readonly IngestRecord[];
}

export interface ProtocolMutation {
  readonly mutationId: string;
  readonly operation: Operation;
  readonly table: string;
  readonly recordId: string;
  readonly payload?: unknown;
  readonly baseRevision?: string;
  readonly resurrect: boolean;
  readonly status: LocalProtocolStatus;
}

export interface PushRequest {
  readonly protocolVersion: 1;
  readonly clientId: string;
  readonly mutations: readonly ProtocolMutation[];
}

export interface MutationResult {
  readonly mutationId: string;
  readonly status: ResultStatus;
  readonly originalStatus?: ResultStatus;
  readonly checkpoint?: string;
  readonly revision?: string;
  readonly code?: string;
  readonly message?: string;
}

export interface PushResponse {
  readonly protocolVersion: 1;
  readonly clientId: string;
  readonly lastMutationId: string;
  readonly checkpoint: string;
  readonly results: readonly MutationResult[];
}

export interface ConnectivitySnapshot {
  readonly state: ConnectivityState;
  readonly mode: ConnectivityMode;
  readonly source: ConnectivitySource;
  readonly changedAt: number;
  readonly verifiedAt?: number;
}
