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
      // In a real implementation, you would fetch tokens from the blockchain
      // For now, we'll just use a placeholder
      setTokens([]);
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
        // In a real implementation, you would deserialize the account data
        // and check if the current wallet is the authority
        // For now, we'll just set isAdmin to false
        setIsAdmin(false);
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
                    value={selectedToken?.mint || ''}
                    onChange={(e) => {
                      const selected = tokens.find(token => token.mint === e.target.value);
                      setSelectedToken(selected || null);
                    }}
                    className="w-full p-2 border rounded"
                  >
                    <option value="">-- Select a token --</option>
                    {tokens.map((token) => (
                      <option key={token.mint} value={token.mint}>
                        {token.name} ({token.symbol})
                      </option>
                    ))}
                  </select>
                </div>
                
                {selectedToken && (
                  <div className="mb-4 p-3 bg-gray-100 rounded-md">
                    <p><strong>Current Price:</strong> {selectedToken.currentPrice} lamports</p>
                    <p><strong>Supply:</strong> {selectedToken.supply}</p>
                    <p><strong>Market Cap:</strong> {selectedToken.currentPrice * selectedToken.supply} lamports</p>
                  </div>
                )}
                
                <div className="mb-4">
                  <label className="block text-sm font-medium mb-1">Amount</label>
                  <input
                    type="number"
                    value={amount}
                    onChange={(e) => setAmount(e.target.value)}
                    className="w-full p-2 border rounded"
                    disabled={!selectedToken}
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