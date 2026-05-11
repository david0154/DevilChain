// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "./DevilCoin.sol";

/**
 * @title DevilBridge
 * @notice Cross-chain bridge contract for DevilChain
 * @dev Lock/Unlock pattern for native DVC bridging
 *      Supports: Ethereum, BNB Chain, Polygon, Solana (via relayer)
 */
contract DevilBridge {
    DevilCoin public dvc;
    address public owner;
    address public relayer;      // Trusted bridge relayer

    uint256 public bridgeFee = 0.001 ether;  // Fee in DVC (18 decimals)
    uint256 public minBridge = 1 ether;       // Min 1 DVC
    uint256 public maxBridge = 1_000_000 ether; // Max 1M DVC

    enum ChainID { DevilChain, Ethereum, BNBChain, Polygon, Solana }

    struct BridgeRequest {
        bytes32 id;
        address sender;
        string destAddress;    // Address on destination chain
        uint256 amount;
        ChainID destChain;
        uint256 timestamp;
        bool completed;
    }

    mapping(bytes32 => BridgeRequest) public bridgeRequests;
    mapping(bytes32 => bool) public processedInbound; // Prevent replay

    event BridgeOut(
        bytes32 indexed bridgeId,
        address indexed sender,
        string destAddress,
        uint256 amount,
        ChainID destChain
    );
    event BridgeIn(
        bytes32 indexed bridgeId,
        address indexed recipient,
        uint256 amount,
        ChainID srcChain
    );
    event FeeUpdated(uint256 newFee);

    modifier onlyOwner() { require(msg.sender == owner, "Not owner"); _; }
    modifier onlyRelayer() { require(msg.sender == relayer, "Not relayer"); _; }

    constructor(address _dvc, address _relayer) {
        dvc = DevilCoin(_dvc);
        owner = msg.sender;
        relayer = _relayer;
    }

    /**
     * @notice Lock DVC and emit bridge event to destination chain
     * @param destAddress Recipient address on destination chain
     * @param amount Amount of DVC to bridge (in wei units)
     * @param destChain Target chain ID
     */
    function bridgeOut(
        string calldata destAddress,
        uint256 amount,
        ChainID destChain
    ) external {
        require(amount >= minBridge, "Below minimum");
        require(amount <= maxBridge, "Exceeds maximum");
        require(bytes(destAddress).length > 0, "Invalid dest address");

        uint256 fee = bridgeFee;
        uint256 total = amount + fee;

        dvc.transferFrom(msg.sender, address(this), total);
        // Fee stays in bridge, amount is locked

        bytes32 bridgeId = keccak256(
            abi.encodePacked(msg.sender, destAddress, amount, block.timestamp, block.number)
        );

        bridgeRequests[bridgeId] = BridgeRequest({
            id: bridgeId,
            sender: msg.sender,
            destAddress: destAddress,
            amount: amount,
            destChain: destChain,
            timestamp: block.timestamp,
            completed: false
        });

        emit BridgeOut(bridgeId, msg.sender, destAddress, amount, destChain);
    }

    /**
     * @notice Relayer mints/unlocks DVC on DevilChain side after source chain confirmation
     * @param bridgeId Unique bridge request ID from source chain
     * @param recipient DevilChain recipient address
     * @param amount Amount to unlock
     * @param srcChain Source chain
     */
    function bridgeIn(
        bytes32 bridgeId,
        address recipient,
        uint256 amount,
        ChainID srcChain
    ) external onlyRelayer {
        require(!processedInbound[bridgeId], "Already processed");
        processedInbound[bridgeId] = true;

        dvc.transfer(recipient, amount);
        emit BridgeIn(bridgeId, recipient, amount, srcChain);
    }

    function setFee(uint256 newFee) external onlyOwner {
        bridgeFee = newFee;
        emit FeeUpdated(newFee);
    }

    function setRelayer(address newRelayer) external onlyOwner {
        relayer = newRelayer;
    }

    function withdrawFees() external onlyOwner {
        uint256 bal = dvc.balanceOf(address(this));
        dvc.transfer(owner, bal);
    }

    function getBridgeRequest(bytes32 bridgeId) external view returns (BridgeRequest memory) {
        return bridgeRequests[bridgeId];
    }
}
