const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("DevilBridge", function () {
  let coin, bridge, owner, relayer, alice;

  beforeEach(async () => {
    [owner, relayer, alice] = await ethers.getSigners();
    const Coin = await ethers.getContractFactory("DevilCoin");
    coin = await Coin.deploy();
    await coin.waitForDeployment();

    const Bridge = await ethers.getContractFactory("DevilBridge");
    bridge = await Bridge.deploy(await coin.getAddress(), relayer.address);
    await bridge.waitForDeployment();
  });

  it("Should initiate bridge and lock tokens", async () => {
    const amount = ethers.parseEther("100");
    await coin.transfer(alice.address, amount);
    await coin.connect(alice).approve(await bridge.getAddress(), amount);
    const fee = await bridge.bridgeFee();
    const tx = await bridge.connect(alice).initiateBridge(
      "0xEthAddress", amount, "ethereum", { value: fee }
    );
    const receipt = await tx.wait();
    const event = receipt.logs[0];
    expect(event).to.not.be.undefined;
    expect(await coin.balanceOf(await bridge.getAddress())).to.equal(amount);
  });

  it("Should release tokens via relayer", async () => {
    const amount = ethers.parseEther("100");
    await coin.transfer(alice.address, amount);
    await coin.connect(alice).approve(await bridge.getAddress(), amount);
    const fee = await bridge.bridgeFee();
    await bridge.connect(alice).initiateBridge("0xEthAddress", amount, "ethereum", { value: fee });
    // Fund bridge for release
    await coin.transfer(await bridge.getAddress(), amount);
    const fakeReqId = ethers.keccak256(ethers.toUtf8Bytes("test"));
    await bridge.connect(relayer).releaseBridge(fakeReqId, owner.address, amount);
    // No revert means success
  });

  it("Should reject bridge below minimum", async () => {
    const tiny = ethers.parseEther("0.0001");
    await coin.transfer(alice.address, tiny);
    await coin.connect(alice).approve(await bridge.getAddress(), tiny);
    const fee = await bridge.bridgeFee();
    await expect(bridge.connect(alice).initiateBridge("0xEth", tiny, "ethereum", { value: fee })).to.be.reverted;
  });
});
