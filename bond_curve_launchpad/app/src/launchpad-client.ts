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
        authority: this.wallet.publicKey,
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
    const [bundleTrackerPDA] = await this.findBundleTrackerPDA(mint, this.wallet.publicKey);

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
          buyerTokenAccount: buyerATA,
          feeVault: feeVaultPDA,
          bundleTracker: bundleTrackerPDA,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
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
    const [bundleTrackerPDA] = await this.findBundleTrackerPDA(mint, this.wallet.publicKey);

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
        sellerTokenAccount: sellerATA,
        feeVault: feeVaultPDA,
        bundleTracker: bundleTrackerPDA,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .rpc();

    return tx;
  }

  async registerWalletRelationship(
    mint: PublicKey,
    walletA: PublicKey,
    walletB: PublicKey,
    relationshipStrength: number,
    transactionCount: number
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
        relationshipStrength,
        transactionCount
      )
      .accounts({
        config: configPDA,
        relationship: relationshipPDA,
        mint: mint,
        walletA: walletA,
        walletB: walletB,
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
    const [projectPDA] = await this.findProjectPDA(mint);
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
        project: projectPDA,
        bundleTracker: bundleTrackerPDA,
        mint: mint,
        wallet: wallet,
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
        mint: mint,
        liquidityPool: liquidityPool,
        authority: this.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    return tx;
  }

  async createRaydiumPool(
    mint: PublicKey,
    initialLiquidityAmount: anchor.BN,
    initialTokenAmount: anchor.BN
  ): Promise<string> {
    const [configPDA] = await this.findConfigPDA();
    const [projectPDA] = await this.findProjectPDA(mint);
    const liquidityPoolKeypair = Keypair.generate();

    const tx = await this.program.methods
      .createRaydiumPool(
        mint,
        initialLiquidityAmount,
        initialTokenAmount
      )
      .accounts({
        config: configPDA,
        project: projectPDA,
        mint: mint,
        authority: this.wallet.publicKey,
        liquidityPool: liquidityPoolKeypair.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .signers([liquidityPoolKeypair])
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

  // Helper method to check if a wallet is bundling
  async checkBundlingStatus(
    mint: PublicKey,
    wallet: PublicKey
  ): Promise<boolean> {
    const [bundleTrackerPDA] = await this.findBundleTrackerPDA(mint, wallet);
    
    try {
      const bundleTracker = await this.program.account.bundleTracker.fetch(bundleTrackerPDA);
      return bundleTracker.isBundling;
    } catch (error) {
      // If the account doesn't exist, the wallet is not bundling
      return false;
    }
  }

  // Helper method to get all related wallets for a given wallet
  async getRelatedWallets(
    mint: PublicKey,
    wallet: PublicKey
  ): Promise<PublicKey[]> {
    const [bundleTrackerPDA] = await this.findBundleTrackerPDA(mint, wallet);
    
    try {
      const bundleTracker = await this.program.account.bundleTracker.fetch(bundleTrackerPDA);
      return bundleTracker.relatedWallets;
    } catch (error) {
      // If the account doesn't exist, there are no related wallets
      return [];
    }
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

  // Helper method to calculate market cap
  calculateMarketCap(
    supply: anchor.BN,
    currentPrice: anchor.BN
  ): anchor.BN {
    return supply.mul(currentPrice);
  }

  // Helper method to check if a token is eligible for graduation
  async isEligibleForGraduation(
    mint: PublicKey
  ): Promise<boolean> {
    const [configPDA] = await this.findConfigPDA();
    const [projectPDA] = await this.findProjectPDA(mint);
    
    try {
      const config = await this.program.account.launchpadConfig.fetch(configPDA);
      const project = await this.program.account.tokenProject.fetch(projectPDA);
      
      const marketCap = new anchor.BN(project.supply).mul(new anchor.BN(project.currentPrice));
      return marketCap.gte(new anchor.BN(config.graduationMarketCap));
    } catch (error) {
      return false;
    }
  }

  // Helper method to get all token projects
  async getAllTokenProjects(): Promise<any[]> {
    const projects = await this.program.account.tokenProject.all();
    return projects.map(project => ({
      mint: project.account.mint,
      name: project.account.name,
      symbol: project.account.symbol,
      currentPrice: project.account.currentPrice,
      supply: project.account.supply,
      isGraduated: project.account.isGraduated,
      liquidityPool: project.account.liquidityPool,
    }));
  }
}