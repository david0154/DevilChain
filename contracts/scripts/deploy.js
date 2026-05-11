const hre = require("hardhat");

async function main() {
  console.log("\n🔥 Deploying DevilChain Smart Contracts...");

  // Deploy DevilCoin
  const DevilCoin = await hre.ethers.getContractFactory("DevilCoin");
  const dvc = await DevilCoin.deploy(hre.ethers.parseEther("1000000000")); // 1B supply
  await dvc.waitForDeployment();
  console.log(`✅ DevilCoin (DVL) deployed to: ${await dvc.getAddress()}`);

  // Deploy Staking
  const DevilStaking = await hre.ethers.getContractFactory("DevilStaking");
  const staking = await DevilStaking.deploy(await dvc.getAddress());
  await staking.waitForDeployment();
  console.log(`✅ DevilStaking deployed to: ${await staking.getAddress()}`);

  // Deploy DAO
  const DevilDAO = await hre.ethers.getContractFactory("DevilDAO");
  const dao = await DevilDAO.deploy(await staking.getAddress());
  await dao.waitForDeployment();
  console.log(`✅ DevilDAO deployed to: ${await dao.getAddress()}`);

  // Deploy NFT
  const DevilNFT = await hre.ethers.getContractFactory("DevilNFT");
  const nft = await DevilNFT.deploy();
  await nft.waitForDeployment();
  console.log(`✅ DevilNFT deployed to: ${await nft.getAddress()}`);

  console.log("\n🚀 All contracts deployed to DevilChain Network!");
}

main().catch((e) => { console.error(e); process.exit(1); });
