import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor';
import { LaunchpadClient } from './launchpad-client';
import { BN } from 'bn.js';

// Initialize connection to Solana devnet
const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
const programId = new PublicKey('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS'); // Replace with your program ID

const TokenLaunchpad: React.FC = () => {
  const { publicKey, signTransaction, signAllTransactions } = useWallet();
  const [client, setClient] = useState<LaunchpadClient | null>(null);
  const [tokens, setTokens] = useState<any[]>([]);
  const [selectedToken, setSelectedToken] = useState<any | null>(null);
  const [amount, setAmount] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  
  // Form states for creating a new token
  const [newTokenName, setNewTokenName] = useState('');
  const [newTokenSymbol, setNewTokenSymbol] = useState('');
  const [newTokenInitialPrice, setNewTokenInitialPrice] = useState('');
  const [newTokenCurveParams, setNewTokenCurveParams] = useState('');
  
  // Admin panel states
  const [withdrawAmount, setWithdrawAmount] = useState('');
  const [withdrawRecipient, setWithdrawRecipient] = useState('');
  const [isAdmin, setIsAdmin] = useState(false);
  const [feeVaultBalance, setFeeVaultBalance] = useState(0);
  
  // Bundle detection states
  const [bundleStatus, setBundleStatus] = useState<{ [key: string]: boolean }>({});
  const [relatedWallets, setRelatedWallets] = useState<{ [key: string]: PublicKey[] }>({});
  
  // Graduation states
  const [graduationStatus, setGraduationStatus] = useState<{ [key: string]: boolean }>({});

  useEffect(() => {
    if (publicKey && signTransaction && signAllTransactions) {
      const wallet = {
        publicKey,
        signTransaction,
        signAllTransactions,
      } as anchor.Wallet;
      
      const newClient = new LaunchpadClient(connection, wallet, programId);
      setClient(newClient);
      
      // Load tokens from the launchpad
      loadTokens();
      
      // Check if current wallet is admin
      checkIfAdmin();
      
      // Load fee vault balance
      loadFeeVaultBalance();
    } else {
      setClient(null);
    }
  }, [publicKey, signTransaction, signAllTransactions]);

  const loadTokens = async () => {
    if (!client) return;
    
    setLoading(true);
    try {
      // Fetch all token projects
      const projects = await client.getAllTokenProjects();
      setTokens(projects);
      
      // Check bundle status for each token
      const bundleStatusMap: { [key: string]: boolean } = {};
      const relatedWalletsMap: { [key: string]: PublicKey[] } = {};
      const graduationStatusMap: { [key: string]: boolean } = {};
      
      for (const project of projects) {
        if (publicKey) {
          // Check if the current wallet is bundling for this token
          bundleStatusMap[project.mint.toString()] = await client.checkBundlingStatus(
            project.mint,
            publicKey
          );
          
          // Get related wallets for this token
          relatedWalletsMap[project.mint.toString()] = await client.getRelatedWallets(
            project.mint,
            publicKey
          );
          
          // Check if the token is eligible for graduation
          graduationStatusMap[project.mint.toString()] = await client.isEligibleForGraduation(
            project.mint
          );
        }
      }
      
      setBundleStatus(bundleStatusMap);
      setRelatedWallets(relatedWalletsMap);
      setGraduationStatus(graduationStatusMap);
    } catch (err) {
      console.error('Error loading tokens:', err);
      setError('Failed to load tokens');
    } finally {
      setLoading(false);
    }
  };

  const checkIfAdmin = async () => {
    if (!client || !publicKey) return;
    
    try {
      const [configPDA] = await client.findConfigPDA();
      const configInfo = await connection.getAccountInfo(configPDA);
      
      if (configInfo) {
        // Deserialize the account data to check if the current wallet is the authority
        const config = await client.program.account.launchpadConfig.fetch(configPDA);
        setIsAdmin(config.authority.equals(publicKey));
      }
    } catch (err) {
      console.error('Error checking admin status:', err);
    }
  };

  const loadFeeVaultBalance = async () => {
    if (!client) return;
    
    try {
      const [feeVaultPDA] = await client.findFeeVaultPDA();
      const feeVaultInfo = await connection.getAccountInfo(feeVaultPDA);
      
      if (feeVaultInfo) {
        setFeeVaultBalance(feeVaultInfo.lamports);
      }
    } catch (err) {
      console.error('Error loading fee vault balance:', err);
    }
  };

  const handleCreateToken = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!client) return;
    
    setLoading(true);
    setError('');
    setSuccess('');
    
    try {
      // Parse curve parameters
      const curveParamsArray = newTokenCurveParams.split(',').map(param => new BN(param.trim()));
      
      // Create the token
      const result = await client.createTokenProject(
        newTokenName,
        newTokenSymbol,
        new BN(newTokenInitialPrice),
        curveParamsArray
      );
      
      setSuccess(`Token created successfully! Mint: ${result.mint.toString()}`);
      
      // Reset form
      setNewTokenName('');
      setNewTokenSymbol('');
      setNewTokenInitialPrice('');
      setNewTokenCurveParams('');
      
      // Reload tokens
      loadTokens();
    } catch (err) {
      console.error('Error creating token:', err);
      setError('Failed to create token');
    } finally {
      setLoading(false);
    }
  };

  const handleBuyTokens = async () => {
    if (!client || !selectedToken) return;
    
    setLoading(true);
    setError('');
    setSuccess('');
    
    try {
      const txId = await client.buyTokens(
        new PublicKey(selectedToken.mint),
        new BN(amount)
      );
      
      setSuccess(`Tokens purchased successfully! Transaction ID: ${txId}`);
      
      // Reload tokens
      loadTokens();
    } catch (err) {
      console.error('Error buying tokens:', err);
      setError('Failed to buy tokens');
    } finally {
      setLoading(false);
    }
  };

  const handleSellTokens = async () => {
    if (!client || !selectedToken) return;
    
    setLoading(true);
    setError('');
    setSuccess('');
    
    try {
      const txId = await client.sellTokens(
        new PublicKey(selectedToken.mint),
        new BN(amount)
      );
      
      setSuccess(`Tokens sold successfully! Transaction ID: ${txId}`);
      
      // Reload tokens
      loadTokens();
    } catch (err) {
      console.error('Error selling tokens:', err);
      setError('Failed to sell tokens');
    } finally {
      setLoading(false);
    }
  };

  const handleWithdrawFees = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!client) return;
    
    setLoading(true);
    setError('');
    setSuccess('');
    
    try {
      const txId = await client.withdrawPlatformFees(
        new BN(withdrawAmount),
        new PublicKey(withdrawRecipient)
      );
      
      setSuccess(`Fees withdrawn successfully! Transaction ID: ${txId}`);
      
      // Reset form
      setWithdrawAmount('');
      setWithdrawRecipient('');
      
      // Reload fee vault balance
      loadFeeVaultBalance();
    } catch (err) {
      console.error('Error withdrawing fees:', err);
      setError('Failed to withdraw fees');
    } finally {
      setLoading(false);
    }
  };

  const handleGraduateToken = async (mint: PublicKey) => {
    if (!client) return;
    
    setLoading(true);
    setError('');
    setSuccess('');
    
    try {
      // Create a new liquidity pool keypair
      const liquidityPoolKeypair = anchor.web3.Keypair.generate();
      
      // Graduate the token
      const txId = await client.createRaydiumPool(
        mint,
        new BN(1000000000), // 1 SOL initial liquidity
        new BN(1000000000)  // Initial token amount
      );
      
      setSuccess(`Token graduated successfully! Transaction ID: ${txId}`);
      
      // Reload tokens
      loadTokens();
    } catch (err) {
      console.error('Error graduating token:', err);
      setError('Failed to graduate token');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="container mx-auto p-4">
      <h1 className="text-3xl font-bold mb-6">Bond Curve Launchpad</h1>
      
      <div className="mb-6">
        <WalletMultiButton />
      </div>
      
      {!publicKey ? (
        <div className="bg-yellow-100 p-4 rounded-md">
          Please connect your wallet to use the launchpad.
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {/* Create Token Form */}
          <div className="bg-white p-6 rounded-lg shadow-md">
            <h2 className="text-xl font-semibold mb-4">Create New Token</h2>
            <form onSubmit={handleCreateToken}>
              <div className="mb-4">
                <label className="block text-sm font-medium mb-1">Token Name</label>
                <input
                  type="text"
                  value={newTokenName}
                  onChange={(e) => setNewTokenName(e.target.value)}
                  className="w-full p-2 border rounded"
                  required
                />
              </div>
              
              <div className="mb-4">
                <label className="block text-sm font-medium mb-1">Token Symbol</label>
                <input
                  type="text"
                  value={newTokenSymbol}
                  onChange={(e) => setNewTokenSymbol(e.target.value)}
                  className="w-full p-2 border rounded"
                  required
                />
              </div>
              
              <div className="mb-4">
                <label className="block text-sm font-medium mb-1">Initial Price (lamports)</label>
                <input
                  type="number"
                  value={newTokenInitialPrice}
                  onChange={(e) => setNewTokenInitialPrice(e.target.value)}
                  className="w-full p-2 border rounded"
                  required
                />
              </div>
              
              <div className="mb-4">
                <label className="block text-sm font-medium mb-1">
                  Curve Parameters (comma-separated)
                  <span className="text-xs text-gray-500 ml-2">
                    Format: base,initial_price
                  </span>
                </label>
                <input
                  type="text"
                  value={newTokenCurveParams}
                  onChange={(e) => setNewTokenCurveParams(e.target.value)}
                  className="w-full p-2 border rounded"
                  placeholder="10100,1000000"
                  required
                />
                <p className="text-xs text-gray-500 mt-1">
                  Base is in basis points (10000 = 1.0). For example, 10100 means 1.01x price increase per token.
                </p>
              </div>
              
              <button
                type="submit"
                className="bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-600 disabled:bg-gray-400"
                disabled={loading}
              >
                {loading ? 'Creating...' : 'Create Token'}
              </button>
            </form>
          </div>
          
          {/* Buy/Sell Tokens */}
          <div className="bg-white p-6 rounded-lg shadow-md">
            <h2 className="text-xl font-semibold mb-4">Buy/Sell Tokens</h2>
            
            <div className="mb-4 p-3 bg-blue-50 rounded-md">
              <p className="text-sm">
                <strong>Fee Structure:</strong>
              </p>
              <ul className="list-disc pl-5 text-sm">
                <li>1% trading fee on the launchpad (0.5% to creator, 0.5% to platform)</li>
                <li>2% fee on external swaps (outside the launchpad)</li>
                <li>Anti-bundling mechanism: 100% tax on transfers from wallets holding &gt;5% collectively</li>
              </ul>
            </div>
            
            {tokens.length === 0 ? (
              <div className="bg-gray-100 p-4 rounded-md">
                No tokens available. Create a new token to get started.
              </div>
            ) : (
              <>
                <div className="mb-4">
                  <label className="block text-sm font-medium mb-1">Select Token</label>
                  <select
                    value={selectedToken?.mint.toString() || ''}
                    onChange={(e) => {
                      const selected = tokens.find(token => token.mint.toString() === e.target.value);
                      setSelectedToken(selected || null);
                    }}
                    className="w-full p-2 border rounded"
                  >
                    <option value="">-- Select a token --</option>
                    {tokens.map((token) => (
                      <option key={token.mint.toString()} value={token.mint.toString()}>
                        {token.name} ({token.symbol})
                      </option>
                    ))}
                  </select>
                </div>
                
                {selectedToken && (
                  <>
                    <div className="mb-4 p-3 bg-gray-100 rounded-md">
                      <p><strong>Current Price:</strong> {selectedToken.currentPrice.toString()} lamports</p>
                      <p><strong>Supply:</strong> {selectedToken.supply.toString()}</p>
                      <p><strong>Market Cap:</strong> {new BN(selectedToken.currentPrice).mul(new BN(selectedToken.supply)).toString()} lamports</p>
                      <p><strong>Graduated:</strong> {selectedToken.isGraduated ? 'Yes' : 'No'}</p>
                      
                      {/* Bundle Status */}
                      {bundleStatus[selectedToken.mint.toString()] !== undefined && (
                        <div className={`mt-2 p-2 rounded ${bundleStatus[selectedToken.mint.toString()] ? 'bg-red-100' : 'bg-green-100'}`}>
                          <p>
                            <strong>Bundle Status:</strong> 
                            {bundleStatus[selectedToken.mint.toString()] 
                              ? ' Bundling Detected (100% tax on transfers)' 
                              : ' No Bundling Detected'}
                          </p>
                          
                          {relatedWallets[selectedToken.mint.toString()] && 
                           relatedWallets[selectedToken.mint.toString()].length > 0 && (
                            <div className="mt-1">
                              <p><strong>Related Wallets:</strong> {relatedWallets[selectedToken.mint.toString()].length}</p>
                              <div className="text-xs mt-1 max-h-20 overflow-y-auto">
                                {relatedWallets[selectedToken.mint.toString()].map((wallet, index) => (
                                  <p key={index}>{wallet.toString().substring(0, 8)}...{wallet.toString().substring(wallet.toString().length - 8)}</p>
                                ))}
                              </div>
                            </div>
                          )}
                        </div>
                      )}
                      
                      {/* Graduation Status */}
                      {graduationStatus[selectedToken.mint.toString()] !== undefined && !selectedToken.isGraduated && (
                        <div className={`mt-2 p-2 rounded ${graduationStatus[selectedToken.mint.toString()] ? 'bg-yellow-100' : 'bg-gray-200'}`}>
                          <p>
                            <strong>Graduation Status:</strong> 
                            {graduationStatus[selectedToken.mint.toString()] 
                              ? ' Eligible for Graduation' 
                              : ' Not Eligible for Graduation'}
                          </p>
                          
                          {isAdmin && graduationStatus[selectedToken.mint.toString()] && (
                            <button
                              onClick={() => handleGraduateToken(selectedToken.mint)}
                              className="mt-2 bg-yellow-500 text-white py-1 px-2 text-xs rounded hover:bg-yellow-600 disabled:bg-gray-400"
                              disabled={loading}
                            >
                              {loading ? 'Processing...' : 'Graduate to Raydium'}
                            </button>
                          )}
                        </div>
                      )}
                    </div>
                    
                    <div className="mb-4">
                      <label className="block text-sm font-medium mb-1">Amount</label>
                      <input
                        type="number"
                        value={amount}
                        onChange={(e) => setAmount(e.target.value)}
                        className="w-full p-2 border rounded"
                      />
                    </div>
                    
                    <div className="flex space-x-2">
                      <button
                        onClick={handleBuyTokens}
                        className="bg-green-500 text-white py-2 px-4 rounded hover:bg-green-600 disabled:bg-gray-400"
                        disabled={loading || !selectedToken || !amount}
                      >
                        {loading ? 'Processing...' : 'Buy Tokens'}
                      </button>
                      
                      <button
                        onClick={handleSellTokens}
                        className="bg-red-500 text-white py-2 px-4 rounded hover:bg-red-600 disabled:bg-gray-400"
                        disabled={loading || !selectedToken || !amount}
                      >
                        {loading ? 'Processing...' : 'Sell Tokens'}
                      </button>
                    </div>
                  </>
                )}
              </>
            )}
          </div>
          
          {/* Token List */}
          <div className="bg-white p-6 rounded-lg shadow-md col-span-2">
            <h2 className="text-xl font-semibold mb-4">Available Tokens</h2>
            
            {tokens.length === 0 ? (
              <div className="bg-gray-100 p-4 rounded-md">
                No tokens available. Create a new token to get started.
              </div>
            ) : (
              <div className="overflow-x-auto">
                <table className="min-w-full divide-y divide-gray-200">
                  <thead className="bg-gray-50">
                    <tr>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Name</th>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Symbol</th>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Price</th>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Supply</th>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Market Cap</th>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Status</th>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Bundle Status</th>
                    </tr>
                  </thead>
                  <tbody className="bg-white divide-y divide-gray-200">
                    {tokens.map((token) => (
                      <tr key={token.mint.toString()}>
                        <td className="px-6 py-4 whitespace-nowrap">{token.name}</td>
                        <td className="px-6 py-4 whitespace-nowrap">{token.symbol}</td>
                        <td className="px-6 py-4 whitespace-nowrap">{token.currentPrice.toString()} lamports</td>
                        <td className="px-6 py-4 whitespace-nowrap">{token.supply.toString()}</td>
                        <td className="px-6 py-4 whitespace-nowrap">
                          {new BN(token.currentPrice).mul(new BN(token.supply)).toString()} lamports
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap">
                          {token.isGraduated ? (
                            <span className="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800">
                              Graduated
                            </span>
                          ) : graduationStatus[token.mint.toString()] ? (
                            <span className="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-yellow-100 text-yellow-800">
                              Eligible
                            </span>
                          ) : (
                            <span className="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-gray-100 text-gray-800">
                              Launchpad
                            </span>
                          )}
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap">
                          {bundleStatus[token.mint.toString()] ? (
                            <span className="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-red-100 text-red-800">
                              Bundling
                            </span>
                          ) : (
                            <span className="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800">
                              Normal
                            </span>
                          )}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </div>
          
          {/* Admin Panel */}
          {isAdmin && (
            <div className="bg-white p-6 rounded-lg shadow-md col-span-2">
              <h2 className="text-xl font-semibold mb-4">Admin Panel</h2>
              
              <div className="mb-4 p-3 bg-gray-100 rounded-md">
                <p><strong>Fee Vault Balance:</strong> {feeVaultBalance / 1e9} SOL</p>
              </div>
              
              <form onSubmit={handleWithdrawFees} className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div>
                  <label className="block text-sm font-medium mb-1">Amount (lamports)</label>
                  <input
                    type="number"
                    value={withdrawAmount}
                    onChange={(e) => setWithdrawAmount(e.target.value)}
                    className="w-full p-2 border rounded"
                    required
                  />
                </div>
                
                <div>
                  <label className="block text-sm font-medium mb-1">Recipient</label>
                  <input
                    type="text"
                    value={withdrawRecipient}
                    onChange={(e) => setWithdrawRecipient(e.target.value)}
                    className="w-full p-2 border rounded"
                    placeholder="Recipient public key"
                    required
                  />
                </div>
                
                <div className="flex items-end">
                  <button
                    type="submit"
                    className="bg-purple-500 text-white py-2 px-4 rounded hover:bg-purple-600 disabled:bg-gray-400"
                    disabled={loading}
                  >
                    {loading ? 'Processing...' : 'Withdraw Fees'}
                  </button>
                </div>
              </form>
            </div>
          )}
        </div>
      )}
      
      {error && (
        <div className="mt-4 p-3 bg-red-100 text-red-700 rounded-md">
          {error}
        </div>
      )}
      
      {success && (
        <div className="mt-4 p-3 bg-green-100 text-green-700 rounded-md">
          {success}
        </div>
      )}
    </div>
  );
};

export default TokenLaunchpad;