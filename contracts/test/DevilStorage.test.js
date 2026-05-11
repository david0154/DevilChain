const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("DevilStorage", function () {
  let storage, owner, alice, bob;

  beforeEach(async function () {
    [owner, alice, bob] = await ethers.getSigners();
    const DevilStorage = await ethers.getContractFactory("DevilStorage");
    storage = await DevilStorage.deploy();
  });

  it("should store a file and return fileId", async function () {
    const tx = await storage.connect(alice).storeFile(
      "Qmtest123", "test.txt", 1024, true
    );
    const receipt = await tx.wait();
    const event = receipt.logs.find(l => l.fragment?.name === "FileStored");
    expect(event).to.exist;
  });

  it("should allow public file access", async function () {
    const tx = await storage.connect(alice).storeFile("Qmpublic", "public.txt", 512, true);
    const receipt = await tx.wait();
    const event = receipt.logs.find(l => l.fragment?.name === "FileStored");
    const fileId = event.args[0];
    const file = await storage.connect(bob).getFile(fileId);
    expect(file.fileName).to.equal("public.txt");
  });

  it("should block private file access without grant", async function () {
    const tx = await storage.connect(alice).storeFile("Qmprivate", "private.txt", 512, false);
    const receipt = await tx.wait();
    const event = receipt.logs.find(l => l.fragment?.name === "FileStored");
    const fileId = event.args[0];
    await expect(storage.connect(bob).getFile(fileId)).to.be.revertedWith("Access denied");
  });

  it("should grant and revoke access", async function () {
    const tx = await storage.connect(alice).storeFile("Qmgrant", "grant.txt", 128, false);
    const receipt = await tx.wait();
    const event = receipt.logs.find(l => l.fragment?.name === "FileStored");
    const fileId = event.args[0];
    await storage.connect(alice).grantAccess(fileId, bob.address);
    expect(await storage.hasAccess(fileId, bob.address)).to.be.true;
    await storage.connect(alice).revokeAccess(fileId, bob.address);
    expect(await storage.hasAccess(fileId, bob.address)).to.be.false;
  });
});
