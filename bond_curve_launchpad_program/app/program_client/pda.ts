import {PublicKey} from "@solana/web3.js";
import {BN} from "@coral-xyz/anchor";

export const deriveLaunchpadConfigPDA = (programId: PublicKey): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("config"),
        ],
        programId,
    )
};

export type TokenProjectSeeds = {
    mint: PublicKey, 
};

export const deriveTokenProjectPDA = (
    seeds: TokenProjectSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("project"),
            seeds.mint.toBuffer(),
        ],
        programId,
    )
};

export type BundleTrackerSeeds = {
    mint: PublicKey, 
    wallet: PublicKey, 
};

export const deriveBundleTrackerPDA = (
    seeds: BundleTrackerSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("bundle"),
            seeds.mint.toBuffer(),
            seeds.wallet.toBuffer(),
        ],
        programId,
    )
};

export type WalletRelationshipSeeds = {
    mint: PublicKey, 
    walletA: PublicKey, 
    walletB: PublicKey, 
};

export const deriveWalletRelationshipPDA = (
    seeds: WalletRelationshipSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("relationship"),
            seeds.mint.toBuffer(),
            seeds.walletA.toBuffer(),
            seeds.walletB.toBuffer(),
        ],
        programId,
    )
};

export module CslSplTokenPDAs {
    export type AccountSeeds = {
        wallet: PublicKey, 
        tokenProgram: PublicKey, 
        mint: PublicKey, 
    };
    
    export const deriveAccountPDA = (
        seeds: AccountSeeds,
        programId: PublicKey
    ): [PublicKey, number] => {
        return PublicKey.findProgramAddressSync(
            [
                seeds.wallet.toBuffer(),
                seeds.tokenProgram.toBuffer(),
                seeds.mint.toBuffer(),
            ],
            programId,
        )
    };
    
}

