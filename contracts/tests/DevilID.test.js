const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("DevilID", function () {
  let id, dao, owner, alice, bob;

  beforeEach(async () => {
    [owner, alice, bob] = await ethers.getSigners();
    dao = owner.address; // owner acts as DAO for tests
    const Factory = await ethers.getContractFactory("DevilID");
    id = await Factory.deploy(dao);
    await id.waitForDeployment();
  });

  it("Should create identity", async () => {
    await id.connect(alice).createIdentity("alice_dev", "Alice", "DevilChain builder", "ipfs://avatar");
    const identity = await id.getIdentity(alice.address);
    expect(identity.username).to.equal("alice_dev");
    expect(identity.isVerified).to.equal(false);
  });

  it("Should reject duplicate username", async () => {
    await id.connect(alice).createIdentity("alice_dev", "Alice", "Bio", "");
    await expect(id.connect(bob).createIdentity("alice_dev", "Bob", "Bio", "")).to.be.reverted;
  });

  it("Should resolve username to address", async () => {
    await id.connect(alice).createIdentity("alice_dev", "Alice", "Bio", "");
    const addr = await id.resolveUsername("alice_dev");
    expect(addr).to.equal(alice.address);
  });

  it("Should allow DAO to verify identity", async () => {
    await id.connect(alice).createIdentity("alice_dev", "Alice", "Bio", "");
    await id.connect(owner).setVerified(alice.address, true);
    const identity = await id.getIdentity(alice.address);
    expect(identity.isVerified).to.equal(true);
  });

  it("Should allow following", async () => {
    await id.connect(alice).createIdentity("alice_dev", "Alice", "Bio", "");
    await id.connect(bob).createIdentity("bob_dev", "Bob", "Bio", "");
    await id.connect(alice).follow(bob.address);
    expect(await id.getFollowerCount(bob.address)).to.equal(1n);
  });
});
