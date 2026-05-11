const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("DevilStaking", function () {
  let dvc, staking, owner, alice, bob;
  const MIN_STAKE = ethers.parseEther("100");

  beforeEach(async function () {
    [owner, alice, bob] = await ethers.getSigners();
    const DevilCoin = await ethers.getContractFactory("DevilCoin");
    dvc = await DevilCoin.deploy(ethers.parseEther("1000000000"));

    const DevilStaking = await ethers.getContractFactory("DevilStaking");
    staking = await DevilStaking.deploy(await dvc.getAddress());

    // Fund alice with 1000 DVC
    await dvc.transfer(alice.address, ethers.parseEther("1000"));
    await dvc.connect(alice).approve(await staking.getAddress(), ethers.parseEther("1000"));
  });

  it("should allow staking above minimum", async function () {
    await staking.connect(alice).stake(MIN_STAKE);
    const v = await staking.validators(alice.address);
    expect(v.staked).to.equal(MIN_STAKE);
    expect(v.active).to.equal(true);
  });

  it("should reject stake below minimum", async function () {
    await expect(
      staking.connect(alice).stake(ethers.parseEther("10"))
    ).to.be.revertedWith("Below minimum stake");
  });

  it("should calculate voting power correctly", async function () {
    await staking.connect(alice).stake(MIN_STAKE);
    const power = await staking.votingPower(alice.address);
    expect(power).to.equal(MIN_STAKE); // reputation=0 initially
  });

  it("should allow unstaking", async function () {
    await staking.connect(alice).stake(MIN_STAKE);
    await staking.connect(alice).unstake(MIN_STAKE);
    const v = await staking.validators(alice.address);
    expect(v.active).to.equal(false);
  });

  it("should list validators", async function () {
    await staking.connect(alice).stake(MIN_STAKE);
    const validators = await staking.getValidators();
    expect(validators).to.include(alice.address);
  });
});
