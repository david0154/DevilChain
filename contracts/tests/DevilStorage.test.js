const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("DevilStorage", function () {
  let storage, owner, alice, nodeA;

  beforeEach(async () => {
    [owner, alice, nodeA] = await ethers.getSigners();
    const Factory = await ethers.getContractFactory("DevilStorage");
    storage = await Factory.deploy();
    await storage.waitForDeployment();
  });

  it("Should store file metadata", async () => {
    const fileHash = ethers.keccak256(ethers.toUtf8Bytes("myfile.pdf"));
    await storage.connect(alice).storeFile(fileHash, "ipfs://Qm...", "myfile.pdf", 1024, true);
    const file = await storage.getFile(fileHash);
    expect(file.name).to.equal("myfile.pdf");
    expect(file.owner).to.equal(alice.address);
  });

  it("Should reject duplicate file store", async () => {
    const fileHash = ethers.keccak256(ethers.toUtf8Bytes("myfile.pdf"));
    await storage.connect(alice).storeFile(fileHash, "ipfs://Qm...", "myfile.pdf", 1024, true);
    await expect(storage.connect(alice).storeFile(fileHash, "ipfs://Qm2", "myfile.pdf", 1024, true)).to.be.reverted;
  });

  it("Should allow file deletion by owner", async () => {
    const fileHash = ethers.keccak256(ethers.toUtf8Bytes("del.pdf"));
    await storage.connect(alice).storeFile(fileHash, "ipfs://Qm...", "del.pdf", 512, true);
    await storage.connect(alice).deleteFile(fileHash);
    await expect(storage.getFile(fileHash)).to.be.reverted;
  });

  it("Should register hosters", async () => {
    const fileHash = ethers.keccak256(ethers.toUtf8Bytes("host.pdf"));
    await storage.connect(alice).storeFile(fileHash, "ipfs://Qm...", "host.pdf", 1024, true);
    await storage.connect(nodeA).registerHoster(fileHash);
    const hosters = await storage.getHosters(fileHash);
    expect(hosters.length).to.equal(1);
    expect(hosters[0]).to.equal(nodeA.address);
  });
});
