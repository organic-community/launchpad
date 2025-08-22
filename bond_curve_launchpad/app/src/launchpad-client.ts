import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { BondCurveLaunchpad } from '../target/types/bond_curve_launchpad';
import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
  sendAndConfirmTransaction,
  Connection,
} from '@solana/web3.js';
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddress,
  createMintToInstruction,
} from '@solana/spl-token';
import { TOKEN_2022_PROGRAM_ID } from '@solana/spl-token-2022';

export class LaunchpadClient {
  private program: Program<BondCurveLaunchpad>;
  private connection: Connection;
  private wallet: anchor.Wallet;

  constructor(
    connection: Connection,
    wallet: anchor.Wallet,
    programId: PublicKey
  ) {
    this.connection = connection;
    this.wallet = wallet;

    const provider = new anchor.AnchorProvider(
      connection,
      wallet,
      { commitment: 'confirmed' }
    );

    this.program = new Program<BondCurveLaunchpad>(
      require('../target/idl/bond_curve_launchpad.json'),
      programId,
      provider
    );
  }

  async findConfigPDA(): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddressSync(
      [Buffer.from('config')],
      this.program.programId
    );
  }

  async findFeeVaultPDA(): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddressSync(
      [Buffer.from('fee_vault')],
      this.program.programId
    );
  }

  async findProjectPDA(mint: PublicKey): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddressSync(
      [Buffer.from('project'), mint.toBuffer()],
      this.program.programId
    );
  }

  async findBundleTrackerPDA(mint: PublicKey, wallet: PublicKey): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddressSync(
      [Buffer.from('bundle'), mint.toBuffer(), wallet.toBuffer()],
      this.program.programId
    );
  }

  async findWalletRelationshipPDA(
    mint: PublicKey,
    walletA: PublicKey,
    walletB: PublicKey
  ): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddressSync(
      [
        Buffer.from('relationship'),
        mint.toBuffer(),
        walletA.toBuffer(),
        walletB.toBuffer(),
      ],
      this.program.programId
    );
  }

  async initializeLaunchpad(
    feeRecipient: PublicKey,
    bundleThresholdPercentage: number,
    graduationMarketCap: anchor.BN
  ): Promise<string> {
    const [configPDA] = await this.findConfigPDA();
    const [feeVaultPDA] = await this.findFeeVaultPDA();

    const tx = await this.program.methods
      .initializeLaunchpad(
        feeRecipient,
        bundleThresholdPercentage,
        graduationMarketCap
      )
      .accounts({
        config: configPDA,
        feeVault: feeVaultPDA,
        authority: this.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    return tx;
  }

  async createTokenProject(
    name: string,
    symbol: string,
    initialPrice: anchor.BN,
    curveParams: anchor.BN[]
  ): Promise<{ txId: string; mint: PublicKey }> {
    const [configPDA] = await this.findConfigPDA();
    const mintKeypair = Keypair.generate();
    const [projectPDA] = await this.findProjectPDA(mintKeypair.publicKey);

    const tx = await this.program.methods
      .createTokenProject(
        name,
        symbol,
        initialPrice,
        curveParams
      )
      .accounts({
        config: configPDA,
        project: projectPDA,
        mint: mintKeypair.publicKey,
        creator: this.wallet.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .signers([mintKeypair])
      .rpc();

    return {
      txId: tx,
      mint: mintKeypair.publicKey,
    };
  }

  async buyTokens(
    mint: PublicKey,
    amount: anchor.BN
  ): Promise<string> {
    const [configPDA] = await this.findConfigPDA();
    const [projectPDA] = await this.findProjectPDA(mint);
    const [feeVaultPDA] = await this.findFeeVaultPDA();

    // Get the associated token account for the buyer
    const buyerATA = await getAssociatedTokenAddress(
      mint,
      this.wallet.publicKey,
      true,
      TOKEN_2022_PROGRAM_ID
    );

    // Check if the ATA exists, if not create it
    const accountInfo = await this.connection.getAccountInfo(buyerATA);
    let tx = new Transaction();

    if (!accountInfo) {
      tx.add(
        createAssociatedTokenAccountInstruction(
          this.wallet.publicKey,
          buyerATA,
          this.wallet.publicKey,
          mint,
          TOKEN_2022_PROGRAM_ID,
          ASSOCIATED_TOKEN_PROGRAM_ID
        )
      );
    }

    // Add the buy tokens instruction
    tx.add(
      await this.program.methods
        .buyTokens(amount)
        .accounts({
          config: configPDA,
          project: projectPDA,
          mint: mint,
          buyer: this.wallet.publicKey,
          feeVault: feeVaultPDA,
          systemProgram: SystemProgram.programId,
        })
        .instruction()
    );

    // Send the transaction
    const txId = await sendAndConfirmTransaction(
      this.connection,
      tx,
      [this.wallet.payer],
      { commitment: 'confirmed' }
    );

    return txId;
  }

  async sellTokens(
    mint: PublicKey,
    amount: anchor.BN
  ): Promise<string> {
    const [configPDA] = await this.findConfigPDA();
    const [projectPDA] = await this.findProjectPDA(mint);
    const [feeVaultPDA] = await this.findFeeVaultPDA();

    // Get the associated token account for the seller
    const sellerATA = await getAssociatedTokenAddress(
      mint,
      this.wallet.publicKey,
      true,
      TOKEN_2022_PROGRAM_ID
    );

    // Add the sell tokens instruction
    const tx = await this.program.methods
      .sellTokens(amount)
      .accounts({
        config: configPDA,
        project: projectPDA,
        mint: mint,
        seller: this.wallet.publicKey,
        feeVault: feeVaultPDA,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    return tx;
  }

  async registerWalletRelationship(
    mint: PublicKey,
    walletA: PublicKey,
    walletB: PublicKey,
    relationshipStrength: number
  ): Promise<string> {
    const [configPDA] = await this.findConfigPDA();
    const [relationshipPDA] = await this.findWalletRelationshipPDA(
      mint,
      walletA,
      walletB
    );

    const tx = await this.program.methods
      .registerWalletRelationship(
        mint,
        walletA,
        walletB,
        relationshipStrength
      )
      .accounts({
        config: configPDA,
        relationship: relationshipPDA,
        authority: this.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    return tx;
  }

  async updateBundleStatus(
    mint: PublicKey,
    wallet: PublicKey,
    relatedWallets: PublicKey[],
    totalBundleBalance: anchor.BN
  ): Promise<string> {
    const [configPDA] = await this.findConfigPDA();
    const [bundleTrackerPDA] = await this.findBundleTrackerPDA(mint, wallet);

    const tx = await this.program.methods
      .updateBundleStatus(
        mint,
        wallet,
        relatedWallets,
        totalBundleBalance
      )
      .accounts({
        config: configPDA,
        bundleTracker: bundleTrackerPDA,
        authority: this.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    return tx;
  }

  async graduateToken(
    mint: PublicKey,
    liquidityPool: PublicKey
  ): Promise<string> {
    const [configPDA] = await this.findConfigPDA();
    const [projectPDA] = await this.findProjectPDA(mint);

    const tx = await this.program.methods
      .graduateToken(
        mint,
        liquidityPool
      )
      .accounts({
        config: configPDA,
        project: projectPDA,
        authority: this.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    return tx;
  }

  async withdrawPlatformFees(
    amount: anchor.BN,
    recipient: PublicKey
  ): Promise<string> {
    const [configPDA] = await this.findConfigPDA();
    const [feeVaultPDA] = await this.findFeeVaultPDA();

    const tx = await this.program.methods
      .withdrawPlatformFees(amount)
      .accounts({
        config: configPDA,
        feeVault: feeVaultPDA,
        authority: this.wallet.publicKey,
        recipient: recipient,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    return tx;
  }

  // Helper method for bond curve calculations
  calculateBuyPrice(
    curveParams: anchor.BN[],
    currentSupply: anchor.BN,
    amount: anchor.BN
  ): anchor.BN {
    // Exponential curve calculation
    const base = curveParams[0];
    const initialPrice = curveParams[1];
    
    // This is a very simplified calculation for demonstration
    // In a real implementation, you would need a more sophisticated approach
    let startPrice = initialPrice;
    for (let i = 0; i < currentSupply.toNumber(); i++) {
      startPrice = startPrice.mul(base).div(new anchor.BN(10000));
    }
    
    let endPrice = initialPrice;
    for (let i = 0; i < currentSupply.add(amount).toNumber(); i++) {
      endPrice = endPrice.mul(base).div(new anchor.BN(10000));
    }
    
    const avgPrice = startPrice.add(endPrice).div(new anchor.BN(2));
    return avgPrice.mul(amount);
  }

  calculateSellPrice(
    curveParams: anchor.BN[],
    currentSupply: anchor.BN,
    amount: anchor.BN
  ): anchor.BN {
    // Exponential curve calculation
    const base = curveParams[0];
    const initialPrice = curveParams[1];
    
    // This is a very simplified calculation for demonstration
    // In a real implementation, you would need a more sophisticated approach
    let startPrice = initialPrice;
    for (let i = 0; i < currentSupply.sub(amount).toNumber(); i++) {
      startPrice = startPrice.mul(base).div(new anchor.BN(10000));
    }
    
    let endPrice = initialPrice;
    for (let i = 0; i < currentSupply.toNumber(); i++) {
      endPrice = endPrice.mul(base).div(new anchor.BN(10000));
    }
    
    const avgPrice = startPrice.add(endPrice).div(new anchor.BN(2));
    return avgPrice.mul(amount);
  }
}