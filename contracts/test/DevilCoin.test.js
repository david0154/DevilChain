const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("DevilCoin (DVC)", function () {
  let dvc, owner, alice, bob;
  const INITIAL_SUPPLY = ethers.parseEther("1000000000"); // 1B DVC

  beforeEach(async function () {
    [owner, alice, bob] = await ethers.getSigners();
    const DevilCoin = await ethers.getContractFactory("DevilCoin");
    dvc = await DevilCoin.deploy(ethers.parseEther("1000000000"));
  });

  it("should have correct name, symbol, decimals", async function () {
    expect(await dvc.name()).to.equal("DevilCoin");
    expect(await dvc.symbol()).to.equal("DVL");
    expect(await dvc.decimals()).to.equal(18);
  });

  it("should mint initial supply to owner", async function () {
    const bal = await dvc.balanceOf(owner.address);
    expect(bal).to.equal(INITIAL_SUPPLY);
  });

  it("should transfer tokens correctly", async function () {
    await dvc.transfer(alice.address, ethers.parseEther("100"));
    expect(await dvc.balanceOf(alice.address)).to.equal(ethers.parseEther("100"));
  });

  it("should fail transfer with insufficient balance", async function () {
    await expect(
      dvc.connect(alice).transfer(bob.address, ethers.parseEther("1"))
    ).to.be.revertedWith("Insufficient balance");
  });

  it("should approve and transferFrom correctly", async function () {
    await dvc.approve(alice.address, ethers.parseEther("50"));
    await dvc.connect(alice).transferFrom(owner.address, bob.address, ethers.parseEther("50"));
    expect(await dvc.balanceOf(bob.address)).to.equal(ethers.parseEther("50"));
  });

  it("should burn tokens correctly", async function () {
    const before = await dvc.totalSupply();
    await dvc.burn(ethers.parseEther("1000"));
    expect(await dvc.totalSupply()).to.equal(before - ethers.parseEther("1000"));
  });

  it("should allow owner to mint", async function () {
    await dvc.mint(alice.address, ethers.parseEther("500"));
    expect(await dvc.balanceOf(alice.address)).to.equal(ethers.parseEther("500"));
  });

  it("should reject non-owner mint", async function () {
    await expect(
      dvc.connect(alice).mint(bob.address, ethers.parseEther("100"))
    ).to.be.revertedWith("Not owner");
  });
});
