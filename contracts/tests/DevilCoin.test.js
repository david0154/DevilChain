const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("DevilCoin (DVC)", function () {
  let coin, owner, alice, bob;

  beforeEach(async () => {
    [owner, alice, bob] = await ethers.getSigners();
    const Factory = await ethers.getContractFactory("DevilCoin");
    coin = await Factory.deploy();
    await coin.waitForDeployment();
  });

  it("Should have correct name, symbol, decimals", async () => {
    expect(await coin.name()).to.equal("DevilCoin");
    expect(await coin.symbol()).to.equal("DVC");
    expect(await coin.decimals()).to.equal(18);
  });

  it("Should assign total supply to owner", async () => {
    const supply = await coin.totalSupply();
    const ownerBal = await coin.balanceOf(owner.address);
    expect(ownerBal).to.equal(supply);
  });

  it("Should transfer tokens", async () => {
    const amount = ethers.parseEther("100");
    await coin.transfer(alice.address, amount);
    expect(await coin.balanceOf(alice.address)).to.equal(amount);
  });

  it("Should reject transfer exceeding balance", async () => {
    const huge = ethers.parseEther("999999999999");
    await expect(coin.connect(alice).transfer(bob.address, huge)).to.be.reverted;
  });

  it("Should allow approve and transferFrom", async () => {
    const amount = ethers.parseEther("50");
    await coin.approve(alice.address, amount);
    await coin.connect(alice).transferFrom(owner.address, bob.address, amount);
    expect(await coin.balanceOf(bob.address)).to.equal(amount);
  });
});
