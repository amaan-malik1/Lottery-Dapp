import { PublicKey } from "@solana/web3.js";

const program_add = import.meta.env.VITE_CONTRACT_ADDRESS;

export const MASTER_SEED = "Amnaaa";
export const LOTTERY_SEED = "malikamaan";
export const TICKET_SEED = "malikamadjnsjknan";
export const PROGRAM_ID = new PublicKey(program_add);