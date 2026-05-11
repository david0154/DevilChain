// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/**
 * @title DevilStorage
 * @notice Decentralized file storage registry on DevilChain
 * @dev Stores IPFS/DevilStorage CIDs on-chain with access control
 */
contract DevilStorage {
    struct File {
        string cid;           // Content ID (IPFS/DevilStorage)
        address owner;
        string fileName;
        uint256 fileSize;     // bytes
        uint256 uploadedAt;
        bool isPublic;
        bool exists;
    }

    mapping(bytes32 => File) private files;         // fileId => File
    mapping(address => bytes32[]) private userFiles; // owner => fileIds
    mapping(bytes32 => mapping(address => bool)) private accessList;

    uint256 public totalFiles;
    uint256 public constant MAX_FILE_SIZE = 1073741824; // 1GB in bytes

    event FileStored(bytes32 indexed fileId, address indexed owner, string cid, string fileName);
    event FileDeleted(bytes32 indexed fileId, address indexed owner);
    event AccessGranted(bytes32 indexed fileId, address indexed user);
    event AccessRevoked(bytes32 indexed fileId, address indexed user);

    modifier fileExists(bytes32 fileId) {
        require(files[fileId].exists, "File not found");
        _;
    }

    modifier onlyFileOwner(bytes32 fileId) {
        require(files[fileId].owner == msg.sender, "Not file owner");
        _;
    }

    function storeFile(
        string calldata cid,
        string calldata fileName,
        uint256 fileSize,
        bool isPublic
    ) external returns (bytes32 fileId) {
        require(fileSize <= MAX_FILE_SIZE, "File too large");
        require(bytes(cid).length > 0, "CID required");
        require(bytes(fileName).length > 0, "Filename required");

        fileId = keccak256(abi.encodePacked(msg.sender, cid, block.timestamp));
        require(!files[fileId].exists, "File already exists");

        files[fileId] = File({
            cid: cid,
            owner: msg.sender,
            fileName: fileName,
            fileSize: fileSize,
            uploadedAt: block.timestamp,
            isPublic: isPublic,
            exists: true
        });

        userFiles[msg.sender].push(fileId);
        totalFiles++;

        emit FileStored(fileId, msg.sender, cid, fileName);
    }

    function getFile(bytes32 fileId) external view fileExists(fileId) returns (File memory) {
        File memory f = files[fileId];
        require(
            f.isPublic || f.owner == msg.sender || accessList[fileId][msg.sender],
            "Access denied"
        );
        return f;
    }

    function grantAccess(bytes32 fileId, address user)
        external fileExists(fileId) onlyFileOwner(fileId) {
        accessList[fileId][user] = true;
        emit AccessGranted(fileId, user);
    }

    function revokeAccess(bytes32 fileId, address user)
        external fileExists(fileId) onlyFileOwner(fileId) {
        accessList[fileId][user] = false;
        emit AccessRevoked(fileId, user);
    }

    function deleteFile(bytes32 fileId)
        external fileExists(fileId) onlyFileOwner(fileId) {
        delete files[fileId];
        totalFiles--;
        emit FileDeleted(fileId, msg.sender);
    }

    function getUserFiles(address user) external view returns (bytes32[] memory) {
        return userFiles[user];
    }

    function hasAccess(bytes32 fileId, address user) external view returns (bool) {
        if (!files[fileId].exists) return false;
        return files[fileId].isPublic || files[fileId].owner == user || accessList[fileId][user];
    }
}
