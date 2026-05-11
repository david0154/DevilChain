#!/bin/bash
# Deploy DevilChain Smart Contracts

set -e
cd contracts

echo "Installing contract dependencies..."
npm install

echo "Compiling contracts..."
npx hardhat compile

echo "Deploying to DevilChain testnet..."
npx hardhat run scripts/deploy.js --network devilchain_testnet

echo "Done! Check deployments above."
