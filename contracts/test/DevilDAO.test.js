const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("DevilDAO", function () {
  let dvc, staking, dao, owner, alice, bob;

  beforeEach(async function () {
    [owner, alice, bob] = await ethers.getSigners();

    const DevilCoin = await ethers.getContractFactory("DevilCoin");
    dvc = await DevilCoin.deploy(ethers.parseEther("1000000000"));

    const DevilStaking = await ethers.getContractFactory("DevilStaking");
    staking = await DevilStaking.deploy(await dvc.getAddress());

    const DevilDAO = await ethers.getContractFactory("DevilDAO");
    dao = await DevilDAO.deploy(await staking.getAddress());

    // Give alice and bob voting power via staking
    for (const user of [alice, bob]) {
      await dvc.transfer(user.address, ethers.parseEther("500"));
      await dvc.connect(user).approve(await staking.getAddress(), ethers.parseEther("500"));
      await staking.connect(user).stake(ethers.parseEther("100"));
    }
  });

  it("should create a proposal", async function () {
    await dao.connect(alice).createProposal("Upgrade v2", "Upgrade network to v2");
    const p = await dao.proposals(1);
    expect(p.title).to.equal("Upgrade v2");
    expect(p.proposer).to.equal(alice.address);
  });

  it("should allow voting", async function () {
    await dao.connect(alice).createProposal("Test", "Test proposal");
    await dao.connect(alice).vote(1, true);
    const p = await dao.proposals(1);
    expect(p.votesFor).to.be.gt(0);
  });

  it("should reject double voting", async function () {
    await dao.connect(alice).createProposal("Test", "Test");
    await dao.connect(alice).vote(1, true);
    await expect(dao.connect(alice).vote(1, true)).to.be.revertedWith("Already voted");
  });

  it("should reject voting without power", async function () {
    const [,,,noStake] = await ethers.getSigners();
    await dao.connect(alice).createProposal("Test", "Test");
    await expect(dao.connect(noStake).vote(1, true)).to.be.revertedWith("No voting power");
  });
});
