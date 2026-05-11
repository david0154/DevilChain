// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

/// @title DevilBridge — Cross-chain Bridge Contract
/// @notice Lock DVC on source chain, mint wrapped tokens on destination
contract DevilBridge is Ownable, ReentrancyGuard {
    IERC20 public devilCoin;

    enum BridgeStatus { Pending, Locked, Released, Cancelled }

    struct BridgeRequest {
        address from;
        string  toAddress;    // Destination chain address
        uint256 amount;
        string  destinationChain; // "ethereum", "bnb", "polygon", "solana"
        uint256 timestamp;
        BridgeStatus status;
        bytes32 txHash;
    }

    mapping(bytes32 => BridgeRequest) public bridgeRequests;
    mapping(bytes32 => bool) public processedRelays;  // prevent replay

    uint256 public bridgeFee = 0.001 ether;
    uint256 public minBridgeAmount = 1 ether;
    uint256 public totalBridged;
    address public relayer; // Trusted off-chain relayer

    event BridgeInitiated(bytes32 indexed reqId, address indexed from, string toAddress, uint256 amount, string destChain);
    event BridgeReleased(bytes32 indexed reqId, address indexed to, uint256 amount);
    event BridgeCancelled(bytes32 indexed reqId, address indexed from, uint256 amount);
    event RelayerUpdated(address indexed newRelayer);
    event FeeUpdated(uint256 newFee);

    modifier onlyRelayer() {
        require(msg.sender == relayer, "DevilBridge: Only relayer");
        _;
    }

    constructor(address _devilCoin, address _relayer) Ownable(msg.sender) {
        devilCoin = IERC20(_devilCoin);
        relayer = _relayer;
    }

    /// @notice Initiate bridge: lock DVC tokens
    function initiateBridge(
        string calldata toAddress,
        uint256 amount,
        string calldata destinationChain
    ) external payable nonReentrant returns (bytes32 reqId) {
        require(amount >= minBridgeAmount, "Amount below minimum");
        require(msg.value >= bridgeFee, "Insufficient bridge fee");
        require(bytes(toAddress).length > 0, "Invalid destination address");

        require(devilCoin.transferFrom(msg.sender, address(this), amount), "Transfer failed");

        reqId = keccak256(abi.encodePacked(msg.sender, toAddress, amount, block.timestamp, block.number));
        require(bridgeRequests[reqId].from == address(0), "Request ID collision");

        bridgeRequests[reqId] = BridgeRequest({
            from: msg.sender,
            toAddress: toAddress,
            amount: amount,
            destinationChain: destinationChain,
            timestamp: block.timestamp,
            status: BridgeStatus.Locked,
            txHash: reqId
        });

        totalBridged += amount;
        emit BridgeInitiated(reqId, msg.sender, toAddress, amount, destinationChain);
    }

    /// @notice Relayer releases tokens after cross-chain confirmation
    function releaseBridge(
        bytes32 reqId,
        address recipient,
        uint256 amount
    ) external onlyRelayer nonReentrant {
        require(!processedRelays[reqId], "Already processed");
        processedRelays[reqId] = true;
        require(devilCoin.transfer(recipient, amount), "Release transfer failed");
        emit BridgeReleased(reqId, recipient, amount);
    }

    /// @notice Cancel bridge and refund (within 24h)
    function cancelBridge(bytes32 reqId) external nonReentrant {
        BridgeRequest storage req = bridgeRequests[reqId];
        require(req.from == msg.sender, "Not bridge owner");
        require(req.status == BridgeStatus.Locked, "Cannot cancel");
        require(block.timestamp <= req.timestamp + 24 hours, "Cancel window expired");
        req.status = BridgeStatus.Cancelled;
        require(devilCoin.transfer(msg.sender, req.amount), "Refund failed");
        emit BridgeCancelled(reqId, msg.sender, req.amount);
    }

    function setRelayer(address _relayer) external onlyOwner { relayer = _relayer; emit RelayerUpdated(_relayer); }
    function setBridgeFee(uint256 fee) external onlyOwner { bridgeFee = fee; emit FeeUpdated(fee); }
    function setMinAmount(uint256 amount) external onlyOwner { minBridgeAmount = amount; }
    function withdrawFees() external onlyOwner { payable(owner()).transfer(address(this).balance); }
    function getBridgeRequest(bytes32 reqId) external view returns (BridgeRequest memory) { return bridgeRequests[reqId]; }
}
