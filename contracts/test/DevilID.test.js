const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("DevilID", function () {
  let dvc, staking, dao, identity, owner, alice, bob;

  beforeEach(async function () {
    [owner, alice, bob] = await ethers.getSigners();
    const DevilCoin = await ethers.getContractFactory("DevilCoin");
    dvc = await DevilCoin.deploy(ethers.parseEther("1000000000"));
    const DevilStaking = await ethers.getContractFactory("DevilStaking");
    staking = await DevilStaking.deploy(await dvc.getAddress());
    const DevilDAO = await ethers.getContractFactory("DevilDAO");
    dao = await DevilDAO.deploy(await staking.getAddress());
    const DevilID = await ethers.getContractFactory("DevilID");
    identity = await DevilID.deploy(await dao.getAddress());
  });

  it("should create a unique identity", async function () {
    await identity.connect(alice).createIdentity("alice_devil", "Alice", "Web3 dev", "Qmavatar");
    const id = await identity.getIdentity(alice.address);
    expect(id.username).to.equal("alice_devil");
    expect(id.active).to.be.true;
  });

  it("should reject duplicate usernames", async function () {
    await identity.connect(alice).createIdentity("devil_user", "Alice", "", "");
    await expect(
      identity.connect(bob).createIdentity("devil_user", "Bob", "", "")
    ).to.be.revertedWith("Username taken");
  });

  it("should reject duplicate identity creation", async function () {
    await identity.connect(alice).createIdentity("alice2", "Alice", "", "");
    await expect(
      identity.connect(alice).createIdentity("alice3", "Alice", "", "")
    ).to.be.revertedWith("Identity exists");
  });

  it("should resolve username to address", async function () {
    await identity.connect(alice).createIdentity("test_user", "Test", "", "");
    const addr = await identity.resolveUsername("test_user");
    expect(addr).to.equal(alice.address);
  });

  it("should issue and validate credentials", async function () {
    await identity.connect(alice).createIdentity("alice_cred", "Alice", "", "");
    const tx = await identity.connect(owner).issueCredential(
      alice.address, "developer", "hash123", 0
    );
    const receipt = await tx.wait();
    const event = receipt.logs.find(l => l.fragment?.name === "CredentialIssued");
    const credId = event.args[0];
    expect(await identity.isCredentialValid(credId)).to.be.true;
  });
});
