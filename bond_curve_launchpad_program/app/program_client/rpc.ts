import {
  AnchorProvider,
  BN,
  IdlAccounts,
  Program,
  web3,
} from "@coral-xyz/anchor";
import { MethodsBuilder } from "@coral-xyz/anchor/dist/cjs/program/namespace/methods";
import { BondCurveLaunchpad } from "../../target/types/bond_curve_launchpad";
import idl from "../../target/idl/bond_curve_launchpad.json";
import * as pda from "./pda";

import { CslSplToken } from "../../target/types/csl_spl_token";
import idlCslSplToken from "../../target/idl/csl_spl_token.json";



let _program: Program<BondCurveLaunchpad>;
let _programCslSplToken: Program<CslSplToken>;


export const initializeClient = (
    programId: web3.PublicKey,
    anchorProvider = AnchorProvider.env(),
) => {
    _program = new Program<BondCurveLaunchpad>(
        idl as never,
        programId,
        anchorProvider,
    );

    _programCslSplToken = new Program<CslSplToken>(
        idlCslSplToken as never,
        new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        anchorProvider,
    );

};

export type InitializeLaunchpadArgs = {
  authority: web3.PublicKey;
  feeRecipient: web3.PublicKey;
  feePercentage: number;
  bundleThresholdPercentage: number;
  graduationMarketCap: bigint;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Initialize the launchpad configuration
 *
 * Accounts:
 * 0. `[signer]` authority: {@link PublicKey} 
 * 1. `[writable]` config: {@link LaunchpadConfig} 
 * 2. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - fee_recipient: {@link PublicKey} 
 * - fee_percentage: {@link number} 
 * - bundle_threshold_percentage: {@link number} 
 * - graduation_market_cap: {@link BigInt} 
 */
export const initializeLaunchpadBuilder = (
	args: InitializeLaunchpadArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<BondCurveLaunchpad, never> => {
  const [configPubkey] = pda.deriveLaunchpadConfigPDA(_program.programId);

  return _program
    .methods
    .initializeLaunchpad(
      args.feeRecipient,
      args.feePercentage,
      args.bundleThresholdPercentage,
      new BN(args.graduationMarketCap.toString()),
    )
    .accountsStrict({
      authority: args.authority,
      config: configPubkey,
      systemProgram: new web3.PublicKey("11111111111111111111111111111111"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Initialize the launchpad configuration
 *
 * Accounts:
 * 0. `[signer]` authority: {@link PublicKey} 
 * 1. `[writable]` config: {@link LaunchpadConfig} 
 * 2. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - fee_recipient: {@link PublicKey} 
 * - fee_percentage: {@link number} 
 * - bundle_threshold_percentage: {@link number} 
 * - graduation_market_cap: {@link BigInt} 
 */
export const initializeLaunchpad = (
	args: InitializeLaunchpadArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    initializeLaunchpadBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Initialize the launchpad configuration
 *
 * Accounts:
 * 0. `[signer]` authority: {@link PublicKey} 
 * 1. `[writable]` config: {@link LaunchpadConfig} 
 * 2. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - fee_recipient: {@link PublicKey} 
 * - fee_percentage: {@link number} 
 * - bundle_threshold_percentage: {@link number} 
 * - graduation_market_cap: {@link BigInt} 
 */
export const initializeLaunchpadSendAndConfirm = async (
  args: Omit<InitializeLaunchpadArgs, "authority"> & {
    signers: {
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return initializeLaunchpadBuilder({
      ...args,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.authority])
    .rpc();
}

export type CreateTokenProjectArgs = {
  creator: web3.PublicKey;
  mint: web3.PublicKey;
  name: string;
  symbol: string;
  initialPrice: bigint;
  curveType: number;
  curveParams: bigint[];
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Create a new token project on the launchpad
 *
 * Accounts:
 * 0. `[signer]` creator: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` project: {@link TokenProject} 
 * 3. `[writable, signer]` mint: {@link Mint} 
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 * 5. `[]` csl_spl_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplTokenProgram v0.0.0
 *
 * Data:
 * - name: {@link string} 
 * - symbol: {@link string} 
 * - initial_price: {@link BigInt} 
 * - curve_type: {@link number} 
 * - curve_params: {@link BigInt[]} 
 */
export const createTokenProjectBuilder = (
	args: CreateTokenProjectArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<BondCurveLaunchpad, never> => {
  const [configPubkey] = pda.deriveLaunchpadConfigPDA(_program.programId);
    const [projectPubkey] = pda.deriveTokenProjectPDA({
        mint: args.mint,
    }, _program.programId);

  return _program
    .methods
    .createTokenProject(
      args.name,
      args.symbol,
      new BN(args.initialPrice.toString()),
      args.curveType,
      args.curveParams.map(e => new BN(e.toString())),
    )
    .accountsStrict({
      creator: args.creator,
      config: configPubkey,
      project: projectPubkey,
      mint: args.mint,
      systemProgram: new web3.PublicKey("11111111111111111111111111111111"),
      cslSplTokenV000: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Create a new token project on the launchpad
 *
 * Accounts:
 * 0. `[signer]` creator: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` project: {@link TokenProject} 
 * 3. `[writable, signer]` mint: {@link Mint} 
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 * 5. `[]` csl_spl_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplTokenProgram v0.0.0
 *
 * Data:
 * - name: {@link string} 
 * - symbol: {@link string} 
 * - initial_price: {@link BigInt} 
 * - curve_type: {@link number} 
 * - curve_params: {@link BigInt[]} 
 */
export const createTokenProject = (
	args: CreateTokenProjectArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    createTokenProjectBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Create a new token project on the launchpad
 *
 * Accounts:
 * 0. `[signer]` creator: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` project: {@link TokenProject} 
 * 3. `[writable, signer]` mint: {@link Mint} 
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 * 5. `[]` csl_spl_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplTokenProgram v0.0.0
 *
 * Data:
 * - name: {@link string} 
 * - symbol: {@link string} 
 * - initial_price: {@link BigInt} 
 * - curve_type: {@link number} 
 * - curve_params: {@link BigInt[]} 
 */
export const createTokenProjectSendAndConfirm = async (
  args: Omit<CreateTokenProjectArgs, "creator" | "mint"> & {
    signers: {
      creator: web3.Signer,
      mint: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return createTokenProjectBuilder({
      ...args,
      creator: args.signers.creator.publicKey,
      mint: args.signers.mint.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.creator, args.signers.mint])
    .rpc();
}

export type BuyTokensArgs = {
  buyer: web3.PublicKey;
  mint: web3.PublicKey;
  amount: bigint;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Buy tokens from the launchpad
 *
 * Accounts:
 * 0. `[writable, signer]` buyer: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` project: {@link TokenProject} 
 * 3. `[writable]` mint: {@link Mint} 
 *
 * Data:
 * - amount: {@link BigInt} Amount of tokens to buy
 */
export const buyTokensBuilder = (
	args: BuyTokensArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<BondCurveLaunchpad, never> => {
  const [configPubkey] = pda.deriveLaunchpadConfigPDA(_program.programId);
    const [projectPubkey] = pda.deriveTokenProjectPDA({
        mint: args.mint,
    }, _program.programId);

  return _program
    .methods
    .buyTokens(
      new BN(args.amount.toString()),
    )
    .accountsStrict({
      buyer: args.buyer,
      config: configPubkey,
      project: projectPubkey,
      mint: args.mint,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Buy tokens from the launchpad
 *
 * Accounts:
 * 0. `[writable, signer]` buyer: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` project: {@link TokenProject} 
 * 3. `[writable]` mint: {@link Mint} 
 *
 * Data:
 * - amount: {@link BigInt} Amount of tokens to buy
 */
export const buyTokens = (
	args: BuyTokensArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    buyTokensBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Buy tokens from the launchpad
 *
 * Accounts:
 * 0. `[writable, signer]` buyer: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` project: {@link TokenProject} 
 * 3. `[writable]` mint: {@link Mint} 
 *
 * Data:
 * - amount: {@link BigInt} Amount of tokens to buy
 */
export const buyTokensSendAndConfirm = async (
  args: Omit<BuyTokensArgs, "buyer"> & {
    signers: {
      buyer: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return buyTokensBuilder({
      ...args,
      buyer: args.signers.buyer.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.buyer])
    .rpc();
}

export type SellTokensArgs = {
  seller: web3.PublicKey;
  mint: web3.PublicKey;
  amount: bigint;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Sell tokens back to the launchpad
 *
 * Accounts:
 * 0. `[writable, signer]` seller: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` project: {@link TokenProject} 
 * 3. `[writable]` mint: {@link Mint} 
 *
 * Data:
 * - amount: {@link BigInt} Amount of tokens to sell
 */
export const sellTokensBuilder = (
	args: SellTokensArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<BondCurveLaunchpad, never> => {
  const [configPubkey] = pda.deriveLaunchpadConfigPDA(_program.programId);
    const [projectPubkey] = pda.deriveTokenProjectPDA({
        mint: args.mint,
    }, _program.programId);

  return _program
    .methods
    .sellTokens(
      new BN(args.amount.toString()),
    )
    .accountsStrict({
      seller: args.seller,
      config: configPubkey,
      project: projectPubkey,
      mint: args.mint,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Sell tokens back to the launchpad
 *
 * Accounts:
 * 0. `[writable, signer]` seller: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` project: {@link TokenProject} 
 * 3. `[writable]` mint: {@link Mint} 
 *
 * Data:
 * - amount: {@link BigInt} Amount of tokens to sell
 */
export const sellTokens = (
	args: SellTokensArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    sellTokensBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Sell tokens back to the launchpad
 *
 * Accounts:
 * 0. `[writable, signer]` seller: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` project: {@link TokenProject} 
 * 3. `[writable]` mint: {@link Mint} 
 *
 * Data:
 * - amount: {@link BigInt} Amount of tokens to sell
 */
export const sellTokensSendAndConfirm = async (
  args: Omit<SellTokensArgs, "seller"> & {
    signers: {
      seller: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return sellTokensBuilder({
      ...args,
      seller: args.signers.seller.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.seller])
    .rpc();
}

export type RegisterWalletRelationshipArgs = {
  authority: web3.PublicKey;
  mint: web3.PublicKey;
  walletA: web3.PublicKey;
  walletB: web3.PublicKey;
  relationshipStrength: number;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Register a relationship between two wallets
 *
 * Accounts:
 * 0. `[signer]` authority: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` relationship: {@link WalletRelationship} 
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - mint: {@link PublicKey} 
 * - wallet_a: {@link PublicKey} 
 * - wallet_b: {@link PublicKey} 
 * - relationship_strength: {@link number} 
 */
export const registerWalletRelationshipBuilder = (
	args: RegisterWalletRelationshipArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<BondCurveLaunchpad, never> => {
  const [configPubkey] = pda.deriveLaunchpadConfigPDA(_program.programId);
    const [relationshipPubkey] = pda.deriveWalletRelationshipPDA({
        mint: args.mint,
        walletA: args.walletA,
        walletB: args.walletB,
    }, _program.programId);

  return _program
    .methods
    .registerWalletRelationship(
      args.mint,
      args.walletA,
      args.walletB,
      args.relationshipStrength,
    )
    .accountsStrict({
      authority: args.authority,
      config: configPubkey,
      relationship: relationshipPubkey,
      systemProgram: new web3.PublicKey("11111111111111111111111111111111"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Register a relationship between two wallets
 *
 * Accounts:
 * 0. `[signer]` authority: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` relationship: {@link WalletRelationship} 
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - mint: {@link PublicKey} 
 * - wallet_a: {@link PublicKey} 
 * - wallet_b: {@link PublicKey} 
 * - relationship_strength: {@link number} 
 */
export const registerWalletRelationship = (
	args: RegisterWalletRelationshipArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    registerWalletRelationshipBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Register a relationship between two wallets
 *
 * Accounts:
 * 0. `[signer]` authority: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` relationship: {@link WalletRelationship} 
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - mint: {@link PublicKey} 
 * - wallet_a: {@link PublicKey} 
 * - wallet_b: {@link PublicKey} 
 * - relationship_strength: {@link number} 
 */
export const registerWalletRelationshipSendAndConfirm = async (
  args: Omit<RegisterWalletRelationshipArgs, "authority"> & {
    signers: {
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return registerWalletRelationshipBuilder({
      ...args,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.authority])
    .rpc();
}

export type UpdateBundleStatusArgs = {
  authority: web3.PublicKey;
  mint: web3.PublicKey;
  wallet: web3.PublicKey;
  relatedWallets: web3.PublicKey[];
  totalBundleBalance: bigint;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Update the bundling status of a wallet
 *
 * Accounts:
 * 0. `[signer]` authority: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` bundle_tracker: {@link BundleTracker} 
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - mint: {@link PublicKey} 
 * - wallet: {@link PublicKey} 
 * - related_wallets: {@link PublicKey[]} 
 * - total_bundle_balance: {@link BigInt} 
 */
export const updateBundleStatusBuilder = (
	args: UpdateBundleStatusArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<BondCurveLaunchpad, never> => {
  const [configPubkey] = pda.deriveLaunchpadConfigPDA(_program.programId);
    const [bundleTrackerPubkey] = pda.deriveBundleTrackerPDA({
        mint: args.mint,
        wallet: args.wallet,
    }, _program.programId);

  return _program
    .methods
    .updateBundleStatus(
      args.mint,
      args.wallet,
      args.relatedWallets,
      new BN(args.totalBundleBalance.toString()),
    )
    .accountsStrict({
      authority: args.authority,
      config: configPubkey,
      bundleTracker: bundleTrackerPubkey,
      systemProgram: new web3.PublicKey("11111111111111111111111111111111"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Update the bundling status of a wallet
 *
 * Accounts:
 * 0. `[signer]` authority: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` bundle_tracker: {@link BundleTracker} 
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - mint: {@link PublicKey} 
 * - wallet: {@link PublicKey} 
 * - related_wallets: {@link PublicKey[]} 
 * - total_bundle_balance: {@link BigInt} 
 */
export const updateBundleStatus = (
	args: UpdateBundleStatusArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    updateBundleStatusBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Update the bundling status of a wallet
 *
 * Accounts:
 * 0. `[signer]` authority: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` bundle_tracker: {@link BundleTracker} 
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - mint: {@link PublicKey} 
 * - wallet: {@link PublicKey} 
 * - related_wallets: {@link PublicKey[]} 
 * - total_bundle_balance: {@link BigInt} 
 */
export const updateBundleStatusSendAndConfirm = async (
  args: Omit<UpdateBundleStatusArgs, "authority"> & {
    signers: {
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return updateBundleStatusBuilder({
      ...args,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.authority])
    .rpc();
}

export type GraduateTokenArgs = {
  authority: web3.PublicKey;
  mint: web3.PublicKey;
  liquidityPool: web3.PublicKey;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Graduate a token from the launchpad to a liquidity pool
 *
 * Accounts:
 * 0. `[signer]` authority: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` project: {@link TokenProject} 
 *
 * Data:
 * - mint: {@link PublicKey} 
 * - liquidity_pool: {@link PublicKey} 
 */
export const graduateTokenBuilder = (
	args: GraduateTokenArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<BondCurveLaunchpad, never> => {
  const [configPubkey] = pda.deriveLaunchpadConfigPDA(_program.programId);
    const [projectPubkey] = pda.deriveTokenProjectPDA({
        mint: args.mint,
    }, _program.programId);

  return _program
    .methods
    .graduateToken(
      args.mint,
      args.liquidityPool,
    )
    .accountsStrict({
      authority: args.authority,
      config: configPubkey,
      project: projectPubkey,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Graduate a token from the launchpad to a liquidity pool
 *
 * Accounts:
 * 0. `[signer]` authority: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` project: {@link TokenProject} 
 *
 * Data:
 * - mint: {@link PublicKey} 
 * - liquidity_pool: {@link PublicKey} 
 */
export const graduateToken = (
	args: GraduateTokenArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    graduateTokenBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Graduate a token from the launchpad to a liquidity pool
 *
 * Accounts:
 * 0. `[signer]` authority: {@link PublicKey} 
 * 1. `[]` config: {@link LaunchpadConfig} 
 * 2. `[writable]` project: {@link TokenProject} 
 *
 * Data:
 * - mint: {@link PublicKey} 
 * - liquidity_pool: {@link PublicKey} 
 */
export const graduateTokenSendAndConfirm = async (
  args: Omit<GraduateTokenArgs, "authority"> & {
    signers: {
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return graduateTokenBuilder({
      ...args,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.authority])
    .rpc();
}

// Getters

export const getLaunchpadConfig = (
    publicKey: web3.PublicKey,
    commitment?: web3.Commitment
): Promise<IdlAccounts<BondCurveLaunchpad>["launchpadConfig"]> => _program.account.launchpadConfig.fetch(publicKey, commitment);

export const getTokenProject = (
    publicKey: web3.PublicKey,
    commitment?: web3.Commitment
): Promise<IdlAccounts<BondCurveLaunchpad>["tokenProject"]> => _program.account.tokenProject.fetch(publicKey, commitment);

export const getBundleTracker = (
    publicKey: web3.PublicKey,
    commitment?: web3.Commitment
): Promise<IdlAccounts<BondCurveLaunchpad>["bundleTracker"]> => _program.account.bundleTracker.fetch(publicKey, commitment);

export const getWalletRelationship = (
    publicKey: web3.PublicKey,
    commitment?: web3.Commitment
): Promise<IdlAccounts<BondCurveLaunchpad>["walletRelationship"]> => _program.account.walletRelationship.fetch(publicKey, commitment);
export module CslSplTokenGetters {
    export const getMint = (
        publicKey: web3.PublicKey,
        commitment?: web3.Commitment
    ): Promise<IdlAccounts<CslSplToken>["mint"]> => _programCslSplToken.account.mint.fetch(publicKey, commitment);
    
    export const getAccount = (
        publicKey: web3.PublicKey,
        commitment?: web3.Commitment
    ): Promise<IdlAccounts<CslSplToken>["account"]> => _programCslSplToken.account.account.fetch(publicKey, commitment);
}

