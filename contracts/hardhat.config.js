require("@nomicfoundation/hardhat-toolbox");

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: "0.8.24",
  networks: {
    devilchain_testnet: {
      url: "http://localhost:8545",
      chainId: 6660,
      accounts: [process.env.PRIVATE_KEY || ""],
    },
  },
};
