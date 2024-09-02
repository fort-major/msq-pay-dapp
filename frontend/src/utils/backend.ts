import { HttpAgent, Identity, AnonymousIdentity, Agent, Actor } from "@fort-major/agent-js-fork";
import {
  _SERVICE as PaymentHubActor,
  idlFactory as PaymentHubIdlFactory,
} from "../declarations/payment_hub/payment_hub.did";
import {
  _SERVICE as InvoiceHistoryActor,
  idlFactory as InvoiceHistoryIdlFactory,
} from "../declarations/invoice_history/invoice_history.did";

export function newPaymentHubActor(agent: Agent): PaymentHubActor {
  return Actor.createActor(PaymentHubIdlFactory, {
    canisterId: import.meta.env.VITE_PAYMENT_HUB_CANISTER_ID,
    agent,
  });
}

export function newInvoiceHistoryActor(agent: Agent): InvoiceHistoryActor {
  return Actor.createActor(InvoiceHistoryIdlFactory, {
    canisterId: import.meta.env.VITE_INVOICE_HISTORY_CANISTER_ID,
    agent,
  });
}

export async function makeAgent(identity?: Identity | undefined): Promise<Agent> {
  const agent = new HttpAgent({
    host: import.meta.env.VITE_IC_HOST,
    identity,
    retryTimes: 10,
  });

  if (import.meta.env.DEV) {
    await agent.fetchRootKey();
  }

  return agent;
}

export async function makeAnonymousAgent(): Promise<Agent> {
  const id = new AnonymousIdentity();
  return makeAgent(id);
}

export function optUnwrap<T>(it: [] | [T] | T[]): T | undefined {
  return it.length > 0 ? it[0] : undefined;
}

export function opt<T>(it: T | undefined): [] | [T] {
  return it !== undefined ? [it] : [];
}
