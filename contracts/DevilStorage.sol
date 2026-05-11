// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title DevilStorage — Decentralized Storage Registry on DevilChain
/// @notice Store, retrieve and manage file metadata on-chain
contract DevilStorage {
    struct FileRecord {
        string  cid;          // IPFS/DevilStorage Content ID
        string  name;         // File name
        uint256 size;         // File size in bytes
        address owner;        // Uploader address
        uint256 timestamp;    // Upload time
        bool    isPublic;     // Public or private
        uint256 rewardPool;   // DVC rewards for hosting nodes
    }

    mapping(bytes32 => FileRecord) private files;   // fileHash => FileRecord
    mapping(address => bytes32[]) private userFiles; // owner => fileHashes
    mapping(bytes32 => address[]) private hosters;  // fileHash => hosting nodes

    uint256 public totalFiles;
    uint256 public constant MIN_STORAGE_REWARD = 0.001 ether;

    event FileStored(bytes32 indexed fileHash, address indexed owner, string cid, uint256 size);
    event FileDeleted(bytes32 indexed fileHash, address indexed owner);
    event NodeRewarded(bytes32 indexed fileHash, address indexed node, uint256 reward);
    event HosterAdded(bytes32 indexed fileHash, address indexed node);

    modifier onlyOwner(bytes32 fileHash) {
        require(files[fileHash].owner == msg.sender, "DevilStorage: Not file owner");
        _;
    }

    /// @notice Store file metadata and optionally fund a reward pool
    function storeFile(
        bytes32 fileHash,
        string calldata cid,
        string calldata name,
        uint256 size,
        bool isPublic
    ) external payable {
        require(files[fileHash].owner == address(0), "File already stored");
        require(size > 0, "Size must be > 0");
        require(bytes(cid).length > 0, "CID required");

        files[fileHash] = FileRecord({
            cid: cid,
            name: name,
            size: size,
            owner: msg.sender,
            timestamp: block.timestamp,
            isPublic: isPublic,
            rewardPool: msg.value
        });

        userFiles[msg.sender].push(fileHash);
        totalFiles++;

        emit FileStored(fileHash, msg.sender, cid, size);
    }

    /// @notice Get file metadata
    function getFile(bytes32 fileHash) external view returns (FileRecord memory) {
        require(files[fileHash].owner != address(0), "File not found");
        FileRecord memory f = files[fileHash];
        require(f.isPublic || f.owner == msg.sender, "Access denied");
        return f;
    }

    /// @notice Delete file (owner only)
    function deleteFile(bytes32 fileHash) external onlyOwner(fileHash) {
        delete files[fileHash];
        totalFiles--;
        emit FileDeleted(fileHash, msg.sender);
    }

    /// @notice Register as a hosting node for a file
    function registerHoster(bytes32 fileHash) external {
        require(files[fileHash].owner != address(0), "File not found");
        hosters[fileHash].push(msg.sender);
        emit HosterAdded(fileHash, msg.sender);
    }

    /// @notice Distribute rewards to hosting nodes
    function distributeRewards(bytes32 fileHash) external {
        FileRecord storage f = files[fileHash];
        require(f.owner == msg.sender, "Only owner can distribute rewards");
        address[] memory nodes = hosters[fileHash];
        require(nodes.length > 0, "No hosters registered");
        require(f.rewardPool > 0, "No reward pool");

        uint256 perNode = f.rewardPool / nodes.length;
        f.rewardPool = 0;
        for (uint i = 0; i < nodes.length; i++) {
            payable(nodes[i]).transfer(perNode);
            emit NodeRewarded(fileHash, nodes[i], perNode);
        }
    }

    /// @notice Get all files owned by an address
    function getUserFiles(address user) external view returns (bytes32[] memory) {
        return userFiles[user];
    }

    /// @notice Get all hosters of a file
    function getHosters(bytes32 fileHash) external view returns (address[] memory) {
        return hosters[fileHash];
    }

    /// @notice Top up reward pool for a file
    function topUpReward(bytes32 fileHash) external payable {
        require(files[fileHash].owner != address(0), "File not found");
        files[fileHash].rewardPool += msg.value;
    }
}
