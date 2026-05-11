const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("DevilStaking", function () {
  let coin, staking, owner, alice;

  beforeEach(async () => {
    [owner, alice] = await ethers.getSigners();
    const Coin = await ethers.getContractFactory("DevilCoin");
    coin = await Coin.deploy();
    await coin.waitForDeployment();

    const Staking = await ethers.getContractFactory("DevilStaking");
    staking = await Staking.deploy(await coin.getAddress());
    await staking.waitForDeployment();
  });

  it("Should allow staking", async () => {
    const amount = ethers.parseEther("200");
    await coin.transfer(alice.address, amount);
    await coin.connect(alice).approve(await staking.getAddress(), amount);
    await staking.connect(alice).stake(amount);
    const info = await staking.getStakeInfo(alice.address);
    expect(info.amount).to.equal(amount);
  });

  it("Should reject stake below minimum", async () => {
    const tiny = ethers.parseEther("0.001");
    await coin.transfer(alice.address, tiny);
    await coin.connect(alice).approve(await staking.getAddress(), tiny);
    await expect(staking.connect(alice).stake(tiny)).to.be.reverted;
  });

  it("Should allow unstake after lock period", async () => {
    const amount = ethers.parseEther("200");
    await coin.transfer(alice.address, amount);
    await coin.connect(alice).approve(await staking.getAddress(), amount);
    await staking.connect(alice).stake(amount);
    // Fast-forward time (Hardhat)
    await ethers.provider.send("evm_increaseTime", [7 * 24 * 3600]);
    await ethers.provider.send("evm_mine");
    await staking.connect(alice).unstake();
    expect(await coin.balanceOf(alice.address)).to.be.greaterThan(0n);
  });
});
